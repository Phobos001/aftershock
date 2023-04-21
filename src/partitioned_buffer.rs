use crate::buffer::*;
use crate::color::*;
use crate::shader;
use crate::shader::Shader;

use std::rc::Rc;
use std::sync::Arc;
use std::thread::*;

pub enum PartitionScheme {
	Full,
	Split2x1,
	Split1x2,
	Split2x2,
	Split3x1,
	Split3x2,
	Split3x3,
	Split4x4,
	Split5x5,
	Split8x8,
}

pub struct PartitionedBuffer {
	pub buffer: Buffer,
	pub partitions: Vec<Buffer>,
	pub scheme: PartitionScheme,
	pub threshold: u32,
}

/// A Buffer that allows for parallel rendering by partioning the image into smaller pieces, usually by how many cores the current CPU has.
impl PartitionedBuffer {

	pub const PARALLEL_THRESHOLD_DEFAULT: u32 = 65536;

	pub fn new(width: usize, height: usize, cores: usize, threshold: u32) -> PartitionedBuffer {
	
		let mut pr = PartitionedBuffer {
			buffer: Buffer::new(width, height),
			partitions:  Vec::new(),
			scheme: PartitionScheme::Full,
			threshold,
		};

		pr.set_core_limit(cores);
		pr
	}

	// For some reason Result doesn't work here????
	pub fn new_from_image(path_to: &str) -> PartitionedBuffer {
		match lodepng::decode32_file(path_to) {
			Ok(image) => {
				//println!("Image: {}, Res: {} x {}, Size: {}B", path_to, image.width, image.height, image.buffer.len());
				let buffer_new: Vec<u8> =  image.buffer.as_bytes().to_vec();
                use rgb::*;

				let mut pr = PartitionedBuffer::new(image.width, image.height, 0, PartitionedBuffer::PARALLEL_THRESHOLD_DEFAULT);
				pr.buffer.color = buffer_new;
				pr.generate_partitions();

				pr
			},
			Err(reason) => {
				println!("ERROR - IMAGE: Could not load {} | {}", path_to, reason);
				PartitionedBuffer::new(1, 1, 1, PartitionedBuffer::PARALLEL_THRESHOLD_DEFAULT)
			}
		}
	}

	pub fn clear(&mut self) {
		self.buffer.clear();
		for part in &mut self.partitions {
			part.clear();
		}
	}

	pub fn clear_color(&mut self, color: Color) {
		self.buffer.clear_color(color);
		for part in &mut self.partitions {
			part.clear_color(color);
		}
	}

	pub fn enable_drawing(&mut self) {
		self.buffer.is_drawing = true;
		for part in &mut self.partitions {
			part.is_drawing = true;
		}
	}

	pub fn disable_drawing(&mut self) {
		self.buffer.is_drawing = false;
		for part in &mut self.partitions {
			part.is_drawing = false;
		}
	}

	pub fn add_shader(&mut self, shader: BufferShader) {
		self.buffer.add_shader(shader.clone());
		for part in &mut self.partitions {
			part.add_shader(shader.clone());
		}
	}

	pub fn set_core_limit(&mut self, cores: usize) {
		let cpu_count = if cores == 0 { num_cpus::get() } else { cores };

		let mut is_unknown: bool = false;

		let mut scheme: PartitionScheme = match cpu_count {
			1 => { PartitionScheme::Full },
			2 => { if self.buffer.height > self.buffer.width { PartitionScheme::Split2x1 } else { PartitionScheme::Split1x2 }},
			3 => { PartitionScheme::Split3x1 }
			4 => { PartitionScheme::Split2x2 },
			5 => { PartitionScheme::Split2x2 },
			6 => { PartitionScheme::Split3x2 },
			8 => { PartitionScheme::Split3x3 },
			10 => { PartitionScheme::Split3x3 },
			12 => { PartitionScheme::Split4x4 },
			16 => { PartitionScheme::Split4x4 },
			20 => { PartitionScheme::Split5x5 },
			24 => { PartitionScheme::Split5x5 },
			_ => { is_unknown = true; PartitionScheme::Split4x4 }
		};

		if is_unknown {
			if cpu_count > 24 {
				scheme = PartitionScheme::Split8x8;
			}
		}
		self.scheme = scheme;
		self.generate_partitions();
	}

	pub fn resize(&mut self, width: usize, height: usize) {
		self.buffer.resize(width, height);
		self.generate_partitions();
	}

	pub fn blit(&mut self, image: &Buffer, x: i32, y: i32) {
		self.buffer.blit(image, x, y);
	}

	pub fn pset(&mut self, x: i32, y: i32, color: Color) {
		self.buffer.pset(x, y, color);
	}

	pub fn pget(&mut self, x: i32, y: i32) -> Color {
		self.buffer.pget(x, y)
	}

	pub fn pget_wrap(&mut self, x: i32, y: i32) -> Color {
		self.buffer.pget_wrap(x, y)
	}

	// Too simple to parallelize
	pub fn pline(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, color: Color) {
		self.buffer.pline(x0, y0, x1, y1, color);
	}

	pub fn prectangle(&mut self, filled: bool, x: i32, y: i32, width: i32, height: i32, color: Color) {
		let total_area = width * height;

		// Run in parallel
		if filled && self.threshold != 0 && total_area >= self.threshold as i32 {
			scope(|s| {
				let mut join_handles: Vec<ScopedJoinHandle<&mut Buffer>> = Vec::new();
			
				for part in & mut self.partitions {
	
					let rx = x - part.offset_x as i32;
					let ry = y - part.offset_y as i32;
	
					let handle = s.spawn(move || {
	
						part.prectangle(filled, rx, ry, width, height, color);
	
						part
					});
					join_handles.push(handle);
				}
	
				for handle in join_handles {
					let part_return = handle.join();
					if part_return.is_ok() {
						let part = part_return.unwrap();
						self.buffer.blit(&part, part.offset_x as i32, part.offset_y as i32);
					} else {
						println!("ERROR - THREAD PANIC: Partition failed in pcircle function!")
					}
				}
			})
		} else { // Just lines
			self.buffer.prectangle(filled, x, y, width, height, color);
		}
	}

	pub fn pcircle(&mut self, filled: bool, xc: i32, yc: i32, radius: i32, color: Color) {
		let total_area = std::f32::consts::PI * (radius * radius) as f32 ;

		// Run in parallel
		if self.threshold != 0 && total_area >= self.threshold as i32 as f32 {
			scope(|s| {
				let mut join_handles: Vec<ScopedJoinHandle<&mut Buffer>> = Vec::new();
			
				for part in & mut self.partitions {
					//let mut part_clone = part.clone();
	
					let rx = xc - part.offset_x as i32;
					let ry = yc - part.offset_y as i32;
	
					let handle = s.spawn(move || {
	
						part.pcircle(filled, rx, ry, radius, color);
	
						part
					});
					join_handles.push(handle);
				}
	
				for handle in join_handles {
					let part_return = handle.join();
					if part_return.is_ok() {
						let part = part_return.unwrap();
						self.buffer.blit(&part, part.offset_x as i32, part.offset_y as i32);
					} else {
						println!("ERROR - THREAD PANIC: Partition failed in pcircle function!")
					}
				}
			})
			
		} else {
			self.buffer.pcircle(filled, xc, yc, radius, color);
		}
	}

	pub fn pimg(&mut self, image: &Buffer, x: i32, y: i32) {

		let width = image.width;
		let height = image.height;
		// Approximate area, can be bigger depending on rotation
		let total_area = (width as i32) * (height as i32);

		// Run in parallel
		if self.threshold != 0 && total_area >= self.threshold as i32 {
			scope(|s| {
				let mut join_handles: Vec<ScopedJoinHandle<&mut Buffer>> = Vec::new();
			
				for part in &mut self.partitions {

					let rx = x - part.offset_x as i32;
					let ry = y - part.offset_y as i32;


					let handle = s.spawn(move || {
	
						part.pimg(image, rx, ry);
	
						part
					});
					join_handles.push(handle);
				}
	
				for handle in join_handles {
					let part_return = handle.join();
					if part_return.is_ok() {
						let part = part_return.unwrap();
						self.buffer.blit(&part, part.offset_x as i32, part.offset_y as i32);
					} else {
						println!("ERROR - THREAD PANIC: Partition failed in pimg function!")
					}
				}
			})
			
		} else {
			self.buffer.pimg(&image, x, y);
		}
		
	}

	pub fn pimgrect(&mut self, image: &Buffer, x: i32, y: i32, ix: i32, iy: i32, iw: i32, ih: i32) {

		let width = image.width;
		let height = image.height;
		// Approximate area, can be bigger depending on rotation
		let total_area = (width as i32) * (height as i32);

		// Run in parallel
		if self.threshold != 0 && total_area >= self.threshold as i32 {
			scope(|s| {
				let mut join_handles: Vec<ScopedJoinHandle<&mut Buffer>> = Vec::new();
			
				for part in &mut self.partitions {

					let rx = x - part.offset_x as i32;
					let ry = y - part.offset_y as i32;


					let handle = s.spawn(move || {
	
						part.pimgrect(image, rx, ry, ix, iy, iw, ih);
	
						part
					});
					join_handles.push(handle);
				}
	
				for handle in join_handles {
					let part_return = handle.join();
					if part_return.is_ok() {
						let part = part_return.unwrap();
						self.buffer.blit(&part, part.offset_x as i32, part.offset_y as i32);
					} else {
						println!("ERROR - THREAD PANIC: Partition failed in pimg function!")
					}
				}
			})
			
		} else {
			self.buffer.pimg(&image, x, y);
		}
		
	}

	pub fn pimgmtx(&mut self, image: &Buffer, x: f32, y: f32, rotation: f32, scale_x: f32, scale_y: f32, offset_x: f32, offset_y: f32) {

		let width = image.width;
		let height = image.height;
		// Approximate area, can be bigger depending on rotation
		let total_area = (width as f32 * scale_x) * (height as f32 * scale_y);

		// Run in parallel
		if self.threshold != 0 && total_area >= self.threshold as i32 as f32 {
			scope(|s| {

				let mut join_handles: Vec<ScopedJoinHandle<&mut Buffer>> = Vec::new();

				// First pass: Find all regions that contain the image
				for part in &mut self.partitions {
					let rx = x - part.offset_x as f32;
					let ry = y - part.offset_y as f32;
					
					let handle = s.spawn( move || {
						
						part.pimgmtx(image, rx, ry, rotation, scale_x, scale_y, offset_x, offset_y);
	
						part
					});
					join_handles.push(handle);
				}

				for handle in join_handles {
					let part_return = handle.join();
					if part_return.is_ok() {
						let part = part_return.unwrap();
						self.buffer.blit(&part, part.offset_x as i32, part.offset_y as i32);
					} else {
						println!("ERROR - THREAD PANIC: Partition failed in pimgmtx function!")
					}
				}
				
				//self.buffer.prectangle(false, rsx as i32, rsy as i32, (rex - rsx) as i32, (rey - rsy) as i32, Color::blue());
			})
			
		} else {
			self.buffer.pimgmtx(&image, x, y, rotation, scale_x, scale_y, offset_x, offset_y);
		}
		
	}

	pub fn ptritex_uvw(&mut self, x0: i32, y0: i32, 
        x1: i32, y1: i32, 
        x2: i32, y2: i32,
        u0: f32, v0: f32, w0: f32,
        u1: f32, v1: f32, w1: f32,
        u2: f32, v2: f32, w2: f32,
        image: &Buffer) {

		let xmin = i32::clamp(i32::min(x0, i32::min(x1, x2)), 0, self.buffer.width as i32);
		let xmax = i32::clamp(i32::max(x0, i32::max(x1, x2)), 0, self.buffer.width as i32);
		let ymin = i32::clamp(i32::min(y0, i32::min(y1, y2)), 0, self.buffer.height as i32);
		let ymax = i32::clamp(i32::max(y0, i32::max(y1, y2)), 0, self.buffer.height as i32);

		let bb_width: i32 = xmax - xmin;
		let bb_height: i32 = ymax - ymin;

		let total_area = bb_width as u32 * bb_height as u32;

		// Run in parallel
		if self.threshold != 0 && total_area >= self.threshold {
			scope(|s| {

				let mut join_handles: Vec<ScopedJoinHandle<&mut Buffer>> = Vec::new();

				// First pass: Find all regions that contain the image
				for part in &mut self.partitions {
					let rx0 = x0 - part.offset_x as i32;
					let ry0 = y0 - part.offset_y as i32;
					let rx1 = x1 - part.offset_x as i32;
					let ry1 = y1 - part.offset_y as i32;
					let rx2 = x2 - part.offset_x as i32;
					let ry2 = y2 - part.offset_y as i32;
					
					let handle = s.spawn( move || {
						
						part.ptritex_uvw(
							rx0, ry0, rx1, ry1, rx2, ry2,
							u0, v0, w0,
							u1, v1, w1,
							u2, v2, w2, image
						);
	
						part
					});
					join_handles.push(handle);
				}

				for handle in join_handles {
					let part_return = handle.join();
					if part_return.is_ok() {
						let part = part_return.unwrap();
						self.buffer.blit(&part, part.offset_x as i32, part.offset_y as i32);
					} else {
						println!("ERROR - THREAD PANIC: Partition failed in ptritex_uvw function!")
					}
				}
			})
			
		} else {
			self.buffer.ptritex_uvw(x0, y0, x1, y1, x2, y2,
				u0, v0, w0,
				u1, v1, w1,
				u2, v2, w2, image
			);
		}
		
	}


	fn generate_partitions(&mut self) {
		self.partitions.clear();
		/*let mut divx = min_pixel_size;

		for i in min_pixel_size..self.buffer.width {
			if (self.buffer.width % i) == 0 {
				divx = i;
				break;
			} 
		}

		let mut divy = min_pixel_size;

		for i in min_pixel_size..self.buffer.height {
			if (self.buffer.height % i) == 0 {
				divy = i;
				break;
			} 
		}

		let (cx, cy) = (self.buffer.width / divx, self.buffer.height / divy);

		for y in 0..cy {
			for x in 0..cx {
				let mut r = Buffer::new(divx, divy);
				r.offset_x = x * divx;
				r.offset_y = y * divy;
				self.partitions.push(r);
			}
		}*/

		match self.scheme {
			PartitionScheme::Full => { self.partition_full(); },
			PartitionScheme::Split1x2 => { self.partition_split_vertical(); },
			PartitionScheme::Split2x1 => { self.partition_split_horizontal(); },
			PartitionScheme::Split2x2 => { self.partition_split_2x2(); },
			PartitionScheme::Split3x3 => { self.partition_split_3x3(); },
			PartitionScheme::Split3x2 => { self.partition_split_3x2(); },
			PartitionScheme::Split3x1 => { self.partition_split_3x1(); },
			PartitionScheme::Split4x4 => { self.partition_split_4x4(); },
			PartitionScheme::Split5x5 => { self.partition_split_5x5(); },
			PartitionScheme::Split8x8 => { self.partition_split_8x8(); },
		}
	}

	pub fn draw_debug_view(&mut self) {
		for part in &self.partitions {
			self.buffer.pline(
				part.offset_x as i32, 
				part.offset_y as i32, 
				(part.offset_x + part.width)  as i32, 
				part.offset_y  as i32, 
				Color::new(255, 0, 255, 255)
			);

			self.buffer.pline(
				part.offset_x as i32, 
				part.offset_y as i32, 
				part.offset_x  as i32, 
				(part.offset_y + part.height) as i32, 
				Color::new(255, 0, 255, 255)
			);
		}
	}

	fn partition_full(&mut self) {
		self.partitions.push(Buffer::new(self.buffer.width, self.buffer.height));
	}

	fn partition_split_vertical(&mut self) {
		let mut left_half: Buffer = Buffer::new(self.buffer.width / 2, self.buffer.height);
		left_half.offset_x = 0;
		left_half.offset_y = 0;

		let mut right_half: Buffer = Buffer::new(self.buffer.width / 2, self.buffer.height);
		right_half.offset_x = self.buffer.width / 2;
		right_half.offset_y = 0;

		self.partitions.push(left_half);
		self.partitions.push(right_half);
	}

	fn partition_split_horizontal(&mut self) {
		let mut top_half: Buffer = Buffer::new(self.buffer.width, self.buffer.height / 2);
		top_half.offset_x = 0;
		top_half.offset_y = 0;

		let mut bottom_half: Buffer = Buffer::new(self.buffer.width, self.buffer.height / 2);
		bottom_half.offset_x = 0;
		bottom_half.offset_y = self.buffer.height / 2;

		self.partitions.push(top_half);
		self.partitions.push(bottom_half);
	}

	fn partition_split_2x2(&mut self) {
		let cell_x = self.buffer.width / 2;
		let cell_y = self.buffer.height / 2;

		for y in 0..2 {
			for x in 0..2 {
				let mut r = Buffer::new(cell_x, cell_y);
				r.offset_x = cell_x * x;
				r.offset_y = cell_y * y;
				self.partitions.push(r);
			}
		}
	}

	fn partition_split_3x1(&mut self) {
		let cell_x = self.buffer.width / 3;
		let cell_y = self.buffer.height;

		let mut r = Buffer::new(cell_x, cell_y);
		r.offset_x = 0;
		r.offset_y = 0;
		self.partitions.push(r);

		let mut r = Buffer::new(cell_x, cell_y);
		r.offset_x = cell_x;
		r.offset_y = 0;
		self.partitions.push(r);

		let mut r = Buffer::new(cell_x, cell_y);
		r.offset_x = cell_x * 2;
		r.offset_y = 0;
		self.partitions.push(r);
	}

	fn partition_split_3x2(&mut self) {
		let cell_x = self.buffer.width / 3;
		let cell_y = self.buffer.height / 2;

		for y in 0..2 {
			for x in 0..3 {
				let mut r = Buffer::new(cell_x, cell_y);
				r.offset_x = cell_x * x;
				r.offset_y = cell_y * y;
				self.partitions.push(r);
			}
		}
	}

	fn partition_split_3x3(&mut self) {
		let cell_x = self.buffer.width / 3;
		let cell_y = self.buffer.height / 3;

		for y in 0..3 {
			for x in 0..3 {
				let mut r = Buffer::new(cell_x, cell_y);
				r.offset_x = cell_x * x;
				r.offset_y = cell_y * y;
				self.partitions.push(r);
			}
		}
	}

	fn partition_split_4x4(&mut self) {
		let cell_x = self.buffer.width / 4;
		let cell_y = self.buffer.height / 4;

		for y in 0..4 {
			for x in 0..4 {
				let mut r = Buffer::new(cell_x, cell_y);
				r.offset_x = cell_x * x;
				r.offset_y = cell_y * y;
				self.partitions.push(r);
			}
		}
	}

	fn partition_split_5x5(&mut self) {
		let cell_x = self.buffer.width / 5;
		let cell_y = self.buffer.height / 5;

		for y in 0..5 {
			for x in 0..5 {
				let mut r = Buffer::new(cell_x, cell_y);
				r.offset_x = cell_x * x;
				r.offset_y = cell_y * y;
				self.partitions.push(r);
			}
		}
	}

	fn partition_split_8x8(&mut self) {
		let cell_x = self.buffer.width / 8;
		let cell_y = self.buffer.height / 8;

		for y in 0..8 {
			for x in 0..8 {
				let mut r = Buffer::new(cell_x, cell_y);
				r.offset_x = cell_x * x;
				r.offset_y = cell_y * y;
				self.partitions.push(r);
			}
		}
	}
}