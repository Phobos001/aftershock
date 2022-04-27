/// Clamps a 32-bit float between the min and max range.
pub fn clampf(value: f64, min: f64, max: f64) -> f64 {
	if value < min { min } else if value > max { max } else { value }
}

/// Clamps a 32-bit integer between the min and max range.
pub fn clampi(value: i64, min: i64, max: i64) -> i64 {
	if value < min { min } else if value > max { max } else { value }
}

/// Returns the remainder of a division for 32-bit floats
pub fn modf(value: f64, rhs: f64) -> f64 {
	//((value % rhs) + rhs) % rhs
	value.rem_euclid(rhs)
}

/// Returns the remainder of a division for 32-bit integers
pub fn modi(value: i64, rhs: i64) -> i64 {
	//((value % rhs) + rhs) % rhs
	value.rem_euclid(rhs)
}

/// Returns a 32-bit float if the value is negative (-1.0), positive (1.0), or zero (0.0)
pub fn signf(value: f64) -> f64 {
	if value > 0.0 { 1.0 } else if value < 0.0 { -1.0 } else { 0.0 }
}

/// Returns a 32-bit integer if the value is negative (-1), positive (1), or zero (0)
pub fn signi(value: i64) -> i64 {
	if value > 0 { 1 } else if value < 0 { -1 } else { 0 }
}

/// Returns a linearly interpolated 32-bit float between a and b, using t as a percentage of... 'betweenness'?
pub fn lerpf(a: f64, b: f64, t: f64) -> f64 {
	a + (b - a) * t
}

/// Returns a linearly interpolated 32-bit integer between a and b, using t as a percentage of... 'betweenness'?
pub fn lerpi(a: i64, b: i64, t: f64) -> i64 {
	((a + (b - a)) as f64 * t).floor() as i64
}

pub fn unlerpf(value: f64, min: f64, max: f64) -> f64 {
	(value - min) / (max - min)
}

/// Returns a vector rotated in 2D space.
pub fn rotate2(x: f64, y: f64, a: f64) -> (f64, f64) {
	let (acos, asin) = (a.cos(), a.sin());
	let ax = (x * acos) - (y * asin);
	let ay = (x * asin) + (y * acos);
	return (ax, ay);
}

pub fn dot2(x0: f64, y0: f64, x1: f64, y1: f64) -> f64 {
	x0 * x1 + y0 * y1
}

pub fn cross2(x0: f64, y0: f64, x1: f64, y1: f64) -> f64 {
	(x0 * y1) - (y0 * x1)
}

pub fn barycentric2(v1x: f64, v1y: f64, v2x: f64, v2y: f64, v3x: f64, v3y: f64) -> (f64, f64, f64) {
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