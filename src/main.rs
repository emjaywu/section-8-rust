mod data;

use std::error::Error;
use data::{load_cleaned_data, HousingProperty};

fn main() -> Result<(), Box<dyn Error>> {
    let path = "data/cleaned_subsidized_housing.csv";
    let properties = load_cleaned_data(path)?;
    println!("Loaded {} cleaned housing entries.", properties.len());

    // might delete later
    for (i, entry) in properties.iter().take(3).enumerate() {
        println!("\n--- Entry {} ---", i + 1);
        println!("Total Units: {}", entry.total_units);
        println!("Subsidies: {}", entry.subsidy_count);
        println!("Latitude: {}", entry.latitude);
        println!("Longitude: {}", entry.longitude);
        println!("Owner Type: {}", entry.owner_type);
    }

    Ok(())
}
