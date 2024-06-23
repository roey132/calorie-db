#![allow(dead_code)]
use actix_cors::Cors;
use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result};
use chrono::NaiveDate;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use dotenv::dotenv;
use models::{MealEnum, UserMealCalculated};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use uuid;
mod load_food_data;
mod middleware;
mod models;
mod product_measures;
mod products;
mod schema;
mod test;
mod user_extractor;
mod user_meals;
mod user_meals_calculated;
mod users;
type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Debug, thiserror::Error)]
pub enum ServerError {
    #[error("Database Error: {0}")]
    DbError(#[from] diesel::result::Error),
    #[error("R2D2 Connection Error")]
    ConnectionError(#[from] r2d2::Error),
    #[error("Parse Error {0}")]
    ParseError(#[from] uuid::Error),
    #[error("User Unauthorized")]
    Unauthorized,
    #[error("{0}")]
    CustomError(String),
}

impl actix_web::error::ResponseError for ServerError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            ServerError::DbError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ServerError::ParseError(_) => StatusCode::BAD_REQUEST,
            ServerError::Unauthorized => StatusCode::UNAUTHORIZED,
            ServerError::ConnectionError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ServerError::CustomError(_) => StatusCode::BAD_REQUEST,
        }
    }
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[derive(Deserialize)]
struct UserProductEdit {
    product_name: String,
    product_id: i32,
    calories_per_100g: f64,
}
#[post("/products/user_product/edit")]
async fn edit_user_product(
    pool: web::Data<DbPool>,
    info: web::Json<UserProductEdit>,
    _: models::User,
) -> Result<HttpResponse, ServerError> {
    let mut conn = pool.get()?;

    let calories_per_gram = info.calories_per_100g / 100.0;

    products::update_product_by_id(
        &mut conn,
        info.product_id,
        &info.product_name,
        calories_per_gram,
    )?;

    Ok(HttpResponse::Ok().body("Successfully updated product"))
}

#[get("products/product/delete/{id}")]
async fn delete_product_by_id(
    pool: web::Data<DbPool>,
    info: web::Path<(i32,)>,
    _: models::User,
) -> Result<HttpResponse, ServerError> {
    let mut conn = pool.get()?;

    products::delete_product_by_id(&mut conn, info.0)?;

    Ok(HttpResponse::Ok().body(format!("Successfully deleted product {}", info.0)))
}

#[derive(Deserialize)]
struct UserProductInfo {
    product_name: String,
    calories_per_100g: f64,
    protein_per_100g: Option<f64>,
    carbs_per_100g: Option<f64>,
    fats_per_100g: Option<f64>,
}
#[post("/products/user_product/create")]
async fn create_user_product(
    pool: web::Data<DbPool>,
    info: web::Json<UserProductInfo>,
    user: models::User,
) -> Result<HttpResponse, ServerError> {
    let mut conn = pool.get()?;

    let calories_per_gram = info.calories_per_100g / 100.0;
    let protein_per_gram = match info.protein_per_100g {
        Some(value) => Some(value / 100.0),
        None => None,
    };
    let carbs_per_gram = match info.carbs_per_100g {
        Some(value) => Some(value / 100.0),
        None => None,
    };
    let fats_per_gram = match info.fats_per_100g {
        Some(value) => Some(value / 100.0),
        None => None,
    };

    products::create_product_for_user(
        &mut conn,
        &info.product_name,
        calories_per_gram,
        protein_per_gram,
        carbs_per_gram,
        fats_per_gram,
        &user.user_id,
    )?;

    Ok(HttpResponse::Ok().body("Successfully created new product"))
}

#[get("/products/product/get/{id}")]
async fn get_product(
    pool: web::Data<DbPool>,
    path: web::Path<(i32,)>,
    _: models::User,
) -> Result<web::Json<HashMap<String, models::Product>>, ServerError> {
    let product_id = path.0;
    let mut conn = pool.get()?;

    let result = products::get_product_by_id(&mut conn, product_id)?;

    let mut product_map: HashMap<String, models::Product> = HashMap::new();
    product_map.insert("product".to_string(), result);

    Ok(web::Json(product_map))
}

#[get("/products/user")]
async fn get_products_for_user_id(
    pool: web::Data<DbPool>,
    user: models::User,
) -> Result<web::Json<HashMap<i32, models::Product>>, ServerError> {
    let mut conn = pool.get()?;

    let user_id = user.user_id;
    let results = products::get_products_by_user(&mut conn, Some(user_id))?;
    let mut products_map: HashMap<i32, models::Product> = HashMap::new();
    for product in results {
        products_map.insert(product.product_id, product);
    }
    Ok(web::Json(products_map))
}

#[derive(Deserialize)]
struct UserData {
    username: String,
    password: String,
}
#[post("/users/create")]
async fn create_new_user(
    pool: web::Data<DbPool>,
    info: web::Json<UserData>,
) -> Result<HttpResponse, ServerError> {
    let mut conn = pool.get()?;
    let hashed_pass = users::hash_password(info.password.clone())
        .map_err(|_| ServerError::CustomError("Failed to hash password".to_string()))?;
    users::create_user(&mut conn, info.username.clone(), hashed_pass)?;
    Ok(HttpResponse::Ok().body("Successfully created user"))
}
#[post("/users/login")]
async fn user_login(
    pool: web::Data<DbPool>,
    info: web::Json<UserData>,
) -> Result<web::Json<HashMap<String, uuid::Uuid>>, ServerError> {
    let mut conn = pool.get()?;
    let user_id = users::check_user_password(&mut conn, info.username.clone(), &info.password)?;
    let mut uuid = HashMap::new();
    uuid.insert("UUID".to_string(), user_id);
    Ok(web::Json(uuid))
}

#[get("/products/system")]
async fn get_all_non_user_products(
    pool: web::Data<DbPool>,
    _: models::User,
) -> Result<web::Json<HashMap<i32, models::Product>>, ServerError> {
    let mut conn = pool.get()?;

    let results = products::get_products_by_user(&mut conn, None)?;
    let mut products_map: HashMap<i32, models::Product> = HashMap::new();
    for product in results {
        products_map.insert(product.product_id, product);
    }
    Ok(web::Json(products_map))
}

#[get("/measures/measure/{id}")]
async fn get_measure_by_id(
    pool: web::Data<DbPool>,
    info: web::Path<(i32,)>,
    _: models::User,
) -> Result<web::Json<HashMap<String, models::ProductMeasure>>, ServerError> {
    let mut conn = pool.get()?;
    let measure_id = info.0;
    let result = product_measures::get_product_measure_by_measure_id(&mut conn, measure_id)?;
    let mut map = HashMap::new();
    map.insert("measure".to_string(), result);

    Ok(web::Json(map))
}

#[get("/measures/product/{id}")]
async fn get_measures_for_product(
    pool: web::Data<DbPool>,
    info: web::Path<(i32,)>,
    _: models::User,
) -> Result<web::Json<HashMap<i32, models::ProductMeasure>>, ServerError> {
    let mut conn = pool.get()?;
    let product_id = info.0;
    let results = product_measures::get_product_measures_by_product(&mut conn, product_id)?;
    let mut measures: HashMap<i32, models::ProductMeasure> = HashMap::new();
    for result in results {
        measures.insert(result.measure_id, result);
    }
    Ok(web::Json(measures))
}

#[derive(Deserialize)]
struct NewMeasure {
    product_id: i32,
    measure_name: String,
    measure_calories: f64,
}
#[post("measures/measure/create")]
async fn create_new_measure(
    pool: web::Data<DbPool>,
    info: web::Json<NewMeasure>,
    _: models::User,
) -> Result<HttpResponse, ServerError> {
    let mut conn = pool.get()?;
    product_measures::create_product_measure(
        &mut conn,
        info.product_id,
        info.measure_name.as_str(),
        info.measure_calories,
        false,
    )?;
    Ok(HttpResponse::Ok().body("Successfully created measure"))
}

#[derive(Deserialize)]
struct EditedMeasure {
    measure_id: i32,
    measure_name: String,
    measure_calories: f64,
}
#[post("measures/measure/edit")]
async fn edit_measure(
    pool: web::Data<DbPool>,
    info: web::Json<EditedMeasure>,
    _: models::User,
) -> Result<HttpResponse, ServerError> {
    let mut conn = pool.get()?;
    product_measures::update_product_measure_by_measure_id(
        &mut conn,
        info.measure_id,
        info.measure_name.as_str(),
        info.measure_calories,
    )?;
    Ok(HttpResponse::Ok().body("Successfully edited measure"))
}
#[get("measures/measure/delete/{id}")]
async fn delete_measure(
    pool: web::Data<DbPool>,
    info: web::Path<(i32,)>,
    _: models::User,
) -> Result<HttpResponse, ServerError> {
    let mut conn = pool.get()?;
    product_measures::delete_product_measure_by_measure_id(&mut conn, info.0)?;
    Ok(HttpResponse::Ok().body("Successfully deleted measure"))
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABSE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

#[get("meals/meal/{id}")]
async fn get_user_meal(
    pool: web::Data<DbPool>,
    info: web::Path<(i32,)>,
    _: models::User,
) -> Result<web::Json<HashMap<String, models::UserMeal>>, ServerError> {
    let mut conn = pool.get()?;
    let meal_id = info.0;
    let user_meal = user_meals::get_user_meal_by_id(&mut conn, meal_id)?;
    let mut map = HashMap::new();

    map.insert("meal".to_string(), user_meal);
    Ok(web::Json(map))
}

#[derive(Deserialize)]
struct NewCaloriesMeal {
    calories: f64,
    meal_date: NaiveDate,
    meal_name: Option<String>,
}
#[post("meals/meal/create/calories")]
async fn create_calories_meal(
    pool: web::Data<DbPool>,
    info: web::Json<NewCaloriesMeal>,
    user: models::User,
) -> Result<HttpResponse, ServerError> {
    let mut conn = pool.get()?;
    user_meals::create_user_meal_calories(
        &mut conn,
        &user.user_id,
        info.meal_name.as_deref(),
        info.calories,
        info.meal_date,
    )?;
    Ok(HttpResponse::Ok().body("Successfully created calories meal"))
}

#[derive(Deserialize)]
struct NewProductMeal {
    product_id: i32,
    product_grams: i32,
    meal_date: NaiveDate,
    meal_name: Option<String>,
}
#[post("meals/meal/create/product")]
async fn create_product_meal(
    pool: web::Data<DbPool>,
    info: web::Json<NewProductMeal>,
    user: models::User,
) -> Result<HttpResponse, ServerError> {
    let mut conn = pool.get()?;
    user_meals::create_user_meal_product(
        &mut conn,
        &user.user_id,
        info.product_id,
        info.product_grams,
        info.meal_date,
    )?;
    Ok(HttpResponse::Ok().body("Successfully created product meal"))
}

#[derive(Deserialize)]
struct NewMeasureMeal {
    measure_id: i32,
    measure_count: f64,
    meal_date: NaiveDate,
}
#[post("meals/meal/create/measure")]
async fn create_measure_meal(
    pool: web::Data<DbPool>,
    info: web::Json<NewMeasureMeal>,
    user: models::User,
) -> Result<HttpResponse, ServerError> {
    let mut conn = pool.get()?;
    user_meals::create_user_meal_measure(
        &mut conn,
        &user.user_id,
        info.measure_id,
        info.measure_count,
        info.meal_date,
    )?;
    Ok(HttpResponse::Ok().body("Successfully created measure meal"))
}

#[get("meals/meal/delete/{id}")]
async fn delete_user_meal(
    pool: web::Data<DbPool>,
    info: web::Path<(i32,)>,
    _: models::User,
) -> Result<HttpResponse, ServerError> {
    let mut conn = pool.get()?;
    user_meals::delete_user_meal_by_id(&mut conn, info.0)?;
    Ok(HttpResponse::Ok().body("Successfully deleted meal"))
}

#[derive(Deserialize)]
struct EditedProductMeal {
    meal_id: i32,
    product_grams: i32,
}
#[post("meals/meal/edit/product")]
async fn edit_product_meal(
    pool: web::Data<DbPool>,
    info: web::Json<EditedProductMeal>,
    _: models::User,
) -> Result<HttpResponse, ServerError> {
    let mut conn = pool.get()?;

    let meal = user_meals::get_user_meal_by_id(&mut conn, info.meal_id)?;
    if meal.meal_type != MealEnum::Product {
        return Err(ServerError::Unauthorized);
    }

    user_meals::update_user_meal_product_by_meal_id(&mut conn, info.meal_id, info.product_grams)?;
    Ok(HttpResponse::Ok().body("Successfully edited meal"))
}

#[derive(Deserialize)]
struct EditedCaloriesMeal {
    meal_id: i32,
    calories: f64,
}
#[post("meals/meal/edit/calories")]
async fn edit_calories_meal(
    pool: web::Data<DbPool>,
    info: web::Json<EditedCaloriesMeal>,
    _: models::User,
) -> Result<HttpResponse, ServerError> {
    let mut conn = pool.get()?;

    let meal = user_meals::get_user_meal_by_id(&mut conn, info.meal_id)?;
    if meal.meal_type != MealEnum::Calories {
        return Err(ServerError::Unauthorized);
    }

    user_meals::update_user_meal_calories_by_meal_id(&mut conn, info.meal_id, info.calories)?;
    Ok(HttpResponse::Ok().body("Successfully edited meal"))
}

#[derive(Deserialize)]
struct EditedMeasureMeal {
    meal_id: i32,
    measure_count: f64,
}
#[post("meals/meal/edit/measure")]
async fn edit_measure_meal(
    pool: web::Data<DbPool>,
    info: web::Json<EditedMeasureMeal>,
    _: models::User,
) -> Result<HttpResponse, ServerError> {
    let mut conn = pool.get()?;

    let meal = user_meals::get_user_meal_by_id(&mut conn, info.meal_id)?;
    if meal.meal_type != MealEnum::Measure {
        return Err(ServerError::CustomError(
            "Meal type is not Measure".to_string(),
        ));
    }

    user_meals::update_user_meal_measure_by_meal_id(&mut conn, info.meal_id, info.measure_count)?;
    Ok(HttpResponse::Ok().body("Successfully edited meal"))
}

#[derive(Deserialize)]
struct DateRange {
    start_date: NaiveDate,
    end_date: Option<NaiveDate>,
}
#[post("meals/total_calories")]
async fn get_total_calories_for_user(
    pool: web::Data<DbPool>,
    info: web::Json<DateRange>,
    user: models::User,
) -> Result<web::Json<HashMap<NaiveDate, f64>>, ServerError> {
    let mut conn = pool.get()?;
    let results = user_meals_calculated::get_user_meals_date_range_calories(
        &mut conn,
        user.user_id,
        info.start_date,
        info.end_date,
    )?;
    Ok(web::Json(results))
}

#[get["meals/date/{date}"]]
async fn get_user_meals_for_date(
    pool: web::Data<DbPool>,
    info: web::Path<(NaiveDate,)>,
    user: models::User,
) -> Result<web::Json<HashMap<i32, UserMealCalculated>>, ServerError> {
    let mut conn = pool.get()?;
    let results = user_meals_calculated::get_user_meals_for_date(&mut conn, user.user_id, info.0)?;
    let mut meals = HashMap::new();
    for meal in results {
        meals.insert(meal.meal_id, meal);
    }
    Ok(web::Json(meals))
}

#[derive(Deserialize, Serialize)]
struct Profile {
    user_id: String,
    username: String,
}
#[get("users/profile")]
async fn get_user_profile(user: models::User) -> Result<web::Json<Profile>, ServerError> {
    let user_profile = Profile {
        user_id: user.user_id.to_string(),
        username: user.username,
    };
    Ok(web::Json(user_profile))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let database_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL must be set in env variables");

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create R2D2 pool");

    HttpServer::new(move || {
        App::new().app_data(web::Data::new(pool.clone())).service(
            web::scope("/api")
                .wrap(
                    Cors::default()
                        .allow_any_origin()
                        .allow_any_method()
                        .allow_any_header(),
                )
                .service(get_product)
                .service(get_products_for_user_id)
                .service(get_all_non_user_products)
                .service(delete_product_by_id)
                .service(create_user_product)
                .service(get_measures_for_product)
                .service(edit_user_product)
                .service(delete_measure)
                .service(edit_measure)
                .service(create_new_measure)
                .service(create_calories_meal)
                .service(create_measure_meal)
                .service(create_product_meal)
                .service(delete_user_meal)
                .service(edit_calories_meal)
                .service(edit_product_meal)
                .service(edit_measure_meal)
                .service(get_total_calories_for_user)
                .service(get_user_meals_for_date)
                .service(create_new_user)
                .service(user_login)
                .service(get_user_profile)
                .service(get_measure_by_id)
                .service(get_user_meal),
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
