use crate::{math::dist2, vector2::*};
use crate::matrix3::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Line {
	pub start: Vector2,
	pub end: Vector2,
	pub dirright: Vector2,
	pub dirleft: Vector2,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum LineSide {
	Right,
	Left,
	Overlap,
}

impl Line {

	pub fn new(start: Vector2, end: Vector2) -> Line {

		let rotrad = (90.0f32).to_radians();

		let direction = (end - start).normalized();
		let dirright = direction.rotated(rotrad);
		let dirleft = direction.rotated(-rotrad);

		Line {
			start,
			end,
			dirright,
			dirleft,
		}
	}

	/// Returns a positive or negative number depending on the side of the line the point is on. 0 means that it is exactly on the line.
	/// From Start to End, positive numbers mean right side, and negative numbers mean left
	pub fn point_on_side(point: Vector2, line: &Line) -> LineSide {
		let sideraw: f32 = Vector2::cross(
			Vector2::new(line.end.x - line.start.x, line.end.y - line.start.y), 
			Vector2::new(point.x - line.start.x, point.y - line.start.y)
		);

		if sideraw > 0.0 { LineSide::Right } else if sideraw < 0.0 { LineSide::Left } else { LineSide::Overlap }
	}

	/// Get the point where two lines cross. The lines are treated as infinite.
	pub fn intersection_point(l1: &Line, l2: &Line) -> Vector2 {
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

	pub fn intersection_test(l1: &Line, l2: &Line) -> bool {
		// We only care about the general cases. We won't have many instances with segments on points
		let d1 = Vector2::orientation(l1.start, l2.start, l1.end);
		let d2 = Vector2::orientation(l1.start, l2.start, l2.end);
		let d3 = Vector2::orientation(l1.end, l2.end, l1.start);
		let d4 = Vector2::orientation(l1.end, l2.end, l2.start);

		d1 != d2 && d3 != d4
	}

	pub fn projected(&self, mtx: Matrix3) -> Line {
		Line::new(
			mtx.forward(self.start),
			mtx.forward(self.end),
		)
	}
}