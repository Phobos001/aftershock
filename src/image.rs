extern crate lodepng;
extern crate rgb;

use crate::color::*;
use rgb::*;

#[derive(Debug, Clone, PartialEq, Eq)]
/// Uncompressed bitmap image, typically loaded from PNG files.
pub struct Image {
	pub buffer: Vec<u8>,
	pub width: usize,
	pub height: usize,

	pub is_error: bool,
}

impl Image {

	pub fn default() -> Image {
		let mut img = Image {
			buffer: Vec::new(),
			width: 1,
			height: 1,
			is_error: true,
		};

		// White pixel (RGBA 4 Bytes)
		img.buffer.push(255);
		img.buffer.push(255);
		img.buffer.push(255);
		img.buffer.push(0);
		img
	}

	pub fn new_with_size(width: usize, height: usize) -> Image {
		Image {
			buffer: vec![0; width * height * 4],
			width,
			height,
			is_error: false,
		}
	}

	/// Create a new image and load from disk. Only supports PNG files.
	pub fn new(path_to: &str) -> Image {
		match lodepng::decode32_file(path_to) {
			Ok(image) => {
				//println!("Image: {}, Res: {} x {}, Size: {}B", path_to, image.width, image.height, image.buffer.len());
				let buffer_new: Vec<u8> =  image.buffer.as_bytes().to_vec();

				return Image {
					buffer: buffer_new,
					width: image.width,
					height: image.height,
					is_error: false
				}
			},
			Err(reason) => {
				println!("ERROR - IMAGE: Could not load {} | {}", path_to, reason);
				return Image::default();
			}
		};
	}

	pub fn save(&self, path_to: &str) {
		let _ = lodepng::encode32_file(path_to, self.buffer.as_slice(), self.width, self.height);
	}

	/// Change a pixel color in the image.
	pub fn pset(&mut self, x: i32, y: i32, color: Color) {
		if self.buffer.len() > 0 {
			let idx: usize = ((y * (self.width as i32) + x) * 4) as usize;
        
			let out_left: bool = x < 0;
			let out_right: bool = x > (self.width) as i32 - 1;
			let out_top: bool = y < 0;
			let out_bottom: bool = y > (self.height) as i32 - 1;
			let out_of_range: bool = idx > (self.width * self.height * 4) - 1;

			if out_of_range || out_left || out_right || out_top || out_bottom  { return; }

			self.buffer[idx + 0] = color.r;  // R
			self.buffer[idx + 1] = color.g;  // G
			self.buffer[idx + 2] = color.b;  // B
			self.buffer[idx + 3] = color.a;  // A
			
		}
	}

	/// Get a pixel color from the image.
	pub fn pget(&self, x: i32, y: i32) -> Color {
		if self.buffer.len() > 0 {
			let idx: usize = ((y * (self.width as i32) + x) as usize) * 4;

			let out_left: bool = x < 0;
			let out_right: bool = x > (self.width) as i32 - 1;
			let out_top: bool = y < 0;
			let out_bottom: bool = y > (self.height) as i32 - 1;
			let out_of_range: bool = idx > (self.width * self.height * 4) - 1;

			if out_of_range || out_left || out_right || out_top || out_bottom  { return Color::clear(); }

			return Color::new(
				self.buffer[idx + 0],
				self.buffer[idx + 1],
				self.buffer[idx + 2],
				self.buffer[idx + 3]
			);
		} else {
			Color::black()
		}
	}
}