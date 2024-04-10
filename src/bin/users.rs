#![allow(dead_code)]
use uuid::Uuid;

use calorie_db::{schema::users, *};
use self::models::*;
use diesel::prelude::*;

fn create_user(username: &String, password:&String ) {
    let conn: &mut PgConnection = &mut establish_connection();
    let new_user = NewUser{
        username:username,
        password:password
    };
    
    diesel::insert_into(users::table)
        .values(new_user)
        .execute(conn)
        .expect("Failed to insert new user");
}
fn get_user_uuid(user_name: &String) -> Result<Uuid,String>{
    let conn: &mut PgConnection = &mut establish_connection();
    use self::schema::users::dsl::*;

    let mut results = users
        .filter(username.eq(user_name))
        .select(user_id)
        .limit(1)
        .load(conn)
        .expect("Error loading user id");

    if results.len() == 1{
        Ok(results.remove(0))
    } else {
        Err("Failed to find user id".to_string())
    }
}
fn main(){
    //create_user(&"username".to_string(), &"password".to_string());
    let user_id = get_user_uuid(&"username".to_string());
    match user_id{
        Ok(value) => println!("found user uuid:{}",value),
        Err(msg) => println!("{}",msg)
    }
    
}
