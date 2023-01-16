use crate::math::*;

/// Two-dimensional floating-point Vector to be used as either a position or direction.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vector2 {
	pub x: f64,
	pub y: f64,
}

impl Vector2 {

	pub fn new(x: f64, y: f64) -> Vector2 {
		Vector2 {
			x,
			y,
		}
	}

	pub const ZERO: 	Vector2 = Vector2 {x:  0.0,  y:  0.0,};
	pub const ONE: 		Vector2 = Vector2 {x:  1.0,  y:  1.0,};
	pub const UP: 		Vector2 = Vector2 {x:  0.0,  y: -1.0,};
	pub const DOWN: 	Vector2 = Vector2 {x:  0.0,  y:  1.0,};
	pub const LEFT: 	Vector2 = Vector2 {x: -1.0,  y:  0.0,};
	pub const RIGHT: 	Vector2 = Vector2 {x:  1.0,  y:  0.0,};

	/// Gets the width/height ratio of the vector as a 32-bit float.
	pub fn ratio(&self) -> f64 {
		self.x / self.y
	}

	pub fn angle(v1: Vector2, v2: Vector2) -> f64 {
		f64::atan2(v2.y - v1.y, v2.x - v1.x)
	}

	pub fn angle_between(v1: Vector2, v2: Vector2, fov: f64) -> bool {
		let angle = Vector2::angle(v1, v2);
		let half_fov = fov / 2.0;
		angle > (-half_fov) && angle < half_fov
	}

	/// Returns the squared magnitude of the vector.
	pub fn magnitude_sqr(&self) -> f64 {
		(self.x * self.x) + (self.y * self.y)
	}

	/// Returns the real magnitude of the vector.
	pub fn magnitude(&self) -> f64 {
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
	pub fn dot(v1: Vector2, v2: Vector2) -> f64 {
		v1.x * v2.x + v1.y * v2.y
	}

	/// Returns the cross product of two 2D vectors.
	pub fn cross(v1: Vector2, v2: Vector2) -> f64 {
		v1.x * v2.y - v1.y * v2.x
	}

	/// Returns the 2D distance between two points.
	pub fn distance(v1: Vector2, v2: Vector2) -> f64 {
		((v2.x - v1.x).powf(2.0) + (v2.y - v1.y).powf(2.0)).sqrt()
	}

	pub fn direction(v1: Vector2, v2: Vector2) -> Vector2 {
		(v2 - v1).normalized()
	}

	pub fn rotated(&self, radians: f64) -> Vector2 {
		let cos = radians.cos();
		let sin = radians.sin();
		Vector2::new(
			(cos * self.x) - (sin * self.y),
			(sin * self.x) + (cos * self.y)
		)
	}

	pub fn rotated_pivot(&self, radians: f64, pivot: Vector2) -> Vector2 {
		let cos = radians.cos();
		let sin = radians.sin();

		let x1 = self.x - pivot.x;
		let y1 = self.y - pivot.y;

		let x2 = x1 * cos - y1 * sin;
		let y2 = x1 * sin + y1 * cos;

		Vector2::new(x2 + pivot.x, y2 + pivot.y)
	}

	pub fn slide(direction: Vector2, normal: Vector2) -> Vector2 {
		direction - (normal * Vector2::dot(direction, normal))
	}

	pub fn reflect(direction: Vector2, normal: Vector2) -> Vector2 {
		direction - (normal * Vector2::dot(direction, normal) * 2.0)
	}

	pub fn lerp(v1: Vector2, v2: Vector2, t: f64) -> Vector2 {
		Vector2 {
			x: lerpf(v1.x, v2.x, t),
			y: lerpf(v1.y, v2.y, t),
		}
	}

	pub fn inverse(&self) -> Vector2 {
		Vector2::new(1.0 / self.x, 1.0 / self.y)
	}

	pub fn point_in_aabb(&self, aabb_point: Vector2, aabb_extents: Vector2) -> bool {
		self.x > aabb_point.x - aabb_extents.x &&
		self.x < aabb_point.x + aabb_extents.x &&
		self.y > aabb_point.y - aabb_extents.y &&
		self.y < aabb_point.y + aabb_extents.y
	}

	// Help from https://www.geeksforgeeks.org/program-for-point-of-intersection-of-two-lines/
	pub fn intersection_infinite(p1_start: Vector2, p1_end: Vector2, p2_start: Vector2, p2_end: Vector2) -> (bool, Vector2) {
		let a1: f64 = p1_end.y - p1_start.y;
		let b1: f64 = p1_start.x - p1_end.x;
		let c1: f64 = a1 * p1_start.x + b1 * p1_start.y;

		let a2: f64 = p2_end.y - p2_start.y;
		let b2: f64 = p2_start.x - p2_end.x;
		let c2: f64 = a2 * p2_start.x + b2 * p2_start.y;

		let determinant = a1 * b2 - a2 * b1;

		if determinant == 0.0 { // No intersection: Lines are parallel
			(false, Vector2::ZERO)
		} else {
			let point = Vector2::new(
				(b2 * c1 - b1 * c2) / determinant,
				(a1 * c2 - a2 * c1) / determinant
			);
			(true, point)
		}
	}

	pub fn intersection_segment(ray_start: Vector2, ray_end: Vector2, line_start: Vector2, line_end: Vector2) -> (bool, Vector2) {

		// Check for any intersection at all
		let (intersection_hit, point) = Vector2::intersection_infinite(ray_start, ray_end, line_start, line_end);

		if intersection_hit {

			// First make sure the point is along the segment; between the start and end
			let error_margin: f64 = 0.00001;

			let dist_line = Vector2::distance(line_start, line_end);
			let dist_point_to_line_start = Vector2::distance(line_start, point);
			let dist_point_to_line_end = Vector2::distance(line_end, point);			

			let diff = dist_line - (dist_point_to_line_start + dist_point_to_line_end);
			let is_on_segment = diff < error_margin && diff > -error_margin;

			// Now make sure the intersected line is actually in front of the ray direction
			// Otherwise intersections can occur on lines behind the ray and be incorrect

			let is_in_ray_direction = Vector2::dot(
				Vector2::direction(ray_start, ray_end), 
				Vector2::direction(ray_start, point)
			) > 0.0;

			// Finally make sure the point is in the distance of the rays magnitude
			let is_in_distance = Vector2::distance(ray_start, point) < Vector2::distance(ray_start, ray_end);

			if is_in_ray_direction && is_on_segment && is_in_distance {
				(true, point)
			} else {
				(false, Vector2::ZERO)
			}
		} else {
			(false, point)
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

impl std::ops::Mul<f64> for Vector2 {
	type Output = Self;

	fn mul(self, rhs: f64) -> Self {
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
			x: self.x.rem_euclid(rhs.x),
			y: self.y.rem_euclid(rhs.y),
		}
	}
}

impl std::ops::Div<f64> for Vector2 {
	type Output = Self;

	fn div(self, rhs: f64) -> Self {
		Self {
			x: self.x / rhs,
			y: self.y / rhs,
		}
	}
}

impl std::ops::Rem<f64> for Vector2 {
	type Output = Self;

	fn rem(self, rhs: f64) -> Self {
		Self {
			x: self.x.rem_euclid(rhs),
			y: self.y.rem_euclid(rhs),
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
			x: self.x.rem_euclid(rhs.x),
			y: self.y.rem_euclid(rhs.y),
		}
	}
}

impl std::ops::MulAssign<f64> for Vector2 {
	fn mul_assign(&mut self, rhs: f64) {
        *self = Self {
            x: self.x * rhs,
            y: self.y * rhs,
        };
    }
}

impl std::ops::DivAssign<f64> for Vector2 {
	fn div_assign(&mut self, rhs: f64) {
        *self = Self {
            x: self.x / rhs,
            y: self.y / rhs,
        };
    }
}

impl std::ops::RemAssign<f64> for Vector2 {
	fn rem_assign(&mut self, rhs: f64) {
		*self = Self {
			x: self.x.rem_euclid(rhs),
			y: self.y.rem_euclid(rhs),
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