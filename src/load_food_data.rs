use serde::Deserialize;
use std::error::Error;
use std::fs::File;

use crate::{establish_connection, products};

#[derive(Debug, Deserialize)]
struct NutrientRecord {
    name: String,
    serving_size: String,
    calories: f64,
    protein: Option<String>,
    carbohydrate: Option<String>,
    total_fat: Option<String>,
}

fn read_csv(file_path: &str) -> Result<Vec<NutrientRecord>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(file);
    let mut records = Vec::new();

    for result in rdr.deserialize() {
        let record: NutrientRecord = result?;
        records.push(record);
    }

    Ok(records)
}
#[test]
fn write_data() -> Result<(), Box<dyn Error>> {
    let path = "./food_data/filtered_nutrition.csv";
    let records = read_csv(path)?;
    let conn = &mut establish_connection();
    for record in records {
        products::create_system_product(conn, &record.name, record.calories / 100.0).unwrap();
    }

    Ok(())
}
