use crate::{quaternion::Quaternion, vector3::*};
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

	pub fn translated(translate: Vector3) -> Matrix4 {
		let mut nmtx: Matrix4 = Matrix4::identity();
		nmtx.m[3][0] = translate.x;
		nmtx.m[3][1] = translate.y;
		nmtx.m[3][2] = translate.z;
		nmtx
	}

	pub fn scaled(scale: Vector3) -> Matrix4 {
		let mut nmtx: Matrix4 = Matrix4::identity();
		nmtx.m[0][0] = scale.x;
		nmtx.m[1][1] = scale.y;
		nmtx.m[2][2] = scale.z;
		nmtx
	}

	pub fn rotated(yaw: f32, pitch: f32, roll: f32) -> Matrix4 {
		Matrix4::rotated_yaw(yaw) * Matrix4::rotated_pitch(pitch) *  Matrix4::rotated_roll(roll)
	}

	pub fn rotated_roll(theta: f32) -> Matrix4 {
		let sin = theta.sin();
		let cos = theta.cos();

		let mut nmtx: Matrix4 = Matrix4::identity();
		nmtx.m[1][2] = cos;
		nmtx.m[1][3] = -sin;
		nmtx.m[2][2] = sin;
		nmtx.m[2][3] = cos;
		nmtx
	}

	pub fn rotated_pitch(theta: f32) -> Matrix4 {
		let sin = theta.sin();
		let cos = theta.cos();

		let mut nmtx: Matrix4 = Matrix4::identity();
		nmtx.m[0][0] = cos;
		nmtx.m[0][2] = sin;
		nmtx.m[2][0] = -sin;
		nmtx.m[2][2] = cos;
		nmtx
	}

	pub fn rotated_yaw(theta: f32) -> Matrix4 {
		let sin = theta.sin();
		let cos = theta.cos();

		let mut nmtx: Matrix4 = Matrix4::identity();
		nmtx.m[0][0] = cos;
		nmtx.m[0][1] = -sin;
		nmtx.m[1][0] = sin;
		nmtx.m[1][1] = cos;
		nmtx
	}

	pub fn transform_triangle(&self, triangle: Triangle) -> Triangle {
		Triangle {
			v1: self.forward(triangle.v1),
			v2: self.forward(triangle.v2),
			v3: self.forward(triangle.v3),
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
		let mut fmtx = Matrix4::identity();
		for c in 0..4 {
			for r in 0..4 {
				fmtx.m[c][r] =  self.m[0][r] * rhs.m[c][0] +
								self.m[1][r] * rhs.m[c][1] +
								self.m[2][r] * rhs.m[c][2] +
								self.m[3][r] * rhs.m[c][3];
			}
		}
		fmtx
	}
}