#![allow(dead_code)]
use crate::models::*;
use crate::users::get_user_uuid;
use crate::*;
use diesel::result::Error;
use uuid::Uuid;

pub fn get_product_by_id(conn: &mut PgConnection, id: i32) -> Result<Product, Error> {
    use self::schema::products::dsl::*;
    let mut results = products
        .filter(product_id.eq(id))
        .limit(1)
        .select(Product::as_select())
        .load(conn)?;

    if results.len() == 1 {
        Ok(results.remove(0))
    } else {
        Err(Error::NotFound)
    }
}

pub fn get_products_by_user(
    conn: &mut PgConnection,
    id: Option<Uuid>,
) -> Result<Vec<Product>, Error> {
    use self::schema::products::dsl::*;

    let final_id =
        id.unwrap_or(get_user_uuid(conn, &"system").expect("Failed to load system uuid"));

    products
        .filter(user_id.eq(final_id))
        .filter(delete_time.is_null())
        .select(Product::as_select())
        .load(conn)
}

pub fn create_product_for_user(
    conn: &mut PgConnection,
    product_name: &str,
    calories_per_gram: f64,
    user_id: &Uuid,
) -> Result<usize, Error> {
    use crate::schema::products;

    let new_product = NewUserProduct {
        product_name: product_name,
        calories_per_gram: calories_per_gram,
        user_id: user_id,
    };

    diesel::insert_into(products::table)
        .values(&new_product)
        .execute(conn)
}

pub fn create_system_product(
    conn: &mut PgConnection,
    product_name: &str,
    calories_per_gram: f64,
) -> Result<usize, Error> {
    use crate::schema::products;
    let sys_uuid = get_user_uuid(conn, &"system").expect("failed to get system uuid");
    let new_product = NewUserProduct {
        product_name: product_name,
        calories_per_gram: calories_per_gram,
        user_id: &sys_uuid,
    };

    diesel::insert_into(products::table)
        .values(&new_product)
        .execute(conn)
}

pub fn update_product_by_id(
    conn: &mut PgConnection,
    product_id: i32,
    product_name: &str,
    calories_per_gram: f64,
) -> Result<usize, Error> {
    use self::schema::products;
    diesel::update(products::table)
        .filter(products::product_id.eq(&product_id))
        .set((
            products::product_name.eq(product_name),
            products::calories_per_gram.eq(calories_per_gram),
        ))
        .execute(conn)
}

pub fn delete_product_by_id(conn: &mut PgConnection, id: i32) -> Result<usize, Error> {
    // soft deletes the products (sets delete time)
    use schema::products;
    diesel::update(products::table)
        .filter(products::product_id.eq(id))
        .set(products::delete_time.eq(diesel::dsl::now))
        .execute(conn)
}
