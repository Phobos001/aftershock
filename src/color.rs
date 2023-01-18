/// 32-bit Color using  1-byte channels for Red, Green, Blue, and Alpha.

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Color {
	pub r: u8,
	pub g: u8,
	pub b: u8,
	pub a: u8,
}

impl Color {

	pub const CLEAR:	Color = Color { r:   0, g:   0, b:   0, a: 255 };
	pub const WHITE: 	Color = Color { r: 255, g: 255, b: 255, a: 255 };
	pub const RED:	 	Color = Color { r: 255, g: 	 0, b:   0, a: 255 };
	pub const GREEN: 	Color = Color { r:   0, g: 255, b:   0, a: 255 };
	pub const BLUE: 	Color = Color { r:   0, g:   0, b: 255, a: 255 };
	pub const MAGENTA: 	Color = Color { r: 255, g:   0, b: 255, a: 255 };
	pub const YELLOW: 	Color = Color { r: 255, g: 255, b:   0, a: 255 };
	pub const CYAN: 	Color = Color { r:   0, g: 255, b: 255, a: 255 };

	/// InVeNt NeW cOlOrS
	pub fn new(r: u8, g: u8, b: u8, a: u8) -> Color {
		Color {
			r, g, b, a
		}
	}

	/// Accurate but slow alpha-blending function
	pub fn blend_slow(src: Color, dst: Color, opacity: f32) -> Color {
		if src.a <= 0 { return Color::CLEAR; }

		let src_rf32 = src.r as f32 / 255.0;
		let src_gf32 = src.g as f32 / 255.0;
		let src_bf32 = src.b as f32 / 255.0;
		let src_af32 = (src.a as f32 / 255.0) * opacity;

		let dst_rf32 = dst.r as f32 / 255.0;
		let dst_gf32 = dst.g as f32 / 255.0;
		let dst_bf32 = dst.b as f32 / 255.0;
		let dst_af32 = dst.a as f32 / 255.0;

		let fa = src_af32 + dst_af32 * (1.0 - src_af32);
		let fr = (src_rf32 * src_af32 + dst_rf32 * (1.0 - src_af32)) / fa;
		let fg = (src_gf32 * src_af32 + dst_gf32 * (1.0 - src_af32)) / fa;
		let fb = (src_bf32 * src_af32 + dst_bf32 * (1.0 - src_af32)) / fa;

		let r = (fr * 255.0).round() as u8;
		let g = (fg * 255.0).round() as u8;
		let b = (fb * 255.0).round() as u8;
		let a = (fa * 255.0).round() as u8;
		
		Color { r, g, b, a}
	}

	/// Faster but less accurate alpha-blending function. Used in rasterizer since it's accurate enough and removes branching in hot code
	/// <https://www.codeguru.com/cpp/cpp/algorithms/general/article.php/c15989/Tip-An-Optimized-Formula-for-Alpha-Blending-Pixels.htm>
	pub fn blend_fast(src: Color, dst: Color, opacity: u8) -> Color {
		let alpha: u32 = (src.a - (255 - opacity)) as u32;

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

	/// Byte inverted copy of the color
	pub fn inverted(&self) -> Color {
		Color {
			r: 255 - self.r,
			g: 255 - self.g,
			b: 255 - self.b,
			a: self.a,
		}
	}

	/// Hue, Saturation, and Value color definition. Should not be used per pixel due to casting and division use.
	pub fn hsv(hue: f32, saturation: f32, value: f32) -> Color {
		let hi: i32 = ((hue / 60.0).floor() as i32) % 6;
		let f: f32 = (hue / 60.0) - (hue / 60.0).floor();

		let p: f32 = value * (1.0 - saturation);
		let q: f32 = value * (1.0 - (f * saturation));
		let t: f32 = value * (1.0 - ((1.0 - f) * saturation));

		match hi
		{
			0 => { return Color::new((value * 255.0) as u8, (t * 255.0) as u8, (p * 255.0) as u8, 255); },
			1 => { return Color::new((q * 255.0) as u8, (value * 255.0) as u8, (p * 255.0) as u8, 255); },
			2 => { return Color::new((p * 255.0) as u8, (value * 255.0) as u8, (t * 255.0) as u8, 255); },
			3 => { return Color::new((p * 255.0) as u8, (q * 255.0) as u8, (value * 255.0) as u8, 255); },
			4 => { return Color::new((t * 255.0) as u8, (p * 255.0) as u8, (value * 255.0) as u8, 255); },
			5 => { return Color::new((value * 255.0) as u8, (p * 255.0) as u8, (q * 255.0) as u8, 255); },
			_ => { return Color::WHITE; }
		}
	}

	pub fn lerp_rgb(c1: Color, c2: Color, t: f32) -> Color {
		let tb = (f32::clamp(t, 0.0, 1.0) / 255.0) as u8;
		let mut cf: Color = Color::CLEAR;

		cf.r = c1.r + (c2.r - c1.r) * tb;
		cf.g = c1.g + (c2.g - c1.g) * tb;
		cf.b = c1.b + (c2.b - c1.b) * tb;
		cf.a = c1.a;

		cf
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
		let sr32 = self.r as u32;
		let sg32 = self.g as u32;
		let sb32 = self.b as u32;

		let rhsr32 = rhs.r as u32;
		let rhsg32 = rhs.g as u32;
		let rhsb32 = rhs.b as u32;
		
		*self = Color::new(
			(((sr32 * rhsr32 + 255)) >> 8) as u8,
			(((sg32 * rhsg32 + 255)) >> 8) as u8,
			(((sb32 * rhsb32 + 255)) >> 8) as u8,
			self.a,
		)
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