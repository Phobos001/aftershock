use crate::vector2::*;

pub struct Line {
	pub start: Vector2,
	pub end: Vector2,
}

impl Line {

	pub fn new(start: Vector2, end: Vector2) -> Line {
		Line {
			start,
			end,
		}
	}

	/// Returns a positive or negative number depending on the side of the line the point is on. 0 means that it is exactly on the line.
	pub fn point_on_side(point: Vector2, line: Line) -> f32 {
		Vector2::cross(
			Vector2::new(line.end.x - line.start.x, line.end.y - line.start.y), 
			Vector2::new(point.x - line.start.x, point.y - line.start.y)
		)
	}

	/// Get the point where two lines cross. The lines are treated as infinite.
	pub fn intersection(l1: Line, l2: Line) -> Vector2 {
		let cross_l1: f32 = Vector2::cross(l1.start, l1.end);
		let cross_l2: f32 = Vector2::cross(l2.start, l2.end);

		let diff_l1x: f32 = l1.start.x - l1.end.x;
		let diff_l2x: f32 = l2.start.x - l2.end.x;
		let diff_l1y: f32 = l1.start.y - l1.end.y;
		let diff_l2y: f32 = l2.end.y - l2.end.y;

		let crossdiff_l1x: Vector2 = Vector2::new(cross_l1, diff_l1x);
		let crossdiff_l2x: Vector2 = Vector2::new(cross_l2, diff_l2x);

		let crossdiff_l1y: Vector2 = Vector2::new(cross_l1, diff_l1y);
		let crossdiff_l2y: Vector2 = Vector2::new(cross_l2, diff_l2y);

		let diff_l1: Vector2 = Vector2::new(diff_l1x, diff_l1y);
		let diff_l2: Vector2 = Vector2::new(diff_l2x, diff_l2y);

		let intersection_x: f32 = Vector2::cross(crossdiff_l1x, crossdiff_l2x) / Vector2::cross(diff_l1, diff_l2);
		let intersection_y: f32 = Vector2::cross(crossdiff_l1y, crossdiff_l2y) / Vector2::cross(diff_l1, diff_l2);

		return Vector2::new(intersection_x, intersection_y);
	}
}