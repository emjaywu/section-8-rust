//! Module "clustering": run k-means clustering on features (TotalUnits, ActiveSubs).

use linfa::prelude::*;
use linfa_clustering::KMeans;
use linfa::DatasetBase;
use ndarray::Array2;
use crate::data::HousingProperty;
use std::collections::HashMap;

/// Converts "properties" to Array2<f64> w/ normalized features for linfa clustering
/// & returns (scaled_data, mins, ranges) to denormalize later
/// 
/// Inputs
/// "properties": slice of "HousingProperty"
/// 
/// Outputs
/// (Array2<f64>, [f64;2], [f64;2])
/// Scaled data in [0, 1]
/// "mins": original minimum per feature
/// "ranges": original difference between maximum & minimum per feature
/// 
/// Logic - (1) fill raw values, (2) compute min & max per column, (3) scale each value 
fn to_ndarray_with_scales(properties: &[HousingProperty]) -> (Array2<f64>, [f64; 2], [f64; 2]) {
    let n = properties.len();
    let mut data = Array2::<f64>::zeros((n, 2));

    // fill raw values
    for (i, p) in properties.iter().enumerate() {
        data[[i, 0]] = p.total_units as f64;
        data[[i, 1]] = p.subsidy_count as f64;
    }

    // compute mins & maxs
    let mut mins = [f64::INFINITY; 2];
    let mut maxs = [f64::NEG_INFINITY; 2];
    for col in 0..2 {
        for &v in data.column(col).iter() {
            if v < mins[col] { mins[col] = v; }
            if v > maxs[col] { maxs[col] = v; }
        }
    }

    // scaling into [0, 1]
    let mut ranges = [0.0; 2];
    for col in 0..2 {
        let range = maxs[col] - mins[col];
        ranges[col] = range;
        if range > 0.0 {
            let mut col_data = data.column_mut(col);
            for x in col_data.iter_mut() {
                *x = (*x - mins[col]) / range;
            }
        }
    }

    (data, mins, ranges)
}

/// Run k-means clustering on the aforementioned properties
/// 
/// Inputs
/// "properties": ""
/// "k": # of clusters
///
/// Outputs
/// "Ok(Vec<usize>)" w/ 1 label per property
/// "Err" if all else fails
///
/// Logic - (1) call function, (2) fit k-means, (3) predict labels & receive centroids, (4) denormalize & print
pub fn cluster_properties(properties: &[HousingProperty], k: usize) -> Result<Vec<usize>, Box<dyn std::error::Error>> {
    // scale & keep mins/ranges
    let (data, mins, ranges) = to_ndarray_with_scales(properties);
    let dataset = DatasetBase::from(data);

    // fit k-means
    let model = KMeans::params(k).max_n_iterations(100).fit(&dataset)?;
    let labels = model.predict(&dataset);
    let centroids = model.centroids();

    // denormalize centroids
    println!("Cluster centroids:");
    for (i, c) in centroids.outer_iter().enumerate() {
        let units = c[0] * ranges[0] + mins[0];
        let subs  = c[1] * ranges[1] + mins[1];
        println!("Cluster {}:", i);
        println!("Total Units: {:.0}", units); // can't have a fraction of a unit
        println!("Active Subsidies: {:.0}", subs); // nor can that be the case for subsidies
    }

    // count cluster sizes
    let mut counts = vec![0; k];
    for &lbl in labels.iter() { counts[lbl] += 1; }
    println!("\nCluster sizes:");
    for (i, &cnt) in counts.iter().enumerate() {
        println!("Cluster {}: {} properties", i, cnt);
    }

    // breakdown the owner type
    println!("\nOwnerType distribution by cluster:");
    let mut dist: Vec<HashMap<String, usize>> = vec![HashMap::new(); k];
    for (idx, &lbl) in labels.iter().enumerate() {
        let owner = properties[idx].owner_type.clone();
        *dist[lbl].entry(owner).or_insert(0) += 1;
    }
    for (i, map) in dist.iter().enumerate() {
        println!("Cluster {}:", i);
        for (owner, &cnt) in map {
            println!("{}: {}", owner, cnt);
        }
    }

    Ok(labels.to_vec())
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::data::HousingProperty;
	use std::collections::HashSet;

	#[test]
	fn test_two_clear_clusters() {
		let props = vec![
			HousingProperty { total_units: 1, subsidy_count: 1, owner_type: "A".into() },
			HousingProperty { total_units: 2, subsidy_count: 2, owner_type: "A".into() },
			HousingProperty { total_units: 99, subsidy_count: 99, owner_type: "B".into() },
			HousingProperty { total_units: 100, subsidy_count: 100, owner_type: "B".into() },
		];
		let labels = cluster_properties(&props, 2).expect("Clustering failed");

		// Should get 2 distinct labels
        let uniq: HashSet<_> = labels.iter().cloned().collect();
		assert_eq!(uniq.len(), 2);

		// First 2 share a label, last 2 share another
		assert_eq!(labels[0], labels[1]);
		assert_eq!(labels[2], labels[3]);
		assert_ne!(labels[0], labels[2]);
    }
}