use linfa::prelude::*;
use linfa_clustering::KMeans;
use linfa::DatasetBase;
use ndarray::Array2;
use crate::data::HousingProperty;
use std::collections::HashMap;

/// Converts HousingProperty to Array2<f64> with normalized features for linfa clustering
fn to_ndarray(properties: &[HousingProperty]) -> Array2<f64> {
	let n_rows = properties.len();
	let mut data = Array2::<f64>::zeros((n_rows, 2));

	for (i, prop) in properties.iter().enumerate() {
		data[[i, 0]] = prop.total_units as f64;
		data[[i, 1]] = prop.subsidy_count as f64;
	}
	/*
	let n_rows = properties.len();
	let n_cols = 4; // TotalUnits, ActiveSubs, Latitude, Longitude
	let mut data = Array2::<f64>::zeros((n_rows, n_cols));
	for (i, p) in properties.iter().enumerate() {
		data[[i, 0]] = p.total_units as f64;
		data[[i, 1]] = p.subsidy_count as f64;
		data[[i, 2]] = p.latitude;
		data[[i, 3]] = p.longitude;
	}
	*/
	data
}

/// Run k-means clustering via linfa
pub fn cluster_properties(properties: &[HousingProperty], k: usize) -> Result<Vec<usize>, Box<dyn std::error::Error>> {
	let data = to_ndarray(properties);
	let dataset = DatasetBase::from(data); // added to fix mismatched types
	let model = KMeans::params(k).max_n_iterations(100).fit(&dataset)?;
	let labels = model.predict(&dataset);
	let centroids = model.centroids();

	println!("Cluster centroids:");
	for (i, centroid) in centroids.outer_iter().enumerate() {
		println!("Cluster {}:", i);
		println!("Total Units: {:.2}", centroid[0]);
		println!("Active Subsidies: {:.2}", centroid[1]);
		// println!("Latitude: {:.4}", centroid[2]);
		// println!("Longitude: {:.4}", centroid[3]);
	}

	// Count the # of points in each cluster
	let mut counts = vec![0; k];
	for &label in labels.iter() {
		counts[label] += 1;
	}

	println!("\nCluster sizes:");
	for (i, count) in counts.iter().enumerate() {
		println!("Cluster {}: {} properties", i, count);
	}

	println!("\nOwnerType distribution by cluster:");
	let mut type_counts: Vec<HashMap<String, usize>> = vec![HashMap::new(); k];

	for (i, &label) in labels.iter().enumerate() {
		let owner_type = properties[i].owner_type.to_string();
		let cluster_map = &mut type_counts[label];
		*cluster_map.entry(owner_type).or_insert(0) += 1;
	}

	for (cluster_idx, counts) in type_counts.iter().enumerate() {
		println!("Cluster {}:", cluster_idx);
		for (owner_type, count) in counts {
			println!("{}: {}", owner_type, count);
		}
	}
	Ok(labels.to_vec())
}