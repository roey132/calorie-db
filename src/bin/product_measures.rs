#![allow(dead_code)]

use std::error;

use self::models::*;
use calorie_db::*;
use diesel::{
    prelude::*,
    result::Error::{self, NotFound},
};

fn create_product_measure(
    product_id: &i32,
    measure_name: &str,
    measure_calories: &f64,
    default_measure: &bool,
) -> Result<usize, Error> {
    use self::schema::product_measures;

    let conn = &mut establish_connection();

    let new_product_measure = NewProductMeasure {
        product_id: product_id,
        measure_name: measure_name,
        measure_calories: measure_calories,
        default_measure: default_measure,
    };

    diesel::insert_into(product_measures::table)
        .values(new_product_measure)
        .execute(conn)
}

fn get_product_measures_by_product(product_id: &i32) -> Result<Vec<ProductMeasure>, Error> {
    use self::schema::product_measures;
    let conn = &mut establish_connection();
    let results = product_measures::table
        .filter(product_measures::product_id.eq(product_id))
        .select(product_measures::table::all_columns())
        .load(conn);

    results
}

fn get_product_measure_by_measure_id(measure_id: &i32) -> Result<ProductMeasure, Error> {
    use self::schema::product_measures;

    let conn = &mut establish_connection();

    let mut results = product_measures::table
        .filter(product_measures::measure_id.eq(measure_id))
        .limit(1)
        .select(product_measures::table::all_columns())
        .load(conn);

    match results {
        Ok(mut values) => {
            if values.len() == 1 {
                Ok(values.remove(0))
            } else {
                Err(NotFound)
            }
        }
        Err(error) => Err(error),
    }
}
fn main() {
    match create_product_measure(&3, &"spoon", &55.0, &false) {
        Ok(_) => println!("Creation Success"),
        Err(error) => println!("Creation Failed, {error}"),
    }

    match get_product_measures_by_product(&1) {
        Ok(_) => println!("get product_id success for product id 1"),
        Err(error) => println!("{error}"),
    }

    match get_product_measures_by_product(&3) {
        Ok(_) => println!("get product_id success for product id 3"),
        Err(error) => println!("{error}"),
    }

    match get_product_measure_by_measure_id(&1) {
        Ok(product) => println!("measure name: {} for measure id 1", product.measure_name),
        Err(error) => println!("{error}"),
    }
}
