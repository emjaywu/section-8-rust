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