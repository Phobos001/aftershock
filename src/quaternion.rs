use crate::matrix4::*;

#[derive(Debug, Copy, Clone)]
pub struct Quaternion {
	pub x: f32,
	pub y: f32,
	pub z: f32,
	pub w: f32,
}

impl Quaternion {
	pub fn identity() -> Quaternion {
		Quaternion {
			w: 1.0,
			x: 0.0,
			y: 0.0,
			z: 0.0
		}
	}

	pub fn normalize(&mut self) {
		let magnitude = ((self.w * self.w) + (self.x * self.x) + (self.y * self.y) + (self.z * self.z)).sqrt();

		self.w /= magnitude;
		self.x /= magnitude;
		self.y /= magnitude;
		self.z /= magnitude;
	}

	pub fn normalized(&self) -> Quaternion {
		let mut quat: Quaternion = self.clone();
		quat.normalize();
		quat
	}

	pub fn to_matrix(&self) -> Matrix4 {
		let mut mtx: Matrix4 = Matrix4::identity();

		// short hand
		let w = self.w;
		let x = self.x;
		let y = self.y;
		let z = self.z;

		let w2 = self.w * self.w;
		let x2 = self.x * self.x;
		let y2 = self.y * self.y;
		let z2 = self.z * self.z;

		// HELP
		mtx.m = [
			[1.0 - 2.0 * y2, 					2.0 * x * y - 2.0 * w * z, 		2.0 * x * z + 2.0 * w * y, 		0.0],

			[2.0 * x * y + 2.0 * w * z, 		1.0 - 2.0 * x2 - 2.0 * z2, 		2.0 * y * z + 2.0 * w * x, 		0.0],

			[2.0 * x * z - 2.0 * w * y, 		2.0 * y * z - 2.0 * w * x, 		1.0 - 2.0 * x2 - 2.0 * y2, 				0.0],

			[0.0, 								0.0, 							0.0, 							1.0],
		];

		mtx
	}
}

impl std::ops::Mul for Quaternion {
	type Output = Self;

	fn mul(self, rhs: Self) -> Self {
		Self {
			w: (self.w * rhs.w) - (self.x * rhs.x) - (self.y * rhs.y) - (self.z * rhs.z),
			x: (self.w * rhs.x) + (self.x * rhs.w) + (self.y * rhs.z) - (self.z * rhs.y),
			y: (self.w * rhs.y) - (self.x * rhs.z) + (self.y * rhs.w) + (self.z * rhs.x),
			z: (self.w * rhs.z) + (self.x * rhs.y) - (self.y * rhs.x) + (self.z * rhs.w)
		}
	}
}

impl std::ops::MulAssign for Quaternion {
	fn mul_assign(&mut self, rhs: Self) {
        *self = Self {
            w: (self.w * rhs.w) - (self.x * rhs.x) - (self.y * rhs.y) - (self.z * rhs.z),
			x: (self.w * rhs.x) + (self.x * rhs.w) + (self.y * rhs.z) - (self.z * rhs.y),
			y: (self.w * rhs.y) - (self.x * rhs.z) + (self.y * rhs.w) + (self.z * rhs.x),
			z: (self.w * rhs.z) + (self.x * rhs.y) - (self.y * rhs.x) + (self.z * rhs.w)
        };
    }
}