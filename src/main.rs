#![allow(dead_code)]
use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result};
use std::{future::Future, pin::Pin};

use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use uuid;
use uuid::Uuid;
mod middleware;
mod models;
mod product_measures;
mod products;
mod schema;
mod test;
mod user_meals;
mod user_meals_calculated;
mod users;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}
#[derive(Debug, serde::Deserialize)]
struct ProductIdPath {
    id: i32,
}

#[derive(Deserialize)]
struct UserProductEdit {
    user_id: String,
    product_name: String,
    product_id: i32,
    calories_per_100g: f64,
}
#[post("/products/user_product/edit")]
async fn edit_user_product(
    pool: web::Data<DbPool>,
    info: web::Json<UserProductEdit>,
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
    user_id: String,
    product_name: String,
    calories_per_100g: f64,
}
#[post("/products/user_product/create")]
async fn create_user_product(
    pool: web::Data<DbPool>,
    info: web::Json<UserProductInfo>,
) -> impl Responder {
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().body("Database connection failed."),
    };

    let user_id = match Uuid::parse_str(&info.user_id) {
        Ok(result) => result,
        Err(e) => {
            return HttpResponse::InternalServerError().body(format!(
                "Failed to format user id to UUID due to error: {e}"
            ))
        }
    };

    let calories_per_gram = info.calories_per_100g / 100.0;

    let result = products::create_product_for_user(
        &mut conn,
        &info.product_name,
        calories_per_gram,
        &user_id,
    );
    match result {
        Ok(_) => HttpResponse::Ok().body("Successfully created new product"),
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("Failed to create new product: {}", e))
        }
    }
}
#[get("/products/id/{id}")]
async fn get_product(pool: web::Data<DbPool>, path: web::Path<ProductIdPath>) -> impl Responder {
    let product_id = path.id;

    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().body("Database connection failed."),
    };

    #[derive(Serialize, Deserialize)]
    struct ProductResponse {
        product: models::Product,
    }
    let result = products::get_product_by_id(&mut conn, product_id);
    match result {
        Ok(product_result) => {
            let mut product_map: HashMap<String, models::Product> = HashMap::new();
            product_map.insert("product".to_string(), product_result);

            HttpResponse::Ok().json(product_map)
        }
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("Request failed due to error: {}", e))
        }
    }
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
async fn get_all_non_user_products(pool: web::Data<DbPool>) -> impl Responder {
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

impl actix_web::FromRequest for models::User {
    type Error = ServerError;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        println!("Request: {}", req.path());
        let req = req.clone();
        Box::pin(async move {
            let mut conn = req.app_data::<web::Data<DbPool>>().unwrap().get()?;

            let headers = req.headers();
            let id = headers
                .get("user_id")
                .ok_or(ServerError::Unauthorized)?
                .to_str()
                .map_err(|_| ServerError::Unauthorized)?;
            let user_id = uuid::Uuid::parse_str(id)?;
            let user = users::get_user_by_id(&mut conn, user_id)?;
            Ok(user)
        })
    }
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
