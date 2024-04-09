#![allow(dead_code)]

use calorie_db::{schema::users, *};
use self::models::*;
use diesel::prelude::*;

fn create_user(username: &String, password:&String ){
    let conn: &mut PgConnection = &mut establish_connection();
    let new_user = NewUser{
        username:username,
        password:password
    };
    {
    let _temp = diesel::insert_into(users::table)
        .values(new_user)
        .execute(conn)
        .expect("Failed to insert new user");
    }
}

fn main(){
    create_user(&"username".to_string(), &"password".to_string())
}
