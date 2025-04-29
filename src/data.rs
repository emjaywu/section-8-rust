use std::error::Error;
use std::fs::File;
use serde::Deserialize;
use serde::de::{self, Deserializer};
use csv;

#[derive(Debug, Deserialize)]
pub struct HousingProperty {
    #[serde(rename = "TotalUnits")]
    pub total_units: u32,

    #[serde(rename = "ActiveSubs")]
    pub subsidy_count: u32,

    #[serde(rename = "Latitude")]
    pub latitude: f64,

    #[serde(rename = "Longitude")]
    pub longitude: f64,

    #[serde(rename = "OwnerType", deserialize_with = "parse_owner_type")]
    pub owner_type: u8,
}

// insert description here later
pub fn load_cleaned_data(path: &str) -> Result<Vec<HousingProperty>, Box<dyn Error>> {
    let file = File::open(path)?;
    let mut rdr = csv::Reader::from_reader(file);

    let mut properties = Vec::new();

    for result in rdr.deserialize() {
        let record: Result<HousingProperty, csv::Error> = result;

        match record {
            Ok(property) => properties.push(property),
            Err(e) => {
                // Skip invalid or incomplete rows
                eprintln!("Skipping invalid row: {}", e);
            }
        }
    }

    Ok(properties)
}

// function to parse the OwnerType field
fn parse_owner_type<'de, D>(deserializer: D) -> Result<u8, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    match s.trim().to_lowercase().as_str() {
        "for profit" => Ok(0),
        "non-profit" => Ok(1),
        "multiple" => Ok(2),
        "" => Err(de::Error::custom("Missing OwnerType")),
        other => Err(de::Error::custom(format!("Unrecognized OwnerType: {}", other))),
    }
}
