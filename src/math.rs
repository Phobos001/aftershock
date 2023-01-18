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