
use crate::math::*;
#[derive(Debug, Copy, Clone)]
pub struct Color {
	pub r: u8,
	pub g: u8,
	pub b: u8,
	pub a: u8,
}

impl Color {
	pub fn new(r: u8, g: u8, b: u8, a: u8) -> Color {
		Color {
			r, g, b, a
		}
	}

	/// Accurate but slow alpha-blending function
	pub fn blend(src: Color, dst: Color, opacity: f32) -> Color {
		if src.a <= 0 { return Color::clear(); }

		let src_rf32 = src.r as f32 / 255.0;
		let src_gf32 = src.g as f32 / 255.0;
		let src_bf32 = src.b as f32 / 255.0;
		let src_af32 = (src.a as f32 / 255.0) * opacity;

		let dst_rf32 = dst.r as f32 / 255.0;
		let dst_gf32 = dst.g as f32 / 255.0;
		let dst_bf32 = dst.b as f32 / 255.0;
		let dst_af32 = dst.a as f32 / 255.0;

		let fa = (src_af32 + dst_af32 * (1.0 - src_af32));
		let fr = (src_rf32 * src_af32 + dst_rf32 * (1.0 - src_af32)) / fa;
		let fg = (src_gf32 * src_af32 + dst_gf32 * (1.0 - src_af32)) / fa;
		let fb = (src_bf32 * src_af32 + dst_bf32 * (1.0 - src_af32)) / fa;

		let r = (fr * 255.0).round() as u8;
		let g = (fg * 255.0).round() as u8;
		let b = (fb * 255.0).round() as u8;
		let a = (fa * 255.0).round() as u8;
		
		Color { r, g, b, a}
	}

	/// Faster but lest accurate alpha-blending function. https://www.codeguru.com/cpp/cpp/algorithms/general/article.php/c15989/Tip-An-Optimized-Formula-for-Alpha-Blending-Pixels.htm
	pub fn blend_fast(src: Color, dst: Color, opacity: f32) -> Color {
		if src.a <= 0 { return Color::clear(); }

		let alpha: u32 = lerpu8(0, src.a, opacity) as u32;

		let sr: u32 = src.r as u32;
		let sg: u32 = src.g as u32;
		let sb: u32 = src.b as u32;

		let dr: u32 = dst.r as u32;
		let dg: u32 = dst.g as u32;
		let db: u32 = dst.b as u32;

		let r = ((sr * alpha) + (dr * (255 - alpha))) >> 8;
		let g = ((sg * alpha) + (dg * (255 - alpha))) >> 8;
		let b = ((sb * alpha) + (db * (255 - alpha))) >> 8;

		Color { r: r as u8, g: g as u8, b: b as u8, a: 255}

	}

	pub fn inverted(&self) -> Color {
		Color {
			r: 255 - self.r,
			g: 255 - self.g,
			b: 255 - self.b,
			a: self.a,
		}
	}

	pub fn clear() -> Color {
		Color { r: 0, g: 0, b: 0, a: 0 }
	}


	pub fn black() -> Color {
		Color { r: 0, g: 0, b: 0, a: 255 }
	}

	pub fn white() -> Color {
		Color { r: 255, g: 255, b: 255, a: 255 }
	}

	pub fn red() -> Color {
		Color { r: 255, g: 0, b: 0, a: 255 }
	}

	pub fn green() -> Color {
		Color { r: 0, g: 255, b: 0, a: 255 }
	}

	pub fn blue() -> Color {
		Color { r: 0, g: 0, b: 255, a: 255 }
	}

	pub fn yellow() -> Color {
		Color { r: 255, g: 255, b: 0, a: 255 }
	}

	pub fn cyan() -> Color {
		Color { r: 0, g: 255, b: 255, a: 255 }
	}

	pub fn magenta() -> Color {
		Color { r: 255, g: 0, b: 255, a: 255 }
	}

	pub fn orange() -> Color {
		Color { r: 255, g: 128, b: 0, a: 255 }
	}
}

impl std::ops::Add for Color {
	type Output = Self;

	fn add(self, rhs: Self) -> Self {
		Color::new(
			self.r + rhs.r,
			self.g + rhs.g,
			self.b + rhs.b,
			self.a + rhs.a,
		)
	}
}

impl std::ops::Sub for Color {
	type Output = Self;

	fn sub(self, rhs: Self) -> Self {
		Color::new(
			self.r - rhs.r,
			self.g - rhs.g,
			self.b - rhs.b,
			self.a - rhs.a,
		)
	}
}

impl std::ops::Mul for Color {
	type Output = Self;

	fn mul(self, rhs: Self) -> Self {
		let sr32 = self.r as u32;
		let sg32 = self.g as u32;
		let sb32 = self.b as u32;

		let rhsr32 = rhs.r as u32;
		let rhsg32 = rhs.g as u32;
		let rhsb32 = rhs.b as u32;
		
		Color::new(
			(((sr32 * rhsr32 + 255)) >> 8) as u8,
			(((sg32 * rhsg32 + 255)) >> 8) as u8,
			(((sb32 * rhsb32 + 255)) >> 8) as u8,
			self.a,
		)
	}
}

impl std::ops::Div for Color {
	type Output = Self;

	fn div(self, rhs: Self) -> Self {
		Color::new(
			self.r / rhs.r,
			self.g / rhs.g,
			self.b / rhs.b,
			self.a / rhs.a,
		)
	}
}

impl std::ops::AddAssign for Color {
	fn add_assign(&mut self, rhs: Self) {
        *self = Color::new(
			self.r + rhs.r,
			self.g + rhs.g,
			self.b + rhs.b,
			self.a + rhs.a,
		);
    }
}

impl std::ops::SubAssign for Color {
	fn sub_assign(&mut self, rhs: Self) {
        *self = Color::new(
			self.r - rhs.r,
			self.g - rhs.g,
			self.b - rhs.b,
			self.a - rhs.a,
		);
    }
}

impl std::ops::MulAssign for Color {
	fn mul_assign(&mut self, rhs: Self) {
        *self = Color::new(
			self.r * rhs.r,
			self.g * rhs.g,
			self.b * rhs.b,
			self.a * rhs.a,
		);
    }
}

impl std::ops::DivAssign for Color {
	fn div_assign(&mut self, rhs: Self) {
        *self = Color::new(
			self.r / rhs.r,
			self.g / rhs.g,
			self.b / rhs.b,
			self.a / rhs.a,
		);
    }
}


/* #[derive(Debug, Copy, Clone)]
pub struct Color {
	r: f32,
	g: f32,
	b: f32,
	a: f32,
}

impl Color {
	pub fn new(r: f32, g: f32, b: f32, a: f32) -> Color {
		Color {
			r: clampf(r, 0.0, 1.0),
			g: clampf(g, 0.0, 1.0),
			b: clampf(b, 0.0, 1.0),
			a: clampf(a, 0.0, 1.0),
		}
	}

	pub fn new_from_bit8(color: [u8; 4]) -> Color {
		Color {
			r: color[0] as f32 / 255.0,
			g: color[1] as f32 / 255.0,
			b: color[2] as f32 / 255.0,
			a: color[3] as f32 / 255.0,
		}
	}

	pub fn alpha_composite_gamma(c1: Color, c2: Color, gamma: f32) -> Color {
		let r = (c1.r.powf(gamma) * c1.a + c2.r.powf(gamma) * (1.0 - c2.a)).powf(1.0 / gamma);
		let g = (c1.g.powf(gamma) * c1.a + c2.g.powf(gamma) * (1.0 - c2.a)).powf(1.0 / gamma);
		let b = (c1.b.powf(gamma) * c1.a + c2.b.powf(gamma) * (1.0 - c2.a)).powf(1.0 / gamma);
		let a = (c1.a.powf(gamma) * c1.a + c2.a.powf(gamma) * (1.0 - c2.a)).powf(1.0 / gamma);
		Color::new(r, g, b, a)
	}

	pub fn bit8(&self) -> [u8; 4] {
		[
			(self.r * 255.0).round() as u8,
			(self.g * 255.0).round() as u8,
			(self.b * 255.0).round() as u8,
			(self.a * 255.0).round() as u8,
		]
	}
} */
