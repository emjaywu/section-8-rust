use linfa::prelude::*;
use linfa_clustering::KMeans;
use linfa::DatasetBase;
use ndarray::Array2;
use crate::data::HousingProperty;
use std::collections::HashMap;

/// Converts HousingProperty to Array2<f64> with normalized features for linfa clustering
/// Returns (scaled_data, mins, ranges) so we can denormalize later.
fn to_ndarray_with_scales(
    properties: &[HousingProperty],
) -> (Array2<f64>, [f64; 2], [f64; 2]) {
    let n = properties.len();
    let mut data = Array2::<f64>::zeros((n, 2));

    // 1) fill raw values for units & subsidies
    for (i, p) in properties.iter().enumerate() {
        data[[i, 0]] = p.total_units as f64;
        data[[i, 1]] = p.subsidy_count as f64;
    }

    // 2) compute mins and maxs
    let mut mins = [f64::INFINITY; 2];
    let mut maxs = [f64::NEG_INFINITY; 2];
    for col in 0..2 {
        for &v in data.column(col).iter() {
            if v < mins[col] { mins[col] = v; }
            if v > maxs[col] { maxs[col] = v; }
        }
    }

    // 3) min–max scale into [0,1], record ranges = max–min
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

/// Run k-means clustering via linfa
pub fn cluster_properties(
    properties: &[HousingProperty],
    k: usize,
) -> Result<Vec<usize>, Box<dyn std::error::Error>> {
    // scale and keep mins/ranges
    let (data, mins, ranges) = to_ndarray_with_scales(properties);
    let dataset = DatasetBase::from(data);

    // fit k-means (you can also add .n_runs(5) if you want multiple starts)
    let model = KMeans::params(k)
        .max_n_iterations(100)
        .fit(&dataset)?;
    let labels = model.predict(&dataset);
    let centroids = model.centroids();

    // print denormalized centroids
    println!("Cluster centroids:");
    for (i, c) in centroids.outer_iter().enumerate() {
        let units = c[0] * ranges[0] + mins[0];
        let subs  = c[1] * ranges[1] + mins[1];
        println!("Cluster {}:", i);
        println!("Total Units: {:.0}", units);
        println!("Active Subsidies: {:.0}", subs);
    }

    // cluster sizes
    let mut counts = vec![0; k];
    for &lbl in labels.iter() { counts[lbl] += 1; }
    println!("\nCluster sizes:");
    for (i, &cnt) in counts.iter().enumerate() {
        println!("Cluster {}: {} properties", i, cnt);
    }

    // owner-type breakdown
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