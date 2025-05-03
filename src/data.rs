use std::error::Error;
use std::fs::File;
use serde::Deserialize;
use csv::Reader;

/// Represent a cleaned housing entry from cleaned data
#[derive(Debug, Deserialize)]
pub struct HousingProperty {
    #[serde(rename = "TotalUnits")]
    pub total_units: u32,

    #[serde(rename = "ActiveSubs")]
    pub subsidy_count: u32,

    #[serde(rename = "OwnerType")]
    pub owner_type: String,
}

/// Load filtered CSV into memory, skipping invalid rows
pub fn load_cleaned_data(path: &str) -> Result<Vec<HousingProperty>, Box<dyn Error>> {
    let file = File::open(path)?;
    let mut rdr = Reader::from_reader(file);

    let mut properties = Vec::new();
    for result in rdr.deserialize() {
        match result {
            Ok(entry) => properties.push(entry),
            Err(e) => eprintln!("Skipping invalid row: {}", e),
        }
    }

    Ok(properties)
}