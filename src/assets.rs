extern crate lodepng;
extern crate rgb;
extern crate rusttype;

use std::io;
use std::io::prelude::*;
use std::fs::File;

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
		let mut img = Image {
			buffer: Vec::new(),
			width: 1,
			height: 1,
		};

		// Blank pixel
		img.buffer.push(0);
		img.buffer.push(0);
		img.buffer.push(0);
		img.buffer.push(0);
		img
	}

	pub fn new_with_size(width: usize, height: usize) -> Image {
		Image {
			buffer: vec![0; width * height * 4],
			width,
			height,
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
	pub glyphidx_sizes: Vec<FontGlyph>,
	pub fontimg: Image,
	pub glyph_width: usize,
	pub glyph_height: usize,
	pub glyph_spacing: i32,
}


impl Font {

	/// Load a font image from disk. The order
	pub fn new(path_image: &str, glyphidxstr: &str, glyph_width: usize, glyph_height: usize, glyph_spacing: i32) -> Font {

		let glyphidx = glyphidxstr.to_string().chars().collect();
		let glyphidx_sizes: Vec<FontGlyph> = Vec::new();

		
		let fontimg: Image = Image::new(path_image);

		if fontimg.buffer.len() <= 0 {
			println!("ERROR - FONT: Font image {} does not exist or could not be loaded!", path_image);
		}

		//println!("Font: {} loaded with {}B image size", path_image, fontimg.width * fontimg.height);

		Font {
			glyphidx,
			glyphidx_sizes,
			fontimg,
			glyph_width,
			glyph_height,
			glyph_spacing,
		}
	}

	pub fn new_ttf(path_ttf: &str, glyphidxstr: &str, glyph_spacing: i32, point_size: f32, alpha_threshold: f32) -> Font {
		
		let mut ttf_file = File::open(path_ttf).expect(format!("ERROR - FONT: TTF file {} does not exist!", path_ttf).as_str());
		let mut ttf_buffer: Vec<u8> = Vec::new();

		let bytecount = ttf_file.read_to_end(&mut ttf_buffer).expect("ERROR - FONT: TTF File could not be read.");

		let ttf = rusttype::Font::try_from_vec(ttf_buffer).expect(format!("ERROR - FONT: TTF Font {} cannot be constructed. Make sure there is only one font inside the TTF file.", path_ttf).as_str());


		let glyphidx: Vec<char> = glyphidxstr.to_string().chars().collect();

		let scale = rusttype::Scale::uniform(point_size);
        let mut v_metrics = ttf.v_metrics(scale);
        v_metrics.line_gap = point_size;

        let glyphs: Vec<_> = ttf.layout(
            glyphidxstr, 
            rusttype::Scale::uniform(point_size), 
            rusttype::point(0.0, v_metrics.ascent)).collect();

        // work out the layout size
        let glyphs_height = (v_metrics.ascent - v_metrics.descent).ceil() as u32;
        let glyphs_width = {
            let min_x = glyphs
                .first()
                .map(|g| g.pixel_bounding_box().unwrap().min.x)
                .unwrap();
            let max_x = glyphs
                .last()
                .map(|g| g.pixel_bounding_box().unwrap().max.x)
                .unwrap();
            (max_x - min_x) as u32
        };

		let mut fontimg: Image = Image::new_with_size(glyphs_width as usize, glyphs_height as usize);

		let mut glyphidx_sizes: Vec<FontGlyph> = Vec::with_capacity(glyphidx.len());

        for i in 0..glyphs.len() {
            if let Some(bounding_box) = glyphs[i].pixel_bounding_box() {
				glyphidx_sizes[i].x = bounding_box.min.x;
				glyphidx_sizes[i].y = bounding_box.min.y;
				glyphidx_sizes[i].w = bounding_box.max.x;
				glyphidx_sizes[i].h = bounding_box.max.y;

                // Draw the glyph into the image per-pixel by using the draw closure
                glyphs[i].draw(|x, y, v| {
                    fontimg.pset(
                        // Offset the position by the glyph bounding box
                        x as i32 + bounding_box.min.x as i32,
                        y as i32 + bounding_box.min.y as i32,
                        // Turn the coverage into an alpha value
                        Color::new(255, 255, 255, if v > alpha_threshold { 255 } else { 0 })
                    )
                });
            }
        }

		Font {
			glyphidx,
			glyphidx_sizes,
			fontimg,
			glyph_width: point_size as usize,
			glyph_height: point_size as usize,
			glyph_spacing
		}
	}
}

#[derive(Copy, Clone, Debug)]
pub struct FontGlyph {
	pub x: i32,
	pub y: i32,
	pub w: i32,
	pub h: i32,
}

impl FontGlyph {
	pub fn new (x: i32, y: i32, w: i32, h: i32) -> FontGlyph {
		FontGlyph {
			x,
			y,
			w,
			h,
		}
	}
}