use crate::vectors::*;

/// 3x3 Matrix mainly for transforming 2D images, but can be used for anything.
#[derive(Copy, Clone, Debug)]
pub struct Mat3 {
	m: [[f32; 3]; 3],
}

impl Mat3 {

	/// Returns an identity matrix. It's basically like a normalized vector, where the 'magnitudes' of each value are 1.0.
	/// This is usually the foundation of all matrix transformations.
	pub fn identity() -> Mat3 {
		Mat3 {
			m: [
				[1.0, 0.0, 0.0],
				[0.0, 1.0, 0.0],
				[0.0, 0.0, 1.0]
			]
		}
	}

	/// Receives a 2D Vector and returns a transformed copy of it.
	pub fn forward(&self, in_vec: Vec2) -> Vec2 {
		let mut out = Vec2::new(0.0, 0.0);
		out.x = in_vec.x * self.m[0][0] + in_vec.y * self.m[1][0] + self.m[2][0];
		out.y = in_vec.x * self.m[0][1] + in_vec.y * self.m[1][1] + self.m[2][1];
		return out;
	}

	/// Creates a translated 3x3 Matrix
	pub fn translated(v: Vec2) -> Mat3{
		let mut nmtx = Mat3::identity();
		nmtx.m[2][0] = v.x;
		nmtx.m[2][1] = v.y;
		return nmtx;
	}

	/// Creates a rotated 3x3 Matrix
	pub fn rotated(radians: f32) -> Mat3 {
		let mut nmtx = Mat3::identity();
		nmtx.m[0][0] = radians.cos(); nmtx.m[1][0] = radians.sin();
		nmtx.m[0][1] = -(radians.sin()); nmtx.m[1][1] = radians.cos();
		return nmtx;
	}

	/// Creates a scaled 3x3 Matrix
	pub fn scaled(v: Vec2) -> Mat3 {
		let mut nmtx = Mat3::identity();
		nmtx.m[0][0] = v.x;
		nmtx.m[1][1] = v.y;
		return nmtx;
	}

	/// Creates a sheared 3x3 Matrix
	pub fn sheared(v: Vec2) -> Mat3 {
		let mut nmtx = Mat3::identity();
		nmtx.m[0][1] = v.x;
		nmtx.m[1][0] = v.y;
		return nmtx;
	}

	/// Creates an inverse of this Matrix, usually for getting correct pixel information when drawing 2D Images.
	pub fn inv(&self) -> Mat3 {
		let mut out = Mat3::identity();
		let det: f32 = 
			self.m[0][0] * (self.m[1][1] * self.m[2][2] - self.m[1][2] * self.m[2][1]) -
			self.m[1][0] * (self.m[0][1] * self.m[2][2] - self.m[2][1] * self.m[0][2]) +
			self.m[2][0] * (self.m[0][1] * self.m[1][2] - self.m[1][1] * self.m[0][2]);

		let idet: f32 = 1.0 / det;

		out.m[0][0] = (self.m[1][1] * self.m[2][2] - self.m[1][2] * self.m[2][1]) * idet;
		out.m[1][0] = (self.m[2][0] * self.m[1][2] - self.m[1][0] * self.m[2][2]) * idet;
		out.m[2][0] = (self.m[1][0] * self.m[2][1] - self.m[2][0] * self.m[1][1]) * idet;
		out.m[0][1] = (self.m[2][1] * self.m[0][2] - self.m[0][1] * self.m[2][2]) * idet;
		out.m[1][1] = (self.m[0][0] * self.m[2][2] - self.m[2][0] * self.m[0][2]) * idet;
		out.m[2][1] = (self.m[0][1] * self.m[2][0] - self.m[0][0] * self.m[2][1]) * idet;
		out.m[0][2] = (self.m[0][1] * self.m[1][2] - self.m[0][2] * self.m[1][1]) * idet;
		out.m[1][2] = (self.m[0][2] * self.m[1][0] - self.m[0][0] * self.m[1][2]) * idet;
		out.m[2][2] = (self.m[0][0] * self.m[1][1] - self.m[0][1] * self.m[1][0]) * idet;
		return out;
	}
}


// Mat3 Operator Assignments
impl std::ops::Mul for Mat3 {
	type Output = Self;

	fn mul(self, rhs: Mat3) -> Self::Output {
		let mut fmtx: [[f32; 3]; 3] = [[0.0 ;3] ;3];
		for c in 0..3 {
			for r in 0..3 {
				fmtx[c][r] =  self.m[0][r] * rhs.m[c][0] +
								self.m[1][r] * rhs.m[c][1] +
								self.m[2][r] * rhs.m[c][2];
			}
		}
		return Mat3 { m: fmtx };
	}
}