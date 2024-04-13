// @generated automatically by Diesel CLI.

diesel::table! {
    product_measures (measure_id) {
        product_id -> Int4,
        measure_id -> Int4,
        default_measure -> Nullable<Bool>,
        measure_name -> Varchar,
        measure_calories -> Float8,
        create_time -> Timestamp,
        update_time -> Nullable<Timestamp>,
    }
}

diesel::table! {
    products (product_id) {
        product_id -> Int4,
        product_name -> Varchar,
        calories_per_gram -> Float8,
        user_id -> Nullable<Uuid>,
        create_time -> Timestamp,
        update_time -> Nullable<Timestamp>,
    }
}

diesel::table! {
    user_meals (meal_id) {
        meal_id -> Int4,
        user_id -> Uuid,
        product_id -> Int4,
        product_grams -> Nullable<Int4>,
        measure_id -> Nullable<Int4>,
        measure_count -> Nullable<Float8>,
        calories -> Float8,
        meal_name -> Nullable<Varchar>,
        meal_note -> Nullable<Varchar>,
        meal_date -> Date,
        create_time -> Timestamp,
        update_time -> Nullable<Timestamp>,
    }
}

diesel::table! {
    users (user_id) {
        user_id -> Uuid,
        username -> Varchar,
        password -> Varchar,
        create_time -> Timestamp,
        update_time -> Nullable<Timestamp>,
    }
}

diesel::joinable!(product_measures -> products (product_id));
diesel::joinable!(products -> users (user_id));
diesel::joinable!(user_meals -> products (product_id));
diesel::joinable!(user_meals -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    product_measures,
    products,
    user_meals,
    users,
);
