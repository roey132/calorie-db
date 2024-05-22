#![allow(dead_code)]
use self::models::*;
use crate::{schema::users, *};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use diesel::result::Error;
use uuid::Uuid;

pub fn hash_password(password: String) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)?
        .to_string();
    Ok(password_hash)
}

pub fn check_user_password(
    conn: &mut PgConnection,
    username: String,
    password: &String,
) -> Result<Uuid, ServerError> {
    let user = get_user_uuid(conn, &username)?;
    let parsed_hash = PasswordHash::new(&user.password)
        .map_err(|_| ServerError::CustomError("Failed to parse password".to_string()))?;
    Argon2::default()
        .verify_password(&password.as_bytes(), &parsed_hash)
        .map_err(|_| ServerError::Unauthorized)?;
    Ok(user.user_id)
}

pub fn create_user(
    conn: &mut PgConnection,
    username: String,
    password: String,
) -> Result<usize, Error> {
    let new_user = NewUser {
        username: username,
        password: password,
    };
    diesel::insert_into(users::table)
        .values(new_user)
        .execute(conn)
}

pub fn get_user_uuid(
    conn: &mut PgConnection,
    _username: &str,
) -> Result<models::User, ServerError> {
    use self::schema::users::dsl::*;

    let mut results = users
        .filter(username.eq(_username))
        .select(users::all_columns())
        .limit(1)
        .load(conn)?;

    if results.len() == 1 {
        Ok(results.remove(0))
    } else {
        Err(ServerError::CustomError("User does not exist".to_string()))
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
