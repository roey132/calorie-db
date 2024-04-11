#![allow(dead_code)]

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
mod models;
mod product_measures;
mod products;
mod schema;
mod user_meals;
mod users;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let databasse_url = env::var("DATABASE_URL").expect("DATABSE_URL must be set");
    PgConnection::establish(&databasse_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", databasse_url))
}

fn main() {}
