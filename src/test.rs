#![allow(unused_imports)]
use chrono::NaiveDate;
use rand::{distributions::Alphanumeric, Rng};

use crate::{establish_connection, product_measures::*, products::*, user_meals::*, users::*};

#[test]
fn tests() {
    // create connections and rand user name
    let conn = &mut establish_connection();
    let random_string: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(5)
        .map(char::from)
        .collect();

    // test user creation
    create_user(conn, &random_string, "password").unwrap();

    // get user uuid (generated by db)
    let user_id = get_user_uuid(conn, &random_string).unwrap();

    // test product creation for user
    create_product_for_user(conn, "potato", 5.0, &user_id).unwrap();
    create_product_for_user(conn, "cucumber", 0.0, &user_id).unwrap();

    // get vector of products for that use (products that just got created)
    let mut user_products = get_products_by_user(conn, Some(user_id)).unwrap();
    let first_product = user_products.remove(0);

    // set product id to check on updates and get functions
    let product_id = first_product.product_id;

    get_product_by_id(conn, product_id).unwrap();
    update_product_by_id(conn, product_id, "egg", 100.0).unwrap();

    // test measurement functions
    create_product_measure(conn, product_id, "test_measure", 300.0, false).unwrap();
    create_product_measure(conn, product_id, "test_measure_2", 200.0, false).unwrap();

    get_product_measure_by_measure_id(conn, 1).unwrap();
    get_product_measures_by_product(conn, product_id).unwrap();

    let date = NaiveDate::from_ymd_opt(2024, 12, 12).unwrap();
    create_user_meal_measure(
        conn,
        &user_id,
        product_id,
        1,
        2.5,
        Some("test_name".to_string()),
        Some("test_note".to_string()),
        date,
    )
    .unwrap();
    create_user_meal_product(
        conn,
        &user_id,
        product_id,
        100,
        Some("Test Meal Name".to_string()),
        None,
        date,
    )
    .unwrap();
}