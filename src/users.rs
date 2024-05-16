#![allow(dead_code)]
use self::models::*;
use crate::{schema::users, *};
use diesel::result::Error;
use uuid::Uuid;

pub fn create_user(
    conn: &mut PgConnection,
    username: &str,
    password: &str,
) -> Result<usize, Error> {
    let new_user = NewUser {
        username: username,
        password: password,
    };

    diesel::insert_into(users::table)
        .values(new_user)
        .execute(conn)
}

pub fn get_user_uuid(conn: &mut PgConnection, user_name: &str) -> Result<Uuid, &'static str> {
    use self::schema::users::dsl::*;

    let mut results = users
        .filter(username.eq(user_name))
        .select(user_id)
        .limit(1)
        .load(conn)
        .expect("Error loading user id");

    if results.len() == 1 {
        Ok(results.remove(0))
    } else {
        Err("Failed to find user id")
    }
}

pub fn get_user_by_id(conn: &mut PgConnection, user_id: Uuid) -> Result<User, Error> {
    let mut results: Vec<User> = users::table
        .filter(users::user_id.eq(user_id))
        .limit(1)
        .select(schema::users::all_columns)
        .load(conn)?;

    if results.len() == 1 {
        Ok(results.remove(0))
    } else {
        Err(diesel::result::Error::NotFound)
    }
}
