extern crate lodepng;
extern crate rgb;
extern crate rusttype;

use crate::color::*;
use rgb::*;

pub struct Image {
	pub buffer: Vec<u8>,
	pub width: usize,
	pub height: usize,
}

impl Image {

	pub fn default() -> Image {
		Image {
			buffer: Vec::new(),
			width: 0,
			height: 0,
		}
	}

	pub fn new(path_to: &str) -> Image {
		match lodepng::decode32_file(path_to) {
			Ok(image) => {
				//println!("Image: {}, Res: {} x {}, Size: {}B", path_to, image.width, image.height, image.buffer.len());
				let buffer_new: Vec<u8> =  image.buffer.as_bytes().to_vec();

				return Image {
					buffer: buffer_new,
					width: image.width,
					height: image.height,
				}
			},
			Err(reason) => {
				println!("ASERROR - IMAGE: Could not load | {}", reason);
				return Image::default();
			}
		};
	}

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
			
		} else {
			println!("ASERROR - IMAGE: Buffer is not initialized. Did you remember to use load()?");
		}
	}

	pub fn pget(&self, x: i32, y: i32) -> Color {
		if self.buffer.len() > 0 {
			let idx: usize = (y * (self.width as i32) + x) as usize * 4;

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
			println!("ASERROR - IMAGE: Buffer is not initialized. Did you remember to use load()?");
			Color::black()
		}
	}
}

pub struct Font {
	pub glyphidx: Vec<char>,
	pub fontimg: Image,
	pub glyph_width: usize,
	pub glyph_height: usize,
	pub glyph_spacing: i32,
}


impl Font {
	pub fn new(path_image: &str, glyphidxstr: &str, glyph_width: usize, glyph_height: usize, glyph_spacing: i32) -> Font {

		let glyphidx = glyphidxstr.to_string().chars().collect();
		let fontimg: Image = Image::new(path_image);

		if fontimg.buffer.len() <= 0 {
			println!("ASERROR - FONT: Font image {} does not exist or could not be loaded!", path_image);
		}

		//println!("Font: {} loaded with {}B image size", path_image, fontimg.width * fontimg.height);

		Font {
			glyphidx,
			fontimg,
			glyph_width,
			glyph_height,
			glyph_spacing,
		}
	}

	pub fn new_ttf(path_image: &str) {
		// TODO
	}
}

pub struct FontGlyph {
	pub glyphstr: char,
	pub rectx: u32,
	pub recty: u32,
	pub rectw: u32,
	pub recth: u32
}

impl FontGlyph {
	pub fn default() -> FontGlyph {
		FontGlyph {
			glyphstr: 'a',
			rectx: 0,
			recty: 0,
			rectw: 0,
			recth: 0,
		}
		
	}
}