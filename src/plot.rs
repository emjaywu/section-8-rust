use std::path::Path;
use plotters::prelude::*;
use crate::data::HousingProperty;
use crate::utils::get_cluster_color;

/// Plot a scatterplot of latitude & longitude by cluster ID
pub fn plot_clusters(properties: &[HousingProperty], labels: &[usize]) -> Result<(), Box<dyn std::error::Error>> {
	let mut idx = 1;
	let filename = loop {
		let candidate = format!("output/clusters_{}.png", idx); // preventing overwrite when producing images per iteration
		if !Path::new(&candidate).exists() {
			break candidate;
		}
		idx += 1;
	};

	let root = BitMapBackend::new(&filename, (800, 600)).into_drawing_area();
	root.fill(&WHITE)?;

	let (min_x, max_x) = min_max(properties.iter().map(|p| p.total_units as f64));
	let (min_y, max_y) = min_max(properties.iter().map(|p| p.subsidy_count as f64));

	let mut chart = ChartBuilder::on(&root)
		.caption("Clustered Housing Properties (Units vs. Subsidies)", ("sans-serif", 25))
		.margin(20)
		.x_label_area_size(40)
		.y_label_area_size(40)
		.build_cartesian_2d(min_x..max_x, min_y..max_y)?;
	chart.configure_mesh()
		.x_desc("Total Units")
		.y_desc("Active Subsidies")
		.draw()?;

	for (prop, &label) in properties.iter().zip(labels.iter()) {
		let color = get_cluster_color(label);
		chart.draw_series(PointSeries::of_element(
			vec![(prop.total_units as f64, prop.subsidy_count as f64)], 
			3, 
			ShapeStyle::from(&color).filled(), 
			&|c, s, st| EmptyElement::at(c) + Circle::new((0, 0), s, st),
		))?;
	}

	root.present()?;
	println!("Saved plot to {}", filename);
	Ok(())
}

fn min_max<I: Iterator<Item = f64>>(mut values: I) -> (f64, f64) {
	let first = values.next().unwrap_or(0.0);
	values.fold((first, first), |(min, max), v| (v.min(min), v.max(max)))
}