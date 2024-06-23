use chrono::NaiveDate;
use chrono::NaiveDateTime;
use diesel::deserialize;
use diesel::deserialize::FromSql;
use diesel::deserialize::FromSqlRow;
use diesel::expression::AsExpression;
use diesel::pg::{Pg, PgValue};
use diesel::prelude::*;
use diesel::serialize::{IsNull, Output, ToSql};
use serde::Deserialize;
use serde::Serialize;
use std::io::Write;
use uuid::Uuid;

#[derive(Queryable, Selectable, Deserialize, Serialize)]
#[diesel(table_name = crate::schema::products)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Product {
    pub product_id: i32,
    pub product_name: String,
    pub calories_per_gram: f64,
    pub user_id: Option<Uuid>,
    pub create_time: NaiveDateTime,
    pub update_time: Option<NaiveDateTime>,
    pub delete_time: Option<NaiveDateTime>,
    pub protein_per_gram: Option<f64>,
    pub carbs_per_gram: Option<f64>,
    pub fats_per_gram: Option<f64>,
}

use crate::schema::products;
#[derive(Insertable)]
#[diesel(table_name = products)]
pub struct NewUserProduct<'a> {
    pub product_name: &'a str,
    pub calories_per_gram: f64,
    pub user_id: &'a Uuid,
    pub protein_per_gram: Option<f64>,
    pub carbs_per_gram: Option<f64>,
    pub fats_per_gram: Option<f64>,
}

#[derive(Queryable, Selectable, Deserialize, Serialize)]
#[diesel(table_name = crate::schema::product_measures)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ProductMeasure {
    pub product_id: i32,
    pub measure_id: i32,
    pub default_measure: Option<bool>,
    pub measure_name: String,
    pub measure_calories: f64,
    pub create_time: NaiveDateTime,
    pub update_time: Option<NaiveDateTime>,
    pub delete_time: Option<NaiveDateTime>,
}

use crate::schema::product_measures;
#[derive(Insertable)]
#[diesel(table_name = product_measures)]
pub struct NewProductMeasure<'a> {
    pub product_id: i32,
    pub measure_name: &'a str,
    pub measure_calories: f64,
    pub default_measure: bool,
}

#[derive(Queryable, Selectable, Clone)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub user_id: Uuid,
    pub username: String,
    pub password: String,
    pub create_time: NaiveDateTime,
    pub update_time: Option<NaiveDateTime>,
}

use crate::schema::sql_types::MealType;
use crate::schema::users;
#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub username: String,
    pub password: String,
}

#[derive(Debug, PartialEq, FromSqlRow, AsExpression, Eq, Serialize, Deserialize)]
#[diesel(sql_type = crate::schema::sql_types::MealType)]
pub enum MealEnum {
    Calories,
    Measure,
    Product,
}
impl FromSql<MealType, Pg> for MealEnum {
    fn from_sql(bytes: PgValue<'_>) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"Calories" => Ok(MealEnum::Calories),
            b"Measure" => Ok(MealEnum::Measure),
            b"Product" => Ok(MealEnum::Product),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}
impl ToSql<MealType, Pg> for MealEnum {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> diesel::serialize::Result {
        match *self {
            MealEnum::Calories => out.write_all(b"Calories")?,
            MealEnum::Measure => out.write_all(b"Measure")?,
            MealEnum::Product => out.write_all(b"Product")?,
        }
        Ok(IsNull::No)
    }
}

#[derive(Insertable, Queryable, Debug, PartialEq, Deserialize, Serialize)]
#[diesel(table_name = crate::schema::user_meals)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserMeal {
    pub meal_id: i32,
    pub user_id: Uuid,
    pub meal_type: MealEnum,
    pub product_id: Option<i32>,
    pub product_grams: Option<i32>,
    pub measure_id: Option<i32>,
    pub measure_count: Option<f64>,
    pub calories: Option<f64>,
    pub meal_name: Option<String>,
    pub meal_date: NaiveDate,
    pub create_time: NaiveDateTime,
    pub update_time: Option<NaiveDateTime>,
    pub protein: Option<f64>,
    pub carbs: Option<f64>,
    pub fats: Option<f64>,
}

use crate::schema::user_meals;
#[derive(Insertable)]
#[diesel(table_name = user_meals)]
pub struct NewUserMealProduct<'a> {
    pub user_id: &'a Uuid,
    pub meal_type: MealEnum,
    pub product_id: i32,
    pub product_grams: i32,
    pub meal_date: NaiveDate,
}

#[derive(Insertable)]
#[diesel(table_name = user_meals)]
pub struct NewUserMealMeasure<'a> {
    pub user_id: &'a Uuid,
    pub meal_type: MealEnum,
    pub measure_id: i32,
    pub measure_count: f64,
    pub meal_date: NaiveDate,
}

#[derive(Insertable)]
#[diesel(table_name = user_meals)]
pub struct NewUserMealCalories<'a> {
    pub user_id: &'a Uuid,
    pub meal_type: MealEnum,
    pub calories: f64,
    pub protein: Option<f64>,
    pub carbs: Option<f64>,
    pub fats: Option<f64>,
    pub meal_name: Option<&'a str>,
    pub meal_date: NaiveDate,
}

#[derive(Queryable, Serialize, Deserialize)]
#[diesel(table_name = crate::user_meals_calculated::user_meals)]
pub struct UserMealCalculated {
    pub meal_id: i32,
    pub user_id: Uuid,
    pub meal_type: MealEnum,
    pub product_name: Option<String>,
    pub product_grams: Option<i32>,
    pub measure_name: Option<String>,
    pub measure_count: Option<f64>,
    pub meal_date: NaiveDate,
    pub calc_calories: f64,
}
