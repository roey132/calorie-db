#![allow(dead_code)]
use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use dotenv::dotenv;
use serde::Deserialize;
use std::collections::HashMap;
use std::env;
use uuid;
mod middleware;
mod models;
mod product_measures;
mod products;
mod schema;
mod test;
mod user_extractor;
mod user_meals;
mod user_meals_calculated;
mod users;
type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Debug, thiserror::Error)]
pub enum ServerError {
    #[error("Database Error: {0}")]
    DbError(#[from] diesel::result::Error),
    #[error("R2D2 Connection Error")]
    ConnectionError(#[from] r2d2::Error),
    #[error("Parse Error {0}")]
    ParseError(#[from] uuid::Error),
    #[error("User Unauthorized")]
    Unauthorized,
}

impl actix_web::error::ResponseError for ServerError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            ServerError::DbError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ServerError::ParseError(_) => StatusCode::BAD_REQUEST,
            ServerError::Unauthorized => StatusCode::UNAUTHORIZED,
            ServerError::ConnectionError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[derive(Deserialize)]
struct UserProductEdit {
    product_name: String,
    product_id: i32,
    calories_per_100g: f64,
}
#[post("/products/user_product/edit")]
async fn edit_user_product(
    pool: web::Data<DbPool>,
    info: web::Json<UserProductEdit>,
    _: models::User,
) -> impl Responder {
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().body("Database connection failed."),
    };

    let calories_per_gram = info.calories_per_100g / 100.0;

    match products::update_product_by_id(
        &mut conn,
        info.product_id,
        &info.product_name,
        calories_per_gram,
    ) {
        Ok(_) => HttpResponse::Ok().body("Successfully updated product"),
        Err(e) => HttpResponse::InternalServerError()
            .body(format!("Failed to edit product due to error: {e}")),
    }
}

#[derive(Deserialize)]
struct UserProductInfo {
    product_name: String,
    calories_per_100g: f64,
}
#[post("/products/user_product/create")]
async fn create_user_product(
    pool: web::Data<DbPool>,
    info: web::Json<UserProductInfo>,
    user: models::User,
) -> impl Responder {
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().body("Database connection failed."),
    };

    let calories_per_gram = info.calories_per_100g / 100.0;

    let result = products::create_product_for_user(
        &mut conn,
        &info.product_name,
        calories_per_gram,
        &user.user_id,
    );
    match result {
        Ok(_) => HttpResponse::Ok().body("Successfully created new product"),
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("Failed to create new product: {}", e))
        }
    }
}

#[get("/products/product/get/{id}")]
async fn get_product(
    pool: web::Data<DbPool>,
    path: web::Path<(i32,)>,
    _: models::User,
) -> Result<web::Json<HashMap<String, models::Product>>, ServerError> {
    let product_id = path.0;
    let mut conn = pool.get()?;

    let result = products::get_product_by_id(&mut conn, product_id)?;

    let mut product_map: HashMap<String, models::Product> = HashMap::new();
    product_map.insert("product".to_string(), result);

    Ok(web::Json(product_map))
}

#[get("/products/user")]
async fn get_products_for_user_id(pool: web::Data<DbPool>, user: models::User) -> impl Responder {
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().body("Database connection failed."),
    };

    let user_id = user.user_id;
    let results = products::get_products_by_user(&mut conn, Some(user_id));
    let products_map = match results {
        Ok(products) => {
            let mut map: HashMap<i32, models::Product> = HashMap::new();
            for product in products {
                map.insert(product.product_id, product);
            }
            map
        }
        Err(_) => return HttpResponse::InternalServerError().body("error"),
    };

    HttpResponse::Ok().json(products_map)
}

#[get("/products/system")]
async fn get_all_non_user_products(pool: web::Data<DbPool>, _: models::User) -> impl Responder {
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().body("Database connection failed."),
    };

    let results = products::get_products_by_user(&mut conn, None);
    let products_map = match results {
        Ok(products) => {
            let mut map: HashMap<i32, models::Product> = HashMap::new();
            for product in products {
                map.insert(product.product_id, product);
            }
            map
        }
        Err(_) => return HttpResponse::InternalServerError().body("error"),
    };

    HttpResponse::Ok().json(products_map)
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABSE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let database_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL must be set in env variables");

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create R2D2 pool");

    HttpServer::new(move || {
        App::new().app_data(web::Data::new(pool.clone())).service(
            web::scope("/api")
                .service(get_product)
                .service(get_products_for_user_id)
                .service(get_all_non_user_products)
                .service(create_user_product)
                .service(edit_user_product),
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
