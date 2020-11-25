use crate::all::*;

#[derive(Debug)]
pub enum PolyRasterizerCoreFormat {
	Core2, // Split down middle
	Core3, // Split in thirds
	Core4, // Split into quadrents
	Core6, // Split into thirds and in half
	Core8, // Split into fourths and in half
	Core9, // Split into thirds twice
	Core12, // Split into thirds three times
	Core16, // Split into thirds four times
}

pub struct PolyRasterizer {
	pub rasterizers: Vec<Rasterizer>,
	pub rasterizer_offsets: Vec<(usize, usize)>,
	pub core_format: PolyRasterizerCoreFormat,
}

impl PolyRasterizer {
	pub fn new(width: usize, height: usize, core_format: PolyRasterizerCoreFormat) -> PolyRasterizer {
		println!("Polyrasterizer: {:?} format", core_format);
		let mut w: usize = 0;
		let mut h: usize = 0;

		if (width > height) { // Horizontal Screen
			w = width;
			h = height;
		} else if (width < height) { // Vertical Screen
			w = height;
			h = width;
		} else if (width == height) { // Square Screen
			w = width;
			h = width;
		}

		let mut new_rasterizers: Vec<Rasterizer> = Vec::new();
		let mut new_offsets: Vec<(usize, usize)> = Vec::new();

		match core_format {
			PolyRasterizerCoreFormat::Core2 => {
				new_rasterizers.push(Rasterizer::new(w / 2, h));
				new_offsets.push((0, 0));

				new_rasterizers.push(Rasterizer::new(w / 2, h));
				new_offsets.push((w / 2, 0));
			},
			PolyRasterizerCoreFormat::Core3 => {
				new_rasterizers.push(Rasterizer::new(w / 3, h));
				new_offsets.push((0, 0));

				new_rasterizers.push(Rasterizer::new(w / 3, h));
				new_offsets.push((w / 3, 0));

				new_rasterizers.push(Rasterizer::new(w / 3, h));
				new_offsets.push(((w / 3) + (w / 3), 0));
			},
			PolyRasterizerCoreFormat::Core4 => {
				new_rasterizers.push(Rasterizer::new(w / 2, h / 2));
				new_offsets.push((0, 0));

				new_rasterizers.push(Rasterizer::new(w / 2, h / 2));
				new_offsets.push((w / 2, 0));

				new_rasterizers.push(Rasterizer::new(w / 2, h / 2));
				new_offsets.push((0, h / 2));

				new_rasterizers.push(Rasterizer::new(w / 2, h / 2));
				new_offsets.push((w / 2, h / 2));
			},
			_ => {}
		}

		PolyRasterizer {
			rasterizers: new_rasterizers,
			rasterizer_offsets: new_offsets,
			core_format,
		}
	}
}