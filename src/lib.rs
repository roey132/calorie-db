use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub mod models;
pub mod schema;

pub fn establish_connection() -> PgConnection{
    dotenv().ok();

    let databasse_url = env::var("DATABASE_URL").expect("DATABSE_URL must be set");
    PgConnection::establish(&databasse_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", databasse_url))
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
