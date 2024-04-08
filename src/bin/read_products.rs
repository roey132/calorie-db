use self::models::*;
use diesel::prelude::*;
use calorie_db::*;
use uuid::Uuid;

fn get_product_by_id(conn:&mut PgConnection,id:&i32) -> Option<Product> {
    use self::schema::products::dsl::*;
    let mut results = products
        .filter(product_id.eq(id))
        .limit(1)
        .select(Product::as_select())
        .load(conn)
        .expect("Error loading product");

        if results.len() == 1{
        Some(results.remove(0))
    } else {
        None
    }
}
fn get_products_by_user(conn:&mut PgConnection, id:Option<Uuid>) -> Vec<Product> {
    use self::schema::products::dsl::*;
    let mut query = products.into_boxed();
    if let Some(value) = id{
        query = query.filter(user_id.eq(value))
    } else {
        query = query.filter(user_id.is_null())
    }
    let results = query
        .select(Product::as_select())
        .load(conn)
        .expect("Error loading products");

    results
}

fn main(){
    /* 
    use self::schema::products::dsl::*;

    let results = products
        .filter(update_time.is_null())
        .limit(5)
        .select(Product::as_select())
        .load(connection)
        .expect("Error loading products");

    println!("Displaying {} products", results.len());
    for product in results { 
        println!("{}",product.product_id);
        println!("=====================");
        println!("{}",product.product_name);
    }
    */
    let connection = &mut establish_connection();
    if let Some(product) = get_product_by_id(connection,&1){
        println!("{}",product.product_id);
        println!("================");
        println!("product_name: {}",product.product_name);
        println!("calories_1gram: {:?}",product.calories_1gram);
        println!("user_id: {:?}",product.user_id);
        println!("create_time: {}",product.create_time);
        println!("update_time: {:?}",product.update_time);
    }
    else{
        println!("Id not found.")
    }
    let test_uuid = Some(Uuid::new_v4());
    println!("Generated UUID: {:?}",test_uuid);
    let products = get_products_by_user(connection, test_uuid);
    println!("{}",products.len());

    for product in products{
        println!("{}",product.product_id)
    }

}