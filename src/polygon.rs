use crate::{vector2::*, line::*};

pub struct Polygon {
	pub edges: Vec<Line>,
}

impl Polygon {
	pub fn new_empty() -> Polygon {
		Polygon {
			edges: Vec::new(),
		}
	}

	pub fn new_edges(edges: Vec<Line>) -> Polygon {
		Polygon { 
			edges,
		}
	}

	pub fn new_points(points: Vec<Vector2>) -> Polygon {
		let mut edges: Vec<Line> = Vec::new();

		for i in 0..points.len() {
			let idx = i % points.len();
			let line = Line::new(points[i], points[(i + 1) % points.len()]);
		}

		Polygon::new_edges(edges)
	}

	pub fn point_inside(&self, point: Vector2) -> bool {
		// Normally we would have to cast a ray leftward, but we can check if the points Y is between the lines, and that at least one X is left of the point.
		let mut edge_count: u32 = 0;
		for edge in &self.edges {
			let line_crosses_left: bool = edge.start.x < point.x || edge.end.x < point.x;
			let edge_in_y_range: bool = (edge.start.y < point.y && edge.end.y > point.y) || (edge.end.y < point.y && edge.start.y > point.y);
			if line_crosses_left && edge_in_y_range {
				edge_count += 1;
			}
		}

		// Even means its inside, odd means its outside
		if edge_count % 2 == 0 {
			return true;
		} else {
			return false;
		}
	}
}