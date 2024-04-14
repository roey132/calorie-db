#![allow(dead_code)]

use crate::models::*;
use crate::*;
use diesel::result::Error::{self};

pub fn create_product_measure(
    conn: &mut PgConnection,
    product_id: i32,
    measure_name: &str,
    measure_calories: f64,
    default_measure: bool,
) -> Result<usize, Error> {
    use self::schema::product_measures;
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

pub fn get_product_measures_by_product(
    conn: &mut PgConnection,
    product_id: i32,
) -> Result<Vec<ProductMeasure>, Error> {
    use self::schema::product_measures;
    let results = product_measures::table
        .filter(product_measures::product_id.eq(product_id))
        .select(product_measures::table::all_columns())
        .load(conn);

    results
}

pub fn get_product_measure_by_measure_id(
    conn: &mut PgConnection,
    measure_id: i32,
) -> Result<ProductMeasure, Error> {
    use self::schema::product_measures;
    let mut results = product_measures::table
        .filter(product_measures::measure_id.eq(measure_id))
        .limit(1)
        .select(product_measures::table::all_columns())
        .load(conn)?;

    if results.len() == 1 {
        Ok(results.remove(0))
    } else {
        Err(Error::NotFound)
    }
}

#[test]
fn test() {
    let conn = &mut establish_connection();
    create_product_measure(conn, 3, "spoon", 55.0, false).unwrap();

    get_product_measures_by_product(conn, 1).unwrap();

    get_product_measures_by_product(conn, 3).unwrap();

    get_product_measure_by_measure_id(conn, 1).unwrap();
}
