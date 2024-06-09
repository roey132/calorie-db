use self::models::*;
use crate::{schema::user_meals, *};
use chrono::NaiveDate;
use diesel::result::Error::{self, NotFound};
use uuid::Uuid;

pub fn create_user_meal_product(
    conn: &mut PgConnection,
    user_id: &Uuid,
    product_id: i32,
    product_grams: i32,
    meal_date: NaiveDate,
) -> Result<usize, Error> {
    let meal_type = MealEnum::Product;

    let new_meal = NewUserMealProduct {
        user_id,
        meal_type,
        product_id,
        product_grams,
        meal_date,
    };

    diesel::insert_into(user_meals::table)
        .values(new_meal)
        .execute(conn)
}

pub fn create_user_meal_measure(
    conn: &mut PgConnection,
    user_id: &Uuid,
    measure_id: i32,
    measure_count: f64,
    meal_date: NaiveDate,
) -> Result<usize, Error> {
    let meal_type = MealEnum::Measure;

    let new_meal = NewUserMealMeasure {
        user_id,
        meal_type,
        measure_id,
        measure_count,
        meal_date,
    };

    diesel::insert_into(user_meals::table)
        .values(new_meal)
        .execute(conn)
}
pub fn create_user_meal_calories(
    conn: &mut PgConnection,
    user_id: &Uuid,
    meal_name: Option<&str>,
    calories: f64,
    meal_date: NaiveDate,
) -> Result<usize, Error> {
    let meal_type = MealEnum::Calories;

    let new_meal = NewUserMealCalories {
        user_id,
        meal_type,
        calories,
        meal_name,
        meal_date,
    };

    diesel::insert_into(user_meals::table)
        .values(new_meal)
        .execute(conn)
}

pub fn get_user_meal_by_id(conn: &mut PgConnection, meal_id: i32) -> Result<UserMeal, Error> {
    let mut results: Vec<UserMeal> = user_meals::table
        .filter(user_meals::meal_id.eq(meal_id))
        .select(user_meals::table::all_columns())
        .load(conn)?;
    if results.len() == 1 {
        Ok(results.remove(0))
    } else {
        Err(NotFound)
    }
}

pub fn update_user_meal_product_by_meal_id(
    conn: &mut PgConnection,
    meal_id: i32,
    product_grams: i32,
) -> Result<usize, Error> {
    diesel::update(user_meals::table)
        .filter(user_meals::meal_id.eq(meal_id))
        .set(user_meals::product_grams.eq(product_grams))
        .execute(conn)
}

pub fn update_user_meal_measure_by_meal_id(
    conn: &mut PgConnection,
    meal_id: i32,
    measure_count: f64,
) -> Result<usize, Error> {
    diesel::update(user_meals::table)
        .filter(user_meals::meal_id.eq(meal_id))
        .set(user_meals::measure_count.eq(measure_count))
        .execute(conn)
}

pub fn update_user_meal_calories_by_meal_id(
    conn: &mut PgConnection,
    meal_id: i32,
    calories: f64,
) -> Result<usize, Error> {
    diesel::update(user_meals::table)
        .filter(user_meals::meal_id.eq(meal_id))
        .set(user_meals::calories.eq(calories))
        .execute(conn)
}

pub fn delete_user_meal_by_id(conn: &mut PgConnection, meal_id: i32) -> Result<usize, Error> {
    diesel::delete(user_meals::table)
        .filter(user_meals::meal_id.eq(meal_id))
        .execute(conn)
}
