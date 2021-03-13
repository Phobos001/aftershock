use crate::vector3::*;
use crate::three::*;

// Matrix 4x4 for 3D transformations
#[derive(Copy, Clone, Debug)]
pub struct Matrix4 {
	pub m: [[f32; 4]; 4],
}

impl Matrix4 {
	/// Returns an identity matrix. It's basically like a normalized vector, where the 'magnitudes' of each value are 1.0.
	/// This is usually the foundation of all matrix transformations.
	pub fn identity() -> Matrix4 {
		Matrix4 {
			m: [
				[1.0, 0.0, 0.0, 0.0],
				[0.0, 1.0, 0.0, 0.0],
				[0.0, 0.0, 1.0, 0.0],
				[0.0, 0.0, 0.0, 1.0]
			]
		}
	}

	pub fn forward(&self, vi: Vector3) -> Vector3 {
		let mut vo = vi;

		vo.x = vi.x * self.m[0][0] + vi.y * self.m[1][0] + vi.z * self.m[2][0] + self.m[3][0];
		vo.y = vi.x * self.m[0][1] + vi.y * self.m[1][1] + vi.z * self.m[2][1] + self.m[3][1];
		vo.z = vi.x * self.m[0][2] + vi.y * self.m[1][2] + vi.z * self.m[2][2] + self.m[3][2];

		let w: f32 = vi.x * self.m[0][3] + vi.y * self.m[1][3] + vi.z * self.m[2][3] + self.m[3][3];

		if w != 0.0 {
			vo.x /= w; vo.y /= w; vo.z /= w;
		}

		vo
	}
}

// Matrix3 Operator Assignments
impl std::ops::Mul for Matrix4 {
	type Output = Self;

	fn mul(self, rhs: Matrix4) -> Self::Output {
		let mut fmtx: [[f32; 4]; 4] = [[0.0 ;4] ;4];
		for c in 0..4 {
			for r in 0..4 {
				fmtx[c][r] =  self.m[0][r] * rhs.m[c][0] +
								self.m[1][r] * rhs.m[c][1] +
								self.m[2][r] * rhs.m[c][2] +
								self.m[3][r] * rhs.m[c][3];
			}
		}
		return Matrix4 { m: fmtx };
	}
}