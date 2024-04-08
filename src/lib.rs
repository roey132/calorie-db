use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use self::models::{NewProduct,Product};
use std::env;
use chrono::{DateTime,NaiveDateTime,Utc};
pub mod models;
pub mod schema;

pub fn establish_connection() -> PgConnection{
    dotenv().ok();

    let databasse_url = env::var("DATABASE_URL").expect("DATABSE_URL must be set");
    PgConnection::establish(&databasse_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", databasse_url))
}

pub fn create_product(conn: &mut PgConnection,
    product_name: &str, 
    calories_1gram: &i32) -> Product{

    use crate::schema::products;
    let now_utc: DateTime<Utc> = Utc::now();
    let create_time: NaiveDateTime = now_utc.naive_utc();
    
    let new_product = NewProduct{product_name,calories_1gram,create_time:&create_time};

    diesel::insert_into(products::table)
        .values(&new_product)
        .returning(Product::as_returning())
        .get_result(conn)
        .expect("Error saving new product")
}