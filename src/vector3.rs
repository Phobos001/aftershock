use crate::math::*;

#[derive(Debug, Copy, Clone)]
pub struct Vector3 {
	pub x: f32, pub y: f32, pub z: f32,
}

impl Vector3 {
	pub fn new(x: f32, y: f32, z: f32) -> Vector3 {
		Vector3 {
			x, y, z,
		}
	}

	pub fn zero() -> Vector3 { Vector3 { x: 0.0, y: 0.0, z: 0.0} }
	pub fn one() -> Vector3 { Vector3 { x: 1.0, y: 1.0, z: 1.0} }
	pub fn up() -> Vector3 { Vector3 { x: 0.0, y: 1.0, z: 0.0} }
	pub fn down() -> Vector3 { Vector3 { x: 0.0, y: -1.0, z: 0.0} }
	pub fn left() -> Vector3 { Vector3 { x: -1.0, y: 0.0, z: 0.0} }
	pub fn right() -> Vector3 { Vector3 {x: 1.0, y: 0.0, z: 0.0} }
	pub fn forward() -> Vector3 { Vector3 {x: 0.0, y: 0.0, z: 1.0} }
	pub fn backward() -> Vector3 { Vector3 {x: 0.0, y: 0.0, z: -1.0}}

	/// Returns the squared magnitude of the vector.
	pub fn magnitude_sqr(&self) -> f32 {
		(self.x * self.x) + (self.y * self.y) + (self.z * self.z)
	}

	/// Returns the real magnitude of the vector.
	pub fn magnitude(&self) -> f32 {
		((self.x * self.x) + (self.y * self.y) + (self.z * self.z)).sqrt()
	}

	/// Sets the vectors magintude to 1.0 while retaining its direction.
	pub fn normalize(&mut self) {
		let mut magnitude = self.magnitude_sqr();
		if magnitude > 0.0 {
			magnitude = magnitude.sqrt();
			let length_inv = 1.0 / magnitude;
			self.x *= length_inv;
			self.y *= length_inv;
			self.z *= length_inv;
		} else {
			self.x = 0.0;
			self.y = 0.0;
			self.z = 0.0;
		}
	}

	/// Returns a normalized copy of the vector.
	pub fn normalized(&self) -> Vector3 {
		let mut normalized_vec = self.clone();
		normalized_vec.normalize();
		normalized_vec
	}

	/// Returns the dot product of two 3D vectors.
	pub fn dot(v1: Vector3, v2: Vector3) -> f32 {
		v1.x * v2.x + v1.y * v2.y + v1.z * v2.z
	}

	pub fn cross(v1: Vector3, v2: Vector3) -> Vector3 {
		Vector3::new(
			v1.y * v2.z - v1.z * v2.y,
			v1.z * v2.x - v1.x * v2.z,
			v1.x * v2.y - v1.y * v2.x,
		)
	}
}

impl std::ops::Add for Vector3 {
	type Output = Self;

	fn add(self, rhs: Self) -> Self {
		Self {
			x: self.x + rhs.x,
			y: self.y + rhs.y,
			z: self.z + rhs.z,
		}
	}
}

impl std::ops::Sub for Vector3 {
	type Output = Self;

	fn sub(self, rhs: Self) -> Self {
		Self {
			x: self.x - rhs.x,
			y: self.y - rhs.y,
			z: self.z - rhs.z,
		}
	}
}

impl std::ops::Mul for Vector3 {
	type Output = Self;

	fn mul(self, rhs: Self) -> Self {
		Self {
			x: self.x * rhs.x,
			y: self.y * rhs.y,
			z: self.z * rhs.z,
		}
	}
}

impl std::ops::Mul<f32> for Vector3 {
	type Output = Self;

	fn mul(self, rhs: f32) -> Self {
		Self {
			x: self.x * rhs,
			y: self.y * rhs,
			z: self.z * rhs,
		}
	}
}

impl std::ops::Div for Vector3 {
	type Output = Self;

	fn div(self, rhs: Self) -> Self {
		Self {
			x: self.x / rhs.x,
			y: self.y / rhs.y,
			z: self.z / rhs.z,
		}
	}
}

impl std::ops::Rem for Vector3 {
	type Output = Self;

	fn rem(self, rhs: Vector3) -> Self {
		Self {
			x: modf(self.x, rhs.x),
			y: modf(self.y, rhs.y),
			z: modf(self.z, rhs.z),
		}
	}
}

impl std::ops::Div<f32> for Vector3 {
	type Output = Self;

	fn div(self, rhs: f32) -> Self {
		Self {
			x: self.x / rhs,
			y: self.y / rhs,
			z: self.z / rhs,
		}
	}
}

impl std::ops::Rem<f32> for Vector3 {
	type Output = Self;

	fn rem(self, rhs: f32) -> Self {
		Self {
			x: modf(self.x, rhs),
			y: modf(self.y, rhs),
			z: modf(self.z, rhs),
		}
	}
}

impl std::ops::AddAssign for Vector3 {
	fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
			z: self.z + rhs.z,
        };
    }
}

impl std::ops::SubAssign for Vector3 {
	fn sub_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
			z: self.z - rhs.z,
        };
    }
}

impl std::ops::MulAssign for Vector3 {
	fn mul_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
			z: self.z * rhs.z,
        };
    }
}

impl std::ops::RemAssign for Vector3 {
	fn rem_assign(&mut self, rhs: Vector3) {
		*self = Self {
			x: modf(self.x, rhs.x),
			y: modf(self.y, rhs.y),
			z: modf(self.z, rhs.z),
		}
	}
}

impl std::ops::MulAssign<f32> for Vector3 {
	fn mul_assign(&mut self, rhs: f32) {
        *self = Self {
            x: self.x * rhs,
            y: self.y * rhs,
			z: self.z * rhs,
        };
    }
}

impl std::ops::DivAssign<f32> for Vector3 {
	fn div_assign(&mut self, rhs: f32) {
        *self = Self {
            x: self.x / rhs,
            y: self.y / rhs,
			z: self.z / rhs,
        };
    }
}

impl std::ops::RemAssign<f32> for Vector3 {
	fn rem_assign(&mut self, rhs: f32) {
		*self = Self {
			x: modf(self.x, rhs),
			y: modf(self.y, rhs),
			z: modf(self.z, rhs),
		}
	}
}

impl std::ops::Neg for Vector3 {
	type Output = Self; 
	fn neg(self) -> Self {
        Self {
			x: -self.x,
			y: -self.y,
			z: -self.z,
		}
    }
}