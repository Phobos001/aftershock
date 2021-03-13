use crate::vector2::*;

/// Clamps a 32-bit float between the min and max range.
pub fn clampf(value: f32, min: f32, max: f32) -> f32 {
	if value < min { min } else if value > max { max } else { value }
}

/// Clamps a 32-bit integer between the min and max range.
pub fn clampi(value: i32, min: i32, max: i32) -> i32 {
	if value < min { min } else if value > max { max } else { value }
}

/// Returns the remainder of a division for 32-bit floats
pub fn modf(value: f32, rhs: f32) -> f32 {
	((value % rhs) + rhs) % rhs
}

/// Returns the remainder of a division for 32-bit integers
pub fn modi(value: i32, rhs: i32) -> i32 {
	((value % rhs) + rhs) % rhs
}

/// Returns a 32-bit float if the value is negative (-1.0), positive (1.0), or zero (0.0)
pub fn signf(value: f32) -> f32 {
	if value > 0.0 { 1.0 } else if value < 0.0 { -1.0 } else { 0.0 }
}

/// Returns a 32-bit integer if the value is negative (-1), positive (1), or zero (0)
pub fn signi(value: i32) -> i32 {
	if value > 0 { 1 } else if value < 0 { -1 } else { 0 }
}

/// Returns a linearly interpolated 32-bit float between a and b, using t as a percentage of... 'betweenness'?
pub fn lerpf(a: f32, b: f32, t: f32) -> f32 {
	a + (b - a) * t
}

/// Returns a linearly interpolated 32-bit integer between a and b, using t as a percentage of... 'betweenness'?
pub fn lerpi(a: i32, b: i32, t: f32) -> i32 {
	((a + (b - a)) as f32 * t).floor() as i32
}

/// Returns the dot product of two 2D vectors.
pub fn dot2(v1: Vec2, v2: Vec2) -> f32 {
	v1.x * v2.x + v1.y * v2.y
}

/// Returns the cross product of two 2D vectors.
pub fn cross2(v1: Vec2, v2: Vec2) -> f32 {
	(v1.x * v2.y) - (v1.y * v2.x)
}

/// Returns the 2D distance between two points.
pub fn dist2(v1: Vec2, v2: Vec2) -> f32{
	((v2.x - v1.x).powf(2.0) + (v2.y - v1.y).powf(2.0)).sqrt()
}

/// Returns the 3D distance between two points.
/// There is no Vec3 yet so it's broken down into floats.
pub fn dist3(x0: f32, y0: f32, z0: f32, x1: f32, y1: f32, z1: f32) -> f32{
	((x1 - x0).powf(2.0) + (y1 - y0).powf(2.0) + (z1 - z0).powf(2.0)).sqrt()
}

/// Okay I don't know if this is still useful but I was TRYING to make textured triangles.
/// This is basically used to a UV coordinate for textured triangles.
/// You'd loop over the triangle's UV's and use these values somehow to draw it correctly.
/// I don't think I need it though. If you guys can use it then by all means go ahead! 
pub fn barycentric2(v1: Vec2, v2: Vec2, v3: Vec2) -> (f32, f32, f32) {
	let b0 = Vec2::new(v2.x - v1.x, v2.y - v1.y);
	let b1 = Vec2::new(v3.x - v1.x, v3.y - v1.y);
	let b2 = Vec2::new(v1.x - v2.x, v1.y - v2.y);

    let d00 = dot2(b0, b0);
    let d01 = dot2(b0, b1);
    let d11 = dot2(b1, b1);
    let d20 = dot2(b2, b0);
    let d21 = dot2(b2, b1);
    let denom = d00 * d11 - d01 * d01;
   	let bv = (d11 * d20 - d01 * d21) / denom;
    let bw = (d00 * d21 - d01 * d20) / denom;
	let bu = 1.0 - bv - bw;
	
	(bu, bv, bw)
}

/// Returns a vector rotated in 2D space.
pub fn rotate2(x: f32, y: f32, a: f32) -> (f32, f32) {
	let (acos, asin) = (a.cos(), a.sin());
	let ax = (x * acos) - (y * asin);
	let ay = (x * asin) + (y * acos);
	return (ax, ay);
}