use calorie_db::*;

fn main(){
    let connection = &mut establish_connection();

    create_product(connection, "test product", &0);
}