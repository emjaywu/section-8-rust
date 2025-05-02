mod data;
mod clustering;
mod plot;
mod utils;

use std::error::Error;
use data::load_cleaned_data;
use clustering::cluster_properties;
use plot::plot_clusters;

fn main() -> Result<(), Box<dyn Error>> {
    let path = "data/cleaned_subsidized_housing.csv";
    let properties = load_cleaned_data(path)?;
    println!("Loaded {} cleaned housing entries.", properties.len());

    let k = 4; // might adjust later
    let labels = cluster_properties(&properties, k)?;
    plot_clusters(&properties, &labels, "output/clusters.png")?;

    Ok(())
}