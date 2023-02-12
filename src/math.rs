use crate::vector2::Vector2;

/// Returns a 32-bit float if the value is negative (-1.0), positive (1.0), or zero (0.0)
pub fn signf(value: f32) -> f32 {
	if value > 0.0 { 1.0 } else if value < 0.0 { -1.0 } else { 0.0 }
}

/// Returns a 32-bit integer if the value is negative (-1), positive (1), or zero (0)
pub fn signi(value: i32) -> i32 {
	if value > 0 { 1 } else if value < 0 { -1 } else { 0 }
}

pub fn sign3i (p1x: i32, p1y: i32, p2x: i32, p2y: i32, p3x: i32, p3y: i32) -> i32
{
    (p1x - p3x) * (p2y - p3y) - (p2x - p3x) * (p1y - p3y)
}

pub fn sign3f (p1x: f32, p1y: f32, p2x: f32, p2y: f32, p3x: f32, p3y: f32) -> f32
{
    (p1x - p3x) * (p2y - p3y) - (p2x - p3x) * (p1y - p3y)
}

/// Returns a linearly interpolated 32-bit float between a and b, using t as a percentage of... 'betweenness'?
pub fn lerpf(a: f32, b: f32, t: f32) -> f32 {
	a + (b - a) * t
}

/// Returns a linearly interpolated 32-bit integer between a and b, using t as a percentage of... 'betweenness'?
pub fn lerpi(a: i32, b: i32, t: f32) -> i32 {
	((a + (b - a)) as f32 * t).floor() as i32
}

pub fn unlerpf(value: f32, min: f32, max: f32) -> f32 {
	(value - min) / (max - min)
}

/// Returns a vector rotated in 2D space.
pub fn rotate2(x: f32, y: f32, a: f32) -> (f32, f32) {
	let (acos, asin) = (a.cos(), a.sin());
	let ax = (x * acos) - (y * asin);
	let ay = (x * asin) + (y * acos);
	return (ax, ay);
}

pub fn dot2(x0: f32, y0: f32, x1: f32, y1: f32) -> f32 {
	x0 * x1 + y0 * y1
}

pub fn cross2(x0: f32, y0: f32, x1: f32, y1: f32) -> f32 {
	(x0 * y1) - (y0 * x1)
}

pub fn determinant(a: f32, b: f32, c: f32, d: f32) -> f32 {
	a * d - b * c
}

pub fn mapi(value: i32, low1: i32, high1: i32, low2: i32, high2: i32) -> i32 {
	low2 + (value - low1) * (high2 - low2) / (high1 - low1)
}

pub fn mapf(value: f32, low1: f32, high1: f32, low2: f32, high2: f32) -> f32 {
	low2 + (value - low1) * (high2 - low2) / (high1 - low1)
}

pub fn intersection(p1_start: Vector2, p1_end: Vector2, p2_start: Vector2, p2_end: Vector2) -> Option<Vector2> {
	let a1: f32 = p1_end.y - p1_start.y;
	let b1: f32 = p1_start.x - p1_end.x;
	let c1: f32 = a1 * p1_start.x + b1 * p1_start.y;

	let a2: f32 = p2_end.y - p2_start.y;
	let b2: f32 = p2_start.x - p2_end.x;
	let c2: f32 = a2 * p2_start.x + b2 * p2_start.y;

	let determinant = a1 * b2 - a2 * b1;

	if determinant == 0.0 { // No intersection: Lines are parallel
		None
	} else {
		let point = Vector2::new(
			(b2 * c1 - b1 * c2) / determinant,
			(a1 * c2 - a2 * c1) / determinant
		);
		Some(point)
	}
}

pub fn intersection_segment(ray_start: Vector2, ray_end: Vector2, line_start: Vector2, line_end: Vector2) -> Option<Vector2> {

	// Check for any intersection at all
	let intersection_opt = intersection(ray_start, ray_end, line_start, line_end);

	if intersection_opt.is_some() {
		let point = intersection_opt.unwrap();

		// First make sure the point is along the segment; between the start and end
		let error_margin: f32 = 0.00001;

		let dist_line = Vector2::distance(line_start, line_end);
		let dist_point_to_line_start = Vector2::distance(line_start, point);
		let dist_point_to_line_end = Vector2::distance(line_end, point);			

		let diff = dist_line - (dist_point_to_line_start + dist_point_to_line_end);
		let is_on_segment = diff < error_margin && diff > -error_margin;

		// Now make sure the intersected line is actually in front of the ray direction
		// Otherwise intersections can occur on lines behind the ray and be incorrect

		let is_in_ray_direction = Vector2::dot(
			Vector2::direction(ray_start, ray_end), 
			Vector2::direction(ray_start, point)
		) > 0.0;

		// Finally make sure the point is in the distance of the rays magnitude
		let is_in_distance = Vector2::distance(ray_start, point) < Vector2::distance(ray_start, ray_end);

		if is_in_ray_direction && is_on_segment && is_in_distance {
			Some(point)
		} else {
			None
		}
	} else {
		None
	}
}

pub fn barycentric2(v1x: f32, v1y: f32, v2x: f32, v2y: f32, v3x: f32, v3y: f32) -> (f32, f32, f32) {
	let b0 = (v2x - v1x, v2y - v1y);
	let b1 = (v3x - v1x, v3y - v1y);
	let b2 = (v1x - v2x, v1y - v2y);

    let d00 = dot2(b0.0, b0.1, b0.0, b0.1);
    let d01 = dot2(b0.0, b0.1, b1.0, b1.1);
    let d11 = dot2(b1.0, b1.1, b1.0, b1.1);
    let d20 = dot2(b2.0, b2.1, b0.0, b0.1);
    let d21 = dot2(b2.0, b2.1, b1.0, b1.1);
    let denom = d00 * d11 - d01 * d01;
   	let bv = (d11 * d20 - d01 * d21) / denom;
    let bw = (d00 * d21 - d01 * d20) / denom;
	let bu = 1.0 - bv - bw;
	
	(bu, bv, bw)
}

pub fn barycentric(p: Vector2, a: Vector2, b: Vector2, c: Vector2) -> (f32, f32, f32) {
    let v0 = b - a;
	let v1 = c - a;
	let v2 = p - a;

    let d00 = Vector2::dot(v0, v0);
    let d01 = Vector2::dot(v0, v1);
    let d11 = Vector2::dot(v1, v1);
    let d20 = Vector2::dot(v2, v0);
    let d21 = Vector2::dot(v2, v1);
    let denom = d00 * d11 - d01 * d01;
    let v = (d11 * d20 - d01 * d21) / denom;
    let w = (d00 * d21 - d01 * d20) / denom;
    let u = 1.0 - v - w;

	(u, v, w)
}