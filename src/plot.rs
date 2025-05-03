//! Module "plot": build scatterplots of clustering results, annotated.

use std::path::Path;
use plotters::prelude::*;
use crate::data::HousingProperty;
use crate::utils::get_cluster_color;

/// Plot a scatterplot (TotalUnits vs. ActiveSubs) by cluster ID, w/ a legend & "X" markers
pub fn plot_clusters(properties: &[HousingProperty], labels: &[usize]) -> Result<(), Box<dyn std::error::Error>> {
    // Create a filename for each new image (no overwriting)
    let mut idx = 1;
    let filename = loop {
        let f = format!("output/clusters_{}.png", idx);
        if !Path::new(&f).exists() {
            break f;
        }
        idx += 1;
    };

    // Create the graph 
    let root = BitMapBackend::new(&filename, (1600, 1200)).into_drawing_area(); // doubled from 800x600
    root.fill(&WHITE)?;

    // Configure the axes
    let (min_x, max_x) = min_max(properties.iter().map(|p| p.total_units as f64));
    let (min_y, max_y) = min_max(properties.iter().map(|p| p.subsidy_count as f64));

    let mut chart = ChartBuilder::on(&root) // breaking this line up for visibility
        .caption("Clustered Housing Properties (Units vs. Subsidies)", ("sans-serif", 25))
        .margin(20)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(min_x..max_x, min_y..max_y)?;
    chart.configure_mesh() // and here
        .x_desc("Total Units")
        .y_desc("Active Subsidies")
        .draw()?;

    // Find # of clusters 
    let &max_label = labels.iter().max().unwrap_or(&0);
    let k = max_label + 1;

    // Draw each cluster's points w/ a label for the legend
    for cluster_id in 0..k {
        let color = get_cluster_color(cluster_id);
        let pts: Vec<(f64, f64)> = properties.iter() // here too
            .zip(labels.iter())
            .filter(|(_p, &lbl)| lbl == cluster_id)
            .map(|(p, _)| (p.total_units as f64, p.subsidy_count as f64))
            .collect();
        chart.draw_series( // ""
            pts.iter()
               .map(|&(x, y)| Circle::new((x, y), 4, ShapeStyle::from(&color).filled()))
        )?
        .label(format!("Cluster {}", cluster_id))
        .legend(move |(x, y)| Circle::new((x, y), 6, ShapeStyle::from(&color).filled()));
    }

    // Compute centroids & draw 'X' markers
    let mut sums = vec![(0.0, 0.0); k];
    let mut counts = vec![0; k];
    for (p, &lbl) in properties.iter().zip(labels.iter()) {
        sums[lbl].0 += p.total_units as f64;
        sums[lbl].1 += p.subsidy_count as f64;
        counts[lbl] += 1;
    }
    for i in 0..k {
        let (sx, sy) = sums[i];
        let c = (sx / counts[i] as f64, sy / counts[i] as f64);
        let color = get_cluster_color(i);
        chart.draw_series(std::iter::once(
            Cross::new(c, 10, ShapeStyle::from(&color).stroke_width(2))
        ))?;
    }

    // Create the legend
    chart.configure_series_labels() // trying to minimize horizontal scrolling
        .background_style(WHITE.filled())
        .border_style(BLACK)
        .position(SeriesLabelPosition::UpperRight)
        .draw()?;

    // Save to output folder
    root.present()?;
    println!("Saved plot to {}", filename);
    Ok(())
}

fn min_max<I: Iterator<Item = f64>>(mut iter: I) -> (f64, f64) {
    let first = iter.next().unwrap_or(0.0);
    iter.fold((first, first), |(min, max), v| (min.min(v), max.max(v)))
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::data::HousingProperty;
	use std::fs;

	#[test]
	fn test_plot_clusters_runs() {
		// Check if output folder exists
		fs::create_dir_all("output").unwrap();

		// 2 sample points, 2 clusters
		let props = vec![
			HousingProperty { total_units: 10, subsidy_count: 1, owner_type: "X".into() },
			HousingProperty { total_units: 20, subsidy_count: 2, owner_type: "Y".into() },
		];
		let labels = vec![0, 1];

		// Should run w/o errors
		plot_clusters(&props, &labels).unwrap();
    }
}