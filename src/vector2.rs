use crate::math::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ThreePointOrientation {
	CounterClockwise,
	Colinear,
	Clockwise,
}

/// Two-dimensional floating-point Vector to be used as either a position or direction.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vector2 {
	pub x: f32,
	pub y: f32,
}

impl Vector2 {

	pub fn new(x: f32, y: f32) -> Vector2 {
		Vector2 {
			x,
			y,
		}
	}

	pub fn zero() -> Vector2 { Vector2 { x: 0.0, y: 0.0, } }
	pub fn one() -> Vector2 { Vector2 { x: 1.0, y: 1.0, } }
	pub fn up() -> Vector2 { Vector2 { x: 0.0, y: -1.0, } }
	pub fn down() -> Vector2 { Vector2 { x: 0.0, y: 1.0, } }
	pub fn left() -> Vector2 { Vector2 { x: -1.0, y: 0.0, } }
	pub fn right() -> Vector2 { Vector2 {x: 1.0, y: 0.0, } }

	/// Gets the width/height ratio of the vector as a 32-bit float.
	pub fn ratio(&self) -> f32 {
		self.x / self.y
	}

	/// Returns the squared magnitude of the vector.
	pub fn magnitude_sqr(&self) -> f32 {
		(self.x * self.x) + (self.y * self.y)
	}

	/// Returns the real magnitude of the vector.
	pub fn magnitude(&self) -> f32 {
		((self.x * self.x) + (self.y * self.y)).sqrt()
	}

	/// Sets the vectors magintude to 1.0 while retaining its direction.
	pub fn normalize(&mut self) {
		let magnitude = self.magnitude();
		self.x /= magnitude;
		self.y /= magnitude;
	}

	/// Returns a normalized copy of the vector.
	pub fn normalized(&self) -> Vector2 {
		let magnitude = self.magnitude();
		Vector2::new(
			self.x / magnitude,
			self.y / magnitude,
		)
	}

	/// Returns the dot product of two 2D vectors.
	pub fn dot(v1: Vector2, v2: Vector2) -> f32 {
		v1.x * v2.x + v1.y * v2.y
	}

	/// Returns the cross product of two 2D vectors.
	pub fn cross(v1: Vector2, v2: Vector2) -> f32 {
		v1.x * v2.y - v1.y * v2.x
	}

	/// Returns the 2D distance between two points.
	pub fn distance(v1: Vector2, v2: Vector2) -> f32{
		((v2.x - v1.x).powf(2.0) + (v2.y - v1.y).powf(2.0)).sqrt()
	}

	pub fn rotated(&self, radians: f32) -> Vector2 {
		let cos = radians.cos();
		let sin = radians.sin();
		Vector2::new(
			(cos * self.x) - (sin * self.y),
			(sin * self.x) + (cos * self.y)
		)
	}

	/// Get orientation of three points
	pub fn orientation(v1: Vector2, v2: Vector2, v3: Vector2) -> ThreePointOrientation {
		let orientation = (v2.y - v1.y) * (v3.x - v2.x) - (v2.x - v1.x) * (v3.y - v2.y);
		if orientation > 0.0 { ThreePointOrientation::Clockwise } else if orientation < 0.0 { ThreePointOrientation::CounterClockwise} else { ThreePointOrientation::Colinear }
	}

	pub fn reflect(direction: Vector2, normal: Vector2) -> Vector2 {
		direction - (normal * Vector2::dot(direction, normal) * 2.0)
	}

	pub fn lerp(v1: Vector2, v2: Vector2, t: f32) -> Vector2 {
		Vector2 {
			x: lerpf(v1.x, v2.x, t),
			y: lerpf(v1.y, v2.y, t),
		}
	}
}

impl std::ops::Add for Vector2 {
	type Output = Self;

	fn add(self, rhs: Self) -> Self {
		Self {
			x: self.x + rhs.x,
			y: self.y + rhs.y,
		}
	}
}

impl std::ops::Sub for Vector2 {
	type Output = Self;

	fn sub(self, rhs: Self) -> Self {
		Self {
			x: self.x - rhs.x,
			y: self.y - rhs.y,
		}
	}
}

impl std::ops::Mul for Vector2 {
	type Output = Self;

	fn mul(self, rhs: Self) -> Self {
		Self {
			x: self.x * rhs.x,
			y: self.y * rhs.y,
		}
	}
}

impl std::ops::Mul<f32> for Vector2 {
	type Output = Self;

	fn mul(self, rhs: f32) -> Self {
		Self {
			x: self.x * rhs,
			y: self.y * rhs,
		}
	}
}

impl std::ops::Div for Vector2 {
	type Output = Self;

	fn div(self, rhs: Self) -> Self {
		Self {
			x: self.x / rhs.x,
			y: self.y / rhs.y,
		}
	}
}

impl std::ops::Rem for Vector2 {
	type Output = Self;

	fn rem(self, rhs: Vector2) -> Self {
		Self {
			x: modf(self.x, rhs.x),
			y: modf(self.y, rhs.y),
		}
	}
}

impl std::ops::Div<f32> for Vector2 {
	type Output = Self;

	fn div(self, rhs: f32) -> Self {
		Self {
			x: self.x / rhs,
			y: self.y / rhs,
		}
	}
}

impl std::ops::Rem<f32> for Vector2 {
	type Output = Self;

	fn rem(self, rhs: f32) -> Self {
		Self {
			x: modf(self.x, rhs),
			y: modf(self.y, rhs),
		}
	}
}

impl std::ops::AddAssign for Vector2 {
	fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        };
    }
}

impl std::ops::SubAssign for Vector2 {
	fn sub_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        };
    }
}

impl std::ops::MulAssign for Vector2 {
	fn mul_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        };
    }
}

impl std::ops::RemAssign for Vector2 {
	fn rem_assign(&mut self, rhs: Vector2) {
		*self = Self {
			x: modf(self.x, rhs.x),
			y: modf(self.y, rhs.y),
		}
	}
}

impl std::ops::MulAssign<f32> for Vector2 {
	fn mul_assign(&mut self, rhs: f32) {
        *self = Self {
            x: self.x * rhs,
            y: self.y * rhs,
        };
    }
}

impl std::ops::DivAssign<f32> for Vector2 {
	fn div_assign(&mut self, rhs: f32) {
        *self = Self {
            x: self.x / rhs,
            y: self.y / rhs,
        };
    }
}

impl std::ops::RemAssign<f32> for Vector2 {
	fn rem_assign(&mut self, rhs: f32) {
		*self = Self {
			x: modf(self.x, rhs),
			y: modf(self.y, rhs),
		}
	}
}

impl std::ops::Neg for Vector2 {
	type Output = Self; 
	fn neg(self) -> Self {
        Self {
			x: -self.x,
			y: -self.y,
		}
    }
}