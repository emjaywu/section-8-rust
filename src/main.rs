mod data;

use data::{load_cleaned_data, HousingProperty};

fn main() {
    let path = "data/Subsidized_Housing_-_Six_Metro_Areas_-_2017.csv";

    match load_cleaned_data(path) {
        Ok(properties) => {
            println!("Successfully loaded {} housing entries.", properties.len());

            // Print out the first 3 entries for confirmation (TENTATIVE)
            for (i, property) in properties.iter().take(3).enumerate() {
                println!("\n--- Property {} ---", i + 1);
                println!("Total Units: {}", property.total_units);
                println!("Subsidies: {}", property.subsidy_count);
                println!("Latitude: {}", property.latitude);
                println!("Longitude: {}", property.longitude);
                println!("Owner Type: {}", property.owner_type);
            }
        },
        Err(e) => {
            eprintln!("Failed to load dataset: {}", e);
        }
    }
}
