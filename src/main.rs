#![allow(dead_code)]
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use uuid::Uuid;
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

async fn test_path() -> impl Responder {
    HttpResponse::Ok().body("Test Path Is Working")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}
#[derive(Debug, serde::Deserialize)]
struct ProductIdPath {
    id: i32,
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
#[derive(Debug, serde::Deserialize)]
struct UserIdPath {
    user_id: Uuid,
}
#[get("/users/uuid/{user_id}/products")]
async fn get_products_for_user_id(
    pool: web::Data<DbPool>,
    path: web::Path<UserIdPath>,
) -> impl Responder {
    let user_id = path.user_id;
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().body("Database connection failed."),
    };

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
                .service(get_all_non_user_products),
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
