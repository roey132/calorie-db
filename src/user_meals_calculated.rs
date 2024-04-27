use diesel::query_dsl::methods::{FilterDsl, SelectDsl};
use diesel::result::Error;
use diesel::RunQueryDsl;
use diesel::Table;
use diesel::{ExpressionMethods, PgConnection};

use crate::models::UserMealsCalculated;

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
    }
}

pub fn get_user_meal_calories(
    conn: &mut PgConnection,
    meal_id: i32,
) -> Result<UserMealsCalculated, Error> {
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
