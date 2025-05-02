use plotters::style::RGBColor;

/// Assign colors to clusters
pub fn get_cluster_color(cluster_id: usize) -> RGBColor {
	match cluster_id {
		0 => RGBColor(228, 26, 28), // red
		1 => RGBColor(55, 126, 184), // blue
		2 => RGBColor(77, 175, 74), // green
		3 => RGBColor(152, 78, 163), // purple
		_ => RGBColor(0, 0, 0), // black
	} 
}

/// Normalize a numeric slice to [0.0, 1.0]
pub fn normalize(values: &[f64]) -> Vec<f64> {
	let min = values.iter().cloned().fold(f64::INFINITY, f64::min);
	let max = values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

	if (max - min).abs() < std::f64::EPSILON {
		return values.iter().map(|_| 0.5).collect();
	}

	values.iter().map(|v| (v - min) / (max - min)).collect()
}