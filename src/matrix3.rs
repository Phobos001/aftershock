use crate::vector2::*;

/// 3x3 Matrix mainly for transforming 2D images, but can be used for anything.
#[derive(Copy, Clone, Debug)]
pub struct Matrix3 {
	m: [[f64; 3]; 3],
}

impl Matrix3 {

	/// Returns an identity matrix. It's basically like a normalized vector, where the 'magnitudes' of each value are 1.0.
	/// This is usually the foundation of all matrix transformations.
	pub fn identity() -> Matrix3 {
		Matrix3 {
			m: [
				[1.0, 0.0, 0.0],
				[0.0, 1.0, 0.0],
				[0.0, 0.0, 1.0]
			]
		}
	}

	/// Receives a 2D Vector and returns a transformed copy of it.
	pub fn forward(&self, in_vec: Vector2) -> Vector2 {
		let mut out = Vector2::new(0.0, 0.0);
		out.x = in_vec.x * self.m[0][0] + in_vec.y * self.m[1][0] + self.m[2][0];
		out.y = in_vec.x * self.m[0][1] + in_vec.y * self.m[1][1] + self.m[2][1];
		return out;
	}

	/// Creates a translated 3x3 Matrix
	pub fn translated(v: Vector2) -> Matrix3{
		let mut nmtx = Matrix3::identity();
		nmtx.m[2][0] = v.x;
		nmtx.m[2][1] = v.y;
		return nmtx;
	}

	/// Creates a rotated 3x3 Matrix
	pub fn rotated(radians: f64) -> Matrix3 {
		let mut nmtx = Matrix3::identity();
		nmtx.m[0][0] = radians.cos(); nmtx.m[1][0] = radians.sin();
		nmtx.m[0][1] = -(radians.sin()); nmtx.m[1][1] = radians.cos();
		return nmtx;
	}

	/// Creates a scaled 3x3 Matrix
	pub fn scaled(v: Vector2) -> Matrix3 {
		let mut nmtx = Matrix3::identity();
		nmtx.m[0][0] = v.x;
		nmtx.m[1][1] = v.y;
		return nmtx;
	}

	/// Creates a sheared 3x3 Matrix
	pub fn sheared(v: Vector2) -> Matrix3 {
		let mut nmtx = Matrix3::identity();
		nmtx.m[0][1] = v.x;
		nmtx.m[1][0] = v.y;
		return nmtx;
	}

	/// Creates an inverse of this Matrix, usually for getting correct pixel information when drawing 2D Images.
	pub fn inv(&self) -> Matrix3 {
		let mut out = Matrix3::identity();
		let det: f64 = 
			self.m[0][0] * (self.m[1][1] * self.m[2][2] - self.m[1][2] * self.m[2][1]) -
			self.m[1][0] * (self.m[0][1] * self.m[2][2] - self.m[2][1] * self.m[0][2]) +
			self.m[2][0] * (self.m[0][1] * self.m[1][2] - self.m[1][1] * self.m[0][2]);

		let idet: f64 = 1.0 / det;

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


// Matrix3 Operator Assignments
impl std::ops::Mul for Matrix3 {
	type Output = Self;

	fn mul(self, rhs: Matrix3) -> Self::Output {
		let mut fmtx: [[f64; 3]; 3] = [[0.0 ;3] ;3];
		for c in 0..3 {
			for r in 0..3 {
				fmtx[c][r] =  self.m[0][r] * rhs.m[c][0] +
								self.m[1][r] * rhs.m[c][1] +
								self.m[2][r] * rhs.m[c][2];
			}
		}
		return Matrix3 { m: fmtx };
	}
}