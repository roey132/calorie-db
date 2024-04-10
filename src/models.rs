use diesel::prelude::*;
use uuid::Uuid;
use chrono::NaiveDateTime;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::products)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Product {
    pub product_id: i32,
    pub product_name: String,
    pub calories_per_gram: Option<f64>,
    pub user_id: Option<Uuid>,
    pub create_time: NaiveDateTime,
    pub update_time: Option<NaiveDateTime>
}

use crate::schema::products;
#[derive(Insertable)]
#[diesel(table_name = products)]
pub struct NewUserProduct<'a> {
    pub product_name: &'a str,
    pub calories_per_gram: &'a f64,
    pub user_id: &'a Uuid,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::product_measures)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ProductMeasure{
    pub product_id: i32,
    pub measure_id: i32,
    pub default_measure: Option<bool>,
    pub measure_name: String,
    pub measure_calories: i32,
    pub create_time: NaiveDateTime,
    pub update_time: Option<NaiveDateTime>
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User{
    pub user_id: Uuid,
    pub username: String,
    pub password: String,
    pub create_time: NaiveDateTime,
    pub update_time: Option<NaiveDateTime>
}

use crate::schema::users;
#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a>{
    pub username: &'a String,
    pub password: &'a String
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::user_meals)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserMeal{
    pub meal_id: i32,
    pub user_id: Uuid,
    pub product_id: Option<i32>,
    pub measure_id: Option<i32>,
    pub measure_count: Option<i32>,
    pub calories: Option<f64>,
    pub meal_name: Option<String>,
    pub meal_note: Option<String>,
    pub meal_time: NaiveDateTime,
    pub create_time: NaiveDateTime,
    pub update_time: Option<NaiveDateTime>
}