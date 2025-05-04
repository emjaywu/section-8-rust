//! Module "main": starting point to load data, run clustering, and plot results. 

mod data;
mod clustering;
mod plot;
mod utils;

use std::error::Error;
use data::load_cleaned_data;
use clustering::cluster_properties;
use plot::plot_clusters;

/// Main function: loads data, clustering, & plotting
///
/// Outputs
/// "Ok(())" if ""
/// "Err()" if ""
///
/// Logic - (1) load cleaned data from CSV, (2) perform k-means clustering, (3) plot results
fn main() -> Result<(), Box<dyn Error>> {
    let path = "data/cleaned_subsidized_housing.csv";
    let properties = load_cleaned_data(path)?;
    println!("Loaded {} cleaned housing entries.", properties.len());

    let k = 4; // # of clusters
    let labels = cluster_properties(&properties, k)?;
    plot_clusters(&properties, &labels)?;

    Ok(())
}