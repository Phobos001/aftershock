pub fn clampf(value: f32, min: f32, max: f32) -> f32 {
	if value < min { min } else if value > max { max } else { value }
}

pub fn clampi(value: i32, min: i32, max: i32) -> i32 {
	if value < min { min } else if value > max { max } else { value }
}

pub fn signf(value: f32) -> f32 {
	if value > 0.0 { 1.0 } else if value < 0.0 { -1.0 } else { 0.0 }
}

pub fn signi(value: i32) -> i32 {
	if value > 0 { 1 } else if value < 0 { -1 } else { 0 }
}

pub fn lerpf(a: f32, b: f32, t: f32) -> f32 {
	a + (b - a) * t
}

pub fn lerpi(a: i32, b: i32, t: f32) -> i32 {
	((a + (b - a)) as f32 * t).floor() as i32
}

pub fn dot2(x0: f32, y0: f32, x1: f32, y1: f32) -> f32 {
	x0 * x1 + y0 * y1
}

pub fn cross2(x0: f32, y0: f32, x1: f32, y1: f32) -> f32 {
	(x0 * y1) - (y0 * x1)
}

pub fn dist2(x0: f32, y0: f32, x1: f32, y1: f32) -> f32{
	((x1 - x0).powf(2.0) + (y1 - y0).powf(2.0)).sqrt()
}

pub fn dist3(x0: f32, y0: f32, z0: f32, x1: f32, y1: f32, z1: f32) -> f32{
	((x1 - x0).powf(2.0) + (y1 - y0).powf(2.0) + (z1 - z0).powf(2.0)).sqrt()
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

pub fn rotate2(x: f32, y: f32, a: f32) -> (f32, f32) {
	let (acos, asin) = (a.cos(), a.sin());
	let ax = (x * acos) - (y * asin);
	let ay = (x * asin) + (y * acos);
	return (ax, ay);
}