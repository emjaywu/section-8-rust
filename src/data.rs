//! Module "data": Load & deserialize cleaned CSV rows into "HousingProperty" structs.

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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_load_cleaned_data_simple() {
        let csv = "\
        TotalUnits,ActiveSubs,OwnerType\n\
        10,2,Non-Profit\n\
        20,5,For Profit\n\
        ";
        let path = "test_simple.csv";
        fs::write(path, csv).expect("Unable to write test CSV");

        let props = load_cleaned_data(path).expect("Failed to load data");
        assert_eq!(props.len(), 2);
        assert_eq!(props[0].total_units, 10);
        assert_eq!(props[0].subsidy_count, 2);
        assert_eq!(props[0].owner_type, "Non-Profit");

        fs::remove_file(path).unwrap();
    }
}