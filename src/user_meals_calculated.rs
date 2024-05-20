use std::collections::HashMap;

use chrono::NaiveDate;
use diesel::query_dsl::methods::{FilterDsl, SelectDsl};
use diesel::result::Error;
use diesel::RunQueryDsl;
use diesel::Table;
use diesel::{ExpressionMethods, PgConnection};
use uuid::Uuid;

use crate::models::UserMealCalculated;

diesel::table! {
    use diesel::sql_types::*;
    use crate::schema::sql_types::MealType;
    user_meals_calculated (meal_id) {

        meal_id -> Integer,
        user_id -> diesel::sql_types::Uuid,
        meal_type -> MealType,
        product_name -> Nullable<Varchar>,
        product_grams -> Nullable<Integer>,
        measure_name -> Nullable<Varchar>,
        measure_count -> Nullable<Double>,
        meal_date -> Date,
        calc_calories -> Double,
    }
}

pub fn get_user_meal_calories(
    conn: &mut PgConnection,
    meal_id: i32,
) -> Result<UserMealCalculated, Error> {
    let mut results = user_meals_calculated::table
        .filter(user_meals_calculated::meal_id.eq(meal_id))
        .select(user_meals_calculated::table::all_columns())
        .load(conn)?;

    if results.len() == 1 {
        Ok(results.remove(0))
    } else {
        Err(Error::NotFound)
    }
}

pub fn get_user_meals_for_date(
    conn: &mut PgConnection,
    user_id: Uuid,
    date: NaiveDate,
) -> Result<Vec<UserMealCalculated>, Error> {
    user_meals_calculated::table
        .filter(user_meals_calculated::user_id.eq(user_id))
        .filter(user_meals_calculated::meal_date.eq(date))
        .select(user_meals_calculated::table::all_columns())
        .load(conn)
}

pub fn get_user_meals_date_range_calories(
    conn: &mut PgConnection,
    user_id: Uuid,
    start_date: NaiveDate,
    end_date: Option<NaiveDate>,
) -> Result<HashMap<NaiveDate, f64>, Error> {
    let query = user_meals_calculated::table.filter(user_meals_calculated::user_id.eq(user_id));
    let results: Vec<UserMealCalculated> = match end_date {
        Some(date) => query
            .filter(user_meals_calculated::meal_date.between(start_date, date))
            .select(user_meals_calculated::table::all_columns())
            .load(conn)?,
        None => query
            .filter(user_meals_calculated::meal_date.eq(start_date))
            .select(user_meals_calculated::table::all_columns())
            .load(conn)?,
    };
    let mut date_calories = HashMap::new();
    for meal in results {
        *date_calories.entry(meal.meal_date).or_default() += meal.calc_calories;
    }
    Ok(date_calories)
}
