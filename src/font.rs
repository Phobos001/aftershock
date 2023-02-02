//#[cfg(feature = "ttf")]


use crate::color::*;
use crate::buffer::Buffer;

use std::io::prelude::*;
use std::fs::File;

/// Bitmap font for drawing simple text. To be used with the Buffers pprint function.
/// All bitmap fonts need a glyph index that's in order of left-to-right, top-to-bottom of the glyphs used
/// in the font image. The glyph index is used as a lookup table to find the corrisponding glyph subimage.
///
/// For example, a simple five glyph image in the order of 'N', 'O', 'W', 'A', 'Y' must have a glyphidx
/// of "NOWAY" for it to print your text correctly.
/// 
/// The glyph width and height tells the font how big the sections are for the glyphs in the image.

#[derive(Clone)]
pub struct Font {
	pub glyphidx: Vec<char>,
	pub glyphidx_sizes: Vec<FontGlyph>,
	pub fontimg: Buffer,
	pub glyph_width: usize,
	pub glyph_height: usize,
	pub glyph_spacing: i32,
}


impl Font {

	/// Load a font image from disk. The order
	pub fn new(path_image: &str, glyphidxstr: &str, glyph_width: usize, glyph_height: usize, glyph_spacing: i32) -> Result<Font, String> {

		let glyphidx = glyphidxstr.to_string().chars().collect();
		let glyphidx_sizes: Vec<FontGlyph> = Vec::new();

		
		let fontimg_result = Buffer::new_from_image(path_image);
		if fontimg_result.is_ok() {

	
			Ok(Font {
				glyphidx,
				glyphidx_sizes,
				fontimg: fontimg_result.unwrap(),
				glyph_width,
				glyph_height,
				glyph_spacing,
			})
		} else {
			println!("ERROR - FONT: Font image could not be loaded from path {} !", path_image);
			Err(format!("ERROR - FONT: Font image could not be loaded from path {} !", path_image))
		}

		
	}

	pub fn default() -> Font {
		Font { glyphidx: Vec::new(), glyphidx_sizes: Vec::new(), fontimg: Buffer::new(0, 0), glyph_width: 0, glyph_height: 0, glyph_spacing: 0 }
	}

	pub fn new_ttf(path_ttf: &str, glyphidxstr: &str, glyph_spacing: i32, point_size: f32, alpha_threshold: f32) -> Font {
		
		let mut ttf_file = File::open(path_ttf).expect(format!("ERROR - FONT: TTF file {} does not exist!", path_ttf).as_str());
		let mut ttf_buffer: Vec<u8> = Vec::new();

		let _bytecount = ttf_file.read_to_end(&mut ttf_buffer).expect("ERROR - FONT: TTF File could not be read.");

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

		let mut fontimg: Buffer = Buffer::new(glyphs_width as usize, glyphs_height as usize);

		let mut glyphidx_sizes: Vec<FontGlyph> = Vec::with_capacity(glyphidx.len());

        for i in 0..glyphs.len() {
            if let Some(bounding_box) = glyphs[i].pixel_bounding_box() {
				glyphidx_sizes[i].x = bounding_box.min.x as i32;
				glyphidx_sizes[i].y = bounding_box.min.y as i32;
				glyphidx_sizes[i].w = bounding_box.max.x as i32;
				glyphidx_sizes[i].h = bounding_box.max.y as i32;

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