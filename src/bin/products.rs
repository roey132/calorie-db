#![allow(dead_code)]
use self::models::*;
use diesel::prelude::*;
use calorie_db::*;
use uuid::Uuid;

fn get_product_by_id(conn:&mut PgConnection,id:&i32) -> Option<Product> {
    use self::schema::products::dsl::*;
    let mut results = products
        .filter(product_id.eq(id))
        .limit(1)
        .select(Product::as_select())
        .load(conn)
        .expect("Error loading product");

        if results.len() == 1{
        Some(results.remove(0))
    } else {
        None
    }
}

fn get_products_by_user(conn:&mut PgConnection, id:Option<Uuid>) -> Vec<Product> {
    use self::schema::products::dsl::*;
    let mut query = products.into_boxed();
    if let Some(value) = id{
        query = query.filter(user_id.eq(value))
    } else {
        query = query.filter(user_id.is_null())
    }
    let results = query
        .select(Product::as_select())
        .load(conn)
        .expect("Error loading products");

    results
}

fn create_product_for_user(conn:&mut PgConnection
    , product_name:&str
    , calories_1gram:&i32
    , user_id:&Uuid ) {
    
    use crate::schema::products;

    let new_product = NewUserProduct{product_name:product_name
        ,calories_1gram:calories_1gram
        ,user_id:user_id};
        
        diesel::insert_into(products::table)
            .values(&new_product)
            .execute(conn)
            .expect("Failed to insert new product for user");

    }

fn main(){

    let connection = &mut establish_connection();

    if let Ok(user_uuid) = Uuid::parse_str(&mut "ebc8a710-a25f-4090-a45f-1e37cb0c7446"){
        create_product_for_user(connection,&"test_product", &5, &user_uuid)
    } else {
        println!("Failed to create uuid")
    }
}