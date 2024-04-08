use self::models::*;
use diesel::prelude::*;
use calorie_db::*;

fn main(){
    use self::schema::products::dsl::*;

    let connection = &mut establish_connection();
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
}