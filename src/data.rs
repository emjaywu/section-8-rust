//! Module "data": Load & deserialize cleaned CSV rows into "HousingProperty" structs.

use std::error::Error;
use std::fs::File;
use serde::Deserialize;
use csv::Reader;

/// Represents the cleaned dataset based on units, subsidy count, & owner type
#[derive(Debug, Deserialize)]
pub struct HousingProperty {
    #[serde(rename = "TotalUnits")]
    pub total_units: u32,

    #[serde(rename = "ActiveSubs")]
    pub subsidy_count: u32,

    #[serde(rename = "OwnerType")]
    pub owner_type: String,
}

/// Reads the cleaned CSV file at "path" & returns "HousingProperty" vector
/// 
/// Inputs
/// "path": file path to the cleaned CSV data
///
/// Outputs
/// "Ok(Vec<HousingProperty>)" if successful
/// "Err" if unsuccessful
/// 
/// Logic - (1) Open the file, (2) build a CSV reader, (3) push valid & skip invalid rows
pub fn load_cleaned_data(path: &str) -> Result<Vec<HousingProperty>, Box<dyn Error>> {
    // open CSV for reading
    let file = File::open(path)?;
    // create CSV reader from file
    let mut rdr = Reader::from_reader(file);

    let mut properties = Vec::new();
    // iterate over deserialized rows
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