use crate::math::*;

/// Two-dimensional floating-point Vector to be used as either a position or direction.
#[derive(Copy, Clone, Debug)]
pub struct Vec2 {
	pub x: f32,
	pub y: f32,
}

impl Vec2 {
	pub fn new(x: f32, y: f32) -> Vec2 {
		Vec2 {
			x,
			y,
		}
	}

	pub fn zero() -> Vec2 { Vec2 { x: 0.0, y: 0.0, } }
	pub fn one() -> Vec2 { Vec2 { x: 1.0, y: 1.0, } }
	pub fn up() -> Vec2 { Vec2 { x: 0.0, y: 1.0, } }
	pub fn down() -> Vec2 { Vec2 { x: 0.0, y: -1.0, } }
	pub fn left() -> Vec2 { Vec2 { x: -1.0, y: 0.0, } }
	pub fn right() -> Vec2 { Vec2 {x: 1.0, y: 0.0, } }

	pub fn ratio(&self) -> f32 {
		self.x / self.y
	}

	pub fn magnitude_sqr(&self) -> f32 {
		(self.x * self.x) + (self.y * self.y)
	}

	pub fn magnitude(&self) -> f32 {
		((self.x * self.x) + (self.y * self.y)).sqrt()
	}

	pub fn normalize(&mut self) {
		let mut magnitude = self.magnitude_sqr();
		if magnitude > 0.0 {
			magnitude = magnitude.sqrt();
			let length_inv = 1.0 / magnitude;
			self.x *= length_inv;
			self.y *= length_inv;
		} else {
			self.x = 1.0;
			self.y = 0.0;
		}
	}

	pub fn normalized(&self) -> Vec2 {
		let mut normalized_vec = self.clone();
		normalized_vec.normalize();
		normalized_vec
	}

	pub fn dot(v1: Vec2, v2: Vec2) -> f32 {
		v1.x * v2.x + v1.y * v2.y
	}

	pub fn cross(v1: Vec2, v2: Vec2) -> f32 {
		// You cant really do cross products with 2D vectors but if we pretend its 3D we can still get some use out of the result
		v1.x * v2.y - v1.y * v2.x
	}

	pub fn perpendicular_clockwise(&self) -> Vec2 {
		Vec2::new(self.y, -self.x)
	}

	pub fn perpendicular_counterclockwise(&self) -> Vec2 {
		Vec2::new(-self.y, self.x)
	}

	pub fn reflect(direction: Vec2, normal: Vec2) -> Vec2 {
		direction - (normal * Vec2::dot(direction, normal) * 2.0)
	}

	pub fn lerp(v1: Vec2, v2: Vec2, t: f32) -> Vec2 {
		Vec2 {
			x: lerpf(v1.x, v2.x, t),
			y: lerpf(v1.y, v2.y, t),
		}
	}
}

impl std::ops::Add for Vec2 {
	type Output = Self;

	fn add(self, rhs: Self) -> Self {
		Self {
			x: self.x + rhs.x,
			y: self.y + rhs.y,
		}
	}
}

impl std::ops::Sub for Vec2 {
	type Output = Self;

	fn sub(self, rhs: Self) -> Self {
		Self {
			x: self.x - rhs.x,
			y: self.y - rhs.y,
		}
	}
}

impl std::ops::Mul for Vec2 {
	type Output = Self;

	fn mul(self, rhs: Self) -> Self {
		Self {
			x: self.x * rhs.x,
			y: self.y * rhs.y,
		}
	}
}

impl std::ops::Mul<f32> for Vec2 {
	type Output = Self;

	fn mul(self, rhs: f32) -> Self {
		Self {
			x: self.x * rhs,
			y: self.y * rhs,
		}
	}
}

impl std::ops::Div for Vec2 {
	type Output = Self;

	fn div(self, rhs: Self) -> Self {
		Self {
			x: self.x / rhs.x,
			y: self.y / rhs.y,
		}
	}
}

impl std::ops::Rem for Vec2 {
	type Output = Self;

	fn rem(self, rhs: Vec2) -> Self {
		Self {
			x: modf(self.x, rhs.x),
			y: modf(self.y, rhs.y),
		}
	}
}

impl std::ops::Div<f32> for Vec2 {
	type Output = Self;

	fn div(self, rhs: f32) -> Self {
		Self {
			x: self.x / rhs,
			y: self.y / rhs,
		}
	}
}

impl std::ops::Rem<f32> for Vec2 {
	type Output = Self;

	fn rem(self, rhs: f32) -> Self {
		Self {
			x: modf(self.x, rhs),
			y: modf(self.y, rhs),
		}
	}
}

impl std::ops::AddAssign for Vec2 {
	fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        };
    }
}

impl std::ops::SubAssign for Vec2 {
	fn sub_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        };
    }
}

impl std::ops::MulAssign for Vec2 {
	fn mul_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        };
    }
}

impl std::ops::RemAssign for Vec2 {
	fn rem_assign(&mut self, rhs: Vec2) {
		*self = Self {
			x: modf(self.x, rhs.x),
			y: modf(self.y, rhs.y),
		}
	}
}

impl std::ops::MulAssign<f32> for Vec2 {
	fn mul_assign(&mut self, rhs: f32) {
        *self = Self {
            x: self.x * rhs,
            y: self.y * rhs,
        };
    }
}

impl std::ops::DivAssign<f32> for Vec2 {
	fn div_assign(&mut self, rhs: f32) {
        *self = Self {
            x: self.x / rhs,
            y: self.y / rhs,
        };
    }
}

impl std::ops::RemAssign<f32> for Vec2 {
	fn rem_assign(&mut self, rhs: f32) {
		*self = Self {
			x: modf(self.x, rhs),
			y: modf(self.y, rhs),
		}
	}
}

impl std::ops::Neg for Vec2 {
	type Output = Self; 
	fn neg(self) -> Self {
        Self {
			x: -self.x,
			y: -self.y,
		}
    }
}