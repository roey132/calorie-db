#![allow(dead_code)]

use crate::models::*;
use crate::*;
use diesel::result::Error::{self};

fn create_product_measure(
    product_id: i32,
    measure_name: &str,
    measure_calories: f64,
    default_measure: bool,
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

fn get_product_measures_by_product(product_id: i32) -> Result<Vec<ProductMeasure>, Error> {
    use self::schema::product_measures;
    let conn = &mut establish_connection();
    let results = product_measures::table
        .filter(product_measures::product_id.eq(product_id))
        .select(product_measures::table::all_columns())
        .load(conn);

    results
}

fn get_product_measure_by_measure_id(measure_id: i32) -> Result<Option<ProductMeasure>, Error> {
    use self::schema::product_measures;

    let conn = &mut establish_connection();

    let mut results = product_measures::table
        .filter(product_measures::measure_id.eq(measure_id))
        .limit(1)
        .select(product_measures::table::all_columns())
        .load(conn)?;

    if results.len() == 1 {
        Ok(Some(results.remove(0)))
    } else {
        Ok(None)
    }
}

#[test]
fn test() {
    create_product_measure(3, "spoon", 55.0, false).unwrap();

    get_product_measures_by_product(1).unwrap();

    get_product_measures_by_product(3).unwrap();

    get_product_measure_by_measure_id(1).unwrap().unwrap();
}
