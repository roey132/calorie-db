#![allow(dead_code)]
use self::models::*;
use crate::{schema::users, *};
use uuid::Uuid;

fn create_user(conn: &mut PgConnection, username: &str, password: &str) {
    let new_user = NewUser {
        username: username,
        password: password,
    };

    diesel::insert_into(users::table)
        .values(new_user)
        .execute(conn)
        .expect("Failed to insert new user");
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
fn main() {
    let conn = &mut establish_connection();
    create_user(conn, &"username".to_string(), &"password".to_string());
    let user_id = get_user_uuid(conn, &"username".to_string());
    match user_id {
        Ok(value) => println!("found user uuid:{}", value),
        Err(msg) => println!("{}", msg),
    }
}
