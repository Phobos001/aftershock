use glam::Vec3A;

use crate::glam::Vec2;

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

pub fn point_in_triangle(px: i32, py: i32, x0: i32, y0: i32, x1: i32, y1: i32, x2: i32, y2: i32) -> bool {
	let d1 = sign3i(px, py, x0, y0, x1, y1);
	let d2 = sign3i(px, py, x1, y1, x2, y2);
	let d3 = sign3i(px, py, x2, y2, x0, y0);

	let has_neg = (d1 < 0) || (d2 < 0) || (d3 < 0);
	let has_pos = (d1 > 0) || (d2 > 0) || (d3 > 0);

	let is_inside: bool = !(has_neg && has_pos);
	is_inside
}

pub fn barycentric(p: (f32, f32), a: (f32, f32), b: (f32, f32), c: (f32, f32)) -> (f32, f32, f32) {
    let v0 = (b.0 - a.0, b.1 - a.1);
	let v1 = (c.0 - a.0, c.1 - a.1);
	let v2 = (p.0 - a.0, p.1 - a.1);

    let d00 = dot2(v0.0, v0.1, v0.0, v0.1);
    let d01 = dot2(v0.0, v0.1, v1.0, v1.1);
    let d11 = dot2(v1.0, v1.1, v1.0, v1.1);
    let d20 = dot2(v2.0, v2.1, v0.0, v0.1);
    let d21 = dot2(v2.0, v2.1, v1.0, v1.1);
    let denom = d00 * d11 - d01 * d01;
    let v = (d11 * d20 - d01 * d21) / denom;
    let w = (d00 * d21 - d01 * d20) / denom;
    let u = 1.0 - v - w;

	(u, v, w)
}