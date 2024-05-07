#![allow(dead_code)]
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use diesel::r2d2::{self, ConnectionManager};

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

#[get("/products/{id}/name")]
async fn get_product_name(
    pool: web::Data<DbPool>,
    path: web::Path<ProductIdPath>,
) -> impl Responder {
    let product_id = path.id;

    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().body("Database connection failed."),
    };

    let product = products::get_product_by_id(&mut conn, product_id).unwrap();

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
    dotenv::dotenv().ok();
    let database_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL must be set in env variables");

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create R2D2 pool");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(web::scope("/api").service(get_product_name))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
