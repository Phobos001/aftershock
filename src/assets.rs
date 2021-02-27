extern crate lodepng;
extern crate rgb;

use crate::color::*;
use rgb::*;

/// Uncompressed bitmap image, typically loaded from PNG files.
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
				}
			},
			Err(reason) => {
				println!("ASERROR - IMAGE: Could not load | {}", reason);
				return Image::default();
			}
		};
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
			
		} else {
			println!("ASERROR - IMAGE: Buffer is not initialized. Did you remember to use load()?");
		}
	}

	/// Get a pixel color from the image.
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

/// Bitmap font for drawing simple text. To be used with the Rasterizers pprint function.
/// All bitmap fonts need a glyph index that's in order of left-to-right, top-to-bottom of the glyphs used
/// in the font image. The glyph index is used as a lookup table to find the corrisponding glyph subimage.
///
/// For example, a simple five glyph image in the order of 'N', 'O', 'W', 'A', 'Y' must have a glyphidx
/// of "NOWAY" for it to print your text correctly.
/// 
/// The glyph width and height tells the font how big the sections are for the glyphs in the image.
pub struct Font {
	pub glyphidx: Vec<char>,
	pub fontimg: Image,
	pub glyph_width: usize,
	pub glyph_height: usize,
	pub glyph_spacing: i32,
}


impl Font {

	/// Load a font image from disk. The order
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

	/// rusttype will be implemented here eventually. for now this does nothing.
	pub fn new_ttf(path_image: &str) {
		// TODO
	}
}