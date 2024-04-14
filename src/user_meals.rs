use self::models::*;
use crate::{schema::user_meals, *};
use chrono::NaiveDate;
use diesel::result::Error;
use uuid::Uuid;

pub fn create_user_meal_product(
    conn: &mut PgConnection,
    user_id: &Uuid,
    product_id: i32,
    product_grams: i32,
    meal_name: Option<String>,
    meal_note: Option<String>,
    meal_date: NaiveDate,
) -> Result<usize, Error> {
    let product = products::get_product_by_id(conn, product_id)?;

    let product_cal_per_gram = product.calories_per_gram;
    let calories = product_cal_per_gram * product_grams as f64;

    let new_meal = NewUserMealProduct {
        user_id,
        product_id,
        product_grams,
        calories,
        meal_name,
        meal_note,
        meal_date,
    };

    diesel::insert_into(user_meals::table)
        .values(new_meal)
        .execute(conn)
}

pub fn create_user_meal_measure(
    conn: &mut PgConnection,
    user_id: &Uuid,
    product_id: i32,
    measure_id: i32,
    measure_count: f64,
    meal_name: Option<String>,
    meal_note: Option<String>,
    meal_date: NaiveDate,
) -> Result<usize, Error> {
    let measure = product_measures::get_product_measure_by_measure_id(conn, measure_id)?;
    let measure_calories = measure.measure_calories;
    let calories = measure_calories * measure_count;
    let new_meal = NewUserMealMeasure {
        user_id,
        product_id,
        measure_id,
        measure_count,
        calories,
        meal_name,
        meal_note,
        meal_date,
    };

    diesel::insert_into(user_meals::table)
        .values(new_meal)
        .execute(conn)
}
