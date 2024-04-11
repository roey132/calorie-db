#![allow(dead_code)]
use crate::models::*;
use crate::*;
use diesel::result::Error;
use uuid::Uuid;

fn get_product_by_id(conn: &mut PgConnection, id: i32) -> Option<Product> {
    use self::schema::products::dsl::*;
    let mut results = products
        .filter(product_id.eq(id))
        .limit(1)
        .select(Product::as_select())
        .load(conn)
        .expect("Error loading product");

    if results.len() == 1 {
        Some(results.remove(0))
    } else {
        None
    }
}

fn get_products_by_user(conn: &mut PgConnection, id: Option<Uuid>) -> Vec<Product> {
    use self::schema::products::dsl::*;
    let mut query = products.into_boxed();
    if let Some(value) = id {
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

fn create_product_for_user(
    conn: &mut PgConnection,
    product_name: &str,
    calories_per_gram: f64,
    user_id: &Uuid,
) {
    use crate::schema::products;

    let new_product = NewUserProduct {
        product_name: product_name,
        calories_per_gram: calories_per_gram,
        user_id: user_id,
    };

    diesel::insert_into(products::table)
        .values(&new_product)
        .execute(conn)
        .expect("Failed to insert new product for user");
}

fn update_product_by_id(
    product_id: i32,
    product_name: &str,
    calories_per_gram: f64,
) -> Result<usize, Error> {
    use self::schema::products;

    let conn: &mut PgConnection = &mut establish_connection();
    diesel::update(products::table)
        .filter(products::product_id.eq(&product_id))
        .set((
            products::product_name.eq(product_name),
            products::calories_per_gram.eq(calories_per_gram),
        ))
        .execute(conn)
}
#[test]
fn test() {
    let connection = &mut establish_connection();

    if let Ok(user_uuid) = Uuid::parse_str(&mut "a0fc9dc5-4eb1-46ce-b473-416dfd243fa4") {
        create_product_for_user(connection, &"test_product", 5.0, &user_uuid)
    } else {
        println!("Failed to create uuid")
    }

    if let Ok(output) = update_product_by_id(2, "test_update", 50.0) {
        println!("Successfully updated product, {}", output)
    } else {
        println!("Failed to update product")
    }
}
