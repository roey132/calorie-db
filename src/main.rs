#![allow(dead_code)]
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
mod models;
mod product_measures;
mod products;
mod schema;
mod test;
mod user_meals;
mod user_meals_calculated;
mod users;

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
async fn get_product_name(path: web::Path<ProductIdPath>) -> impl Responder {
    // TODO: change conn to get from connection pool (r2d2)
    let product_id = path.id;
    let conn = &mut establish_connection();
    let product = products::get_product_by_id(conn, product_id).unwrap();

    HttpResponse::Ok().body(product.product_name)
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABSE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
            .route("/test", web::get().to(test_path))
            .route("/products/{id}/name", web::get().to(get_product_name))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
