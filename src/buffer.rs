use rayon::prelude::*;

use crate::color::*;
use crate::math;
use crate::partitioned_buffer::PartitionedBuffer;
use crate::vector2::*;
use crate::matrix3::*;
use crate::font::*;
use crate::math::*;

// Draw Mode Definition
pub type PSetOp = fn(&mut Buffer, usize, Color);

/// Controls how a Buffer should draw incoming pixels.
#[derive(Debug, Clone, Copy)]
pub enum DrawMode {
    NoOp,
    NoAlpha,
    Opaque,
    Alpha,
    Addition,
    Subtraction,
    Multiply,
    Divide,
    ForceTint,
    InvertedAlpha,
    InvertedOpaque,
    InvertedBgAlpha,
    InvertedBgOpaque,
    // Collect,
}

fn pset_noop(buffer: &mut Buffer, idx: usize, color: Color) {
    buffer.color[idx + 0] = color.r;  // R
    buffer.color[idx + 1] = color.g;  // G
    buffer.color[idx + 2] = color.b;  // B
    buffer.color[idx + 3] = color.a;  // A
    buffer.drawn_pixels_since_clear += 1;
}

fn pset_noalpha(buffer: &mut Buffer, idx: usize, color: Color) {
    let color = color * buffer.tint;
    buffer.color[idx + 0] = color.r;  // R
    buffer.color[idx + 1] = color.g;  // G
    buffer.color[idx + 2] = color.b;  // B
    buffer.color[idx + 3] = color.a;  // A
    buffer.drawn_pixels_since_clear += 1;
}

/// Draw pixels if they are fully opaque, otherwise ignore them.
fn pset_opaque(buffer: &mut Buffer, idx: usize, color: Color) {
    if color.a < 255 { return; }
    let color = color * buffer.tint;
    buffer.color[idx + 0] = color.r;  // R
    buffer.color[idx + 1] = color.g;  // G
    buffer.color[idx + 2] = color.b;  // B
    buffer.color[idx + 3] = color.a;  // A
    buffer.drawn_pixels_since_clear += 1;
}

/// Draw pixels if they are fully opaque, otherwise ignore them. Forces them to be the tint color.
/// Useful for flashes or making masks
fn pset_force_tint(buffer: &mut Buffer, idx: usize, color: Color) {
    if color.a < 255 { return; }
    buffer.color[idx + 0] = buffer.tint.r;  // R
    buffer.color[idx + 1] = buffer.tint.g;  // G
    buffer.color[idx + 2] = buffer.tint.b;  // B
    buffer.color[idx + 3] = buffer.tint.a;  // A
    buffer.drawn_pixels_since_clear += 1;
}

/// Draw pixels and blend them with the background based on the alpha channel
fn pset_alpha(buffer: &mut Buffer, idx: usize, color: Color) {
    
    let fg = color * buffer.tint;
    let bg = Color::new(
        buffer.color[idx + 0],
        buffer.color[idx + 1],
        buffer.color[idx + 2],
        255,
    );

    let c = Color::blend_fast(fg, bg, buffer.opacity);

    buffer.color[idx + 0] = c.r;  // R
    buffer.color[idx + 1] = c.g;  // G
    buffer.color[idx + 2] = c.b;  // B
    buffer.color[idx + 3] = c.a;  // A
    buffer.drawn_pixels_since_clear += 1;
}

/// Add incoming and buffer pixels together and draw to screen
fn pset_addition(buffer: &mut Buffer, idx: usize, color: Color) {
    if color.a <= 0 { return; }
    let fg = color * buffer.tint;
    let bg = Color::new(
        buffer.color[idx + 0],
        buffer.color[idx + 1],
        buffer.color[idx + 2],
        255,
    );

    let c = Color::blend_fast(fg, bg, buffer.opacity) + bg;

    buffer.color[idx + 0] = c.r;  // R
    buffer.color[idx + 1] = c.g;  // G
    buffer.color[idx + 2] = c.b;  // B
    buffer.color[idx + 3] = c.a;  // A
    buffer.drawn_pixels_since_clear += 1;
}

/// Multiply incoming pixel with buffer pixel.
fn pset_multiply(buffer: &mut Buffer, idx: usize, color: Color) {
    if color.a <= 0 { return; }
    let fg = color * buffer.tint;
    let bg = Color::new(
        buffer.color[idx + 0],
        buffer.color[idx + 1],
        buffer.color[idx + 2],
        255,
    );

    let c = Color::blend_fast(fg.inverted(), bg, buffer.opacity) * bg;

    buffer.color[idx + 0] = c.r;  // R
    buffer.color[idx + 1] = c.g;  // G
    buffer.color[idx + 2] = c.b;  // B
    buffer.color[idx + 3] = c.a;  // A
    buffer.drawn_pixels_since_clear += 1;
}

/// Draw inverted copy of incoming pixel with alpha blending
fn pset_inverted_alpha(buffer: &mut Buffer, idx: usize, color: Color) {
    if color.a <= 0 { return; }
    let fg = color* buffer.tint;
    let bg = Color::new(
        buffer.color[idx + 0],
        buffer.color[idx + 1],
        buffer.color[idx + 2],
        255,
    );

    let c = Color::blend_fast(fg.inverted(), bg, buffer.opacity);

    buffer.color[idx + 0] = c.r;  // R
    buffer.color[idx + 1] = c.g;  // G
    buffer.color[idx + 2] = c.b;  // B
    buffer.color[idx + 3] = c.a;  // A
    buffer.drawn_pixels_since_clear += 1;
}

/// Draw inverted copy of incoming pixel as opaque
fn pset_inverted_opaque(buffer: &mut Buffer, idx: usize, color: Color) {
    if color.a < 255 { return; }

    let bg = Color::new(
        buffer.color[idx + 0],
        buffer.color[idx + 1],
        buffer.color[idx + 2],
        255,
    );

    let c = (bg * buffer.tint).inverted();

    buffer.color[idx + 0] = c.r;  // R
    buffer.color[idx + 1] = c.g;  // G
    buffer.color[idx + 2] = c.b;  // B
    buffer.color[idx + 3] = c.a;  // A
    buffer.drawn_pixels_since_clear += 1;
}

/// Draw inverted copy of incoming pixel as opaque
fn pset_inverted_bg_opaque(buffer: &mut Buffer, idx: usize, color: Color) {
    if color.a < 255 { return; }

    let bg = Color::new(
        buffer.color[idx + 0],
        buffer.color[idx + 1],
        buffer.color[idx + 2],
        255,
    );

    let c = (bg * buffer.tint).inverted();

    buffer.color[idx + 0] = c.r;  // R
    buffer.color[idx + 1] = c.g;  // G
    buffer.color[idx + 2] = c.b;  // B
    buffer.color[idx + 3] = c.a;  // A
    buffer.drawn_pixels_since_clear += 1;
}

/// Draw inverted copy of incoming pixel as opaque
fn pset_inverted_bg_alpha(buffer: &mut Buffer, idx: usize, color: Color) {
    if color.a < 255 { return; }

    let bg = Color::new(
        buffer.color[idx + 0],
        buffer.color[idx + 1],
        buffer.color[idx + 2],
        255,
    );

    let c = Color::blend_fast(bg, (bg * buffer.tint).inverted(), buffer.opacity);

    buffer.color[idx + 0] = c.r;  // R
    buffer.color[idx + 1] = c.g;  // G
    buffer.color[idx + 2] = c.b;  // B
    buffer.color[idx + 3] = c.a;  // A
    buffer.drawn_pixels_since_clear += 1;
}

/// Image in memory with operations to modify it. Pixel modification functions are 
#[derive(Clone)]
pub struct Buffer {
    pset_op: PSetOp,

    // For Partitioned Buffer
    pub offset_x: usize,
    pub offset_y: usize,
    
    pub width: usize,
    pub height: usize,
    pub color: Vec<u8>,

    pub camera_position: Vector2,
    pub camera_rotation: f32,
    pub camera_scale: Vector2,
    pub camera_matrix: Matrix3,

    pub draw_mode: DrawMode,
    pub tint: Color,
    pub opacity: u8,

    pub drawn_pixels_since_clear: u32,
}

impl Buffer {

    /// Makes a new Buffer to draw to a screen-sized buffer
    ///
    /// # Arguments
    /// * 'width' - Horizontal size of the 
    /// * 'height' - Vertical size of the 
    pub fn new(width: usize, height: usize) -> Buffer {
        //println!("Buffer: {} x {} x {}, Memory: {}B", width, height, 4, (width * height * 4));
        Buffer {
            pset_op: pset_opaque,

            offset_x: 0,
            offset_y: 0,

            width,
            height,
            color: vec![0; width * height * 4],

            camera_position: Vector2::ZERO,
            camera_rotation: 0.0,
            camera_scale: Vector2::ONE,
            camera_matrix: Matrix3::identity(),

            draw_mode: DrawMode::Opaque,
            tint: Color::WHITE,
            opacity: 255,

            drawn_pixels_since_clear: 0,
        }
    }

    pub fn new_from_image(path_to: &str) -> Result<Buffer, String> {
		match lodepng::decode32_file(path_to) {
			Ok(image) => {
				//println!("Image: {}, Res: {} x {}, Size: {}B", path_to, image.width, image.height, image.buffer.len());
				//let buffer_new: Vec<u8> =  image.buffer.as_bytes().to_vec();
                use rgb::*;

                // Convert to atomics for parallelism

				Ok(Buffer {
                    pset_op: pset_opaque,

                    width: image.width,
                    height: image.height,
                    color: image.buffer.as_bytes().to_vec(),

                    camera_position: Vector2::ZERO,
                    camera_rotation: 0.0,
                    camera_scale: Vector2::ONE,
                    camera_matrix: Matrix3::identity(),

                    draw_mode: DrawMode::Opaque,
                    tint: Color::WHITE,
                    opacity: 255,

                    offset_x: 0,
                    offset_y: 0,

                    drawn_pixels_since_clear: 0,
                })
			},
			Err(reason) => {
				println!("ERROR - IMAGE: Could not load {} | {}", path_to, reason);
				Ok(Buffer::default())
			}
		}
    }

    /// Clears the framebuffer and changes its width and height to new values.
    pub fn resize(&mut self, width: usize, height: usize) {
        self.width = width;
        self.height = height;
        self.color = vec![0; width * height * 4];
    }

    /// Sets the Buffers drawing mode for incoming pixels. Should be defined before every drawing operation.
    /// # Arguments
    /// * 'mode' - Which drawing function should the Buffer use.
    pub fn set_draw_mode(&mut self, mode: DrawMode) {
        match mode {
            DrawMode::NoOp                  => {self.pset_op = pset_noop;}
            DrawMode::NoAlpha               => {self.pset_op = pset_noalpha;}
            DrawMode::Opaque                => {self.pset_op = pset_opaque;},
            DrawMode::Alpha                 => {self.pset_op = pset_alpha;},
            DrawMode::Addition              => {self.pset_op = pset_addition;},
            DrawMode::Multiply              => {self.pset_op = pset_multiply;}
            DrawMode::ForceTint             => {self.pset_op = pset_force_tint;}
            DrawMode::InvertedAlpha         => {self.pset_op = pset_inverted_alpha;}
            DrawMode::InvertedOpaque        => {self.pset_op = pset_inverted_opaque;}
            DrawMode::InvertedBgOpaque      => {self.pset_op = pset_inverted_bg_opaque;}
            DrawMode::InvertedBgAlpha       => {self.pset_op = pset_inverted_bg_alpha;}
            _ => {},
        }
    }

    pub fn into_partitioned(&self) -> PartitionedBuffer {
        let mut pr = PartitionedBuffer::new(self.width, self.height, 0);
        pr.buffer.blit(self, 0, 0);
        pr
    }

    /// Create a copy of a region 
    /*pub fn blit_copy(&self, x: i32, y: i32, width: usize, height: usize) -> Buffer {
        let stride = 4;

        let mut Buffer: Buffer = Buffer::new(width, height);

        // Go down in rows. i is the current row.
        self.color.chunks_exact(self.width * 4).enumerate().for_each(|(row_idx, pixel)| {
            if (row_idx as i32) > y && (row_idx as i32) <= (y + height as i32) {
                //let sx = i32::clamp(x, 0, self.width)l
            }
        });
    }*/

    pub fn blit(&mut self, src: &Buffer, x: i32, y: i32) {
        let is_equal_size: bool = self.width == src.width && self.height == src.height;
        if is_equal_size {
            self.color.copy_from_slice(&src.color);
            return;
        }

        let stride = 4;
        
        // The color array is a 1D row of bytes, so we have to do this in sets of rows
        // Make sure this actually fits inside the buffer
        let extent_width: usize = x as usize + src.width;
        let extent_height: usize = y as usize + src.height;
    
        // If this goes out of bounds at all we should not draw it. Otherwise it WILL panic.
        let out_of_bounds: bool = extent_width > self.width || extent_height > self.height || x < 0 || y < 0;
        if out_of_bounds { 
            return;
        }
    
        // Lets get an array of rows so we can blit them directly into the color buffer
        let mut rows_src: Vec<&[u8]> = Vec::with_capacity(src.height);
    
        // Build a list of rows to blit to the screen.
        src.color.chunks_exact(src.width * stride).enumerate().for_each(|(_, row)| {
            rows_src.push(row);
        });
    
        // Goes through each row of fbuf and split it twice into the slice that fits our rows_src.
        self.color.chunks_exact_mut(self.width * stride).enumerate().for_each(|(i, row_dst)| {
            // We need to cut the row into a section that we can just set equal to our row
            // Make sure that we are actually in the bounds from our source buffer
            if i >= y as usize as usize && i < (y as usize + src.height) {
                // [......|#######]
                // Split at the stride distance to get the first end
                let rightsect = row_dst.split_at_mut(x as usize * stride).1;

                // [......|####|...]
                // Get the second half but left
                let section = rightsect.split_at_mut((extent_width - x as usize) * stride).0;

                // I HAVE YOU NOW
                section.copy_from_slice(rows_src[i-y as usize]);
            }
        });
    }

    /// Clears the frame memory directly, leaving a black screen.
    pub fn clear(&mut self) {
        self.color = vec![0; self.width * self.height * 4];
        self.drawn_pixels_since_clear = 0;
    }

    /// Clears the screen to a color.
    /// # Arguments
    /// * 'color' - Color the screen should be cleared too.
    pub fn clear_color(&mut self, color: Color) {
        // Check if the amount of work is worth parallelizing
        if self.color.len() > 262144 {
            self.color.par_chunks_exact_mut(4).for_each(|c| {
                c[0] = color.r;
                c[1] = color.g;
                c[2] = color.b;
                c[3] = color.a;
            });
        } else {
            self.color.chunks_exact_mut(4).for_each(|c| {
                c[0] = color.r;
                c[1] = color.g;
                c[2] = color.b;
                c[3] = color.a;
            });
        }

        self.drawn_pixels_since_clear = 0;
    }

    pub fn tint_buffer(&mut self, color: Color) {
        // Check if the amount of work is worth parallelizing
        if self.color.len() > 262144 {
            self.color.par_chunks_exact_mut(4).for_each(|c| {
                let color: Color = Color { r: c[0], g: c[1], b: c[2], a: c[3] } * color;
    
                c[0] = color.r;
                c[1] = color.g;
                c[2] = color.b;
                c[3] = color.a;
            });
        } else {
            self.color.chunks_exact_mut(4).for_each(|c| {
                let color: Color = Color { r: c[0], g: c[1], b: c[2], a: c[3] } * color;
    
                c[0] = color.r;
                c[1] = color.g;
                c[2] = color.b;
                c[3] = color.a;
            });
        }

        
    }

    pub fn update_camera(&mut self) {
        // Camera is usually in the top left corner so we need to change the zoom scaling so it fits in the middle of the screen
        let camera_offset: Vector2 = Vector2::new(
            -lerpf(0.0, self.width as f32, 0.5),
            -lerpf(0.0, self.height as f32, 0.5),
        );

        let camera_mtx_o = Matrix3::translated(camera_offset);
        let camera_mtx_r = Matrix3::rotated(self.camera_rotation);
        let camera_mtx_p = Matrix3::translated(-self.camera_position + Vector2::new(self.width as f32 / 2.0, self.height as f32 / 2.0));
        let camera_mtx_s = Matrix3::scaled(self.camera_scale);

        // Combine matricies using matrix multiplication
        self.camera_matrix = camera_mtx_p * camera_mtx_r * camera_mtx_s * camera_mtx_o;
    }

    /// Draws a pixel to the color buffer, using the Buffers set DrawMode. DrawMode defaults to Opaque.
    pub fn pset(&mut self, x: i32, y: i32, color: Color) {
        self.drawn_pixels_since_clear += 1;
        //let x = x.rem_euclid(self.width as i32);
        //let y = y.rem_euclid(self.height as i32);
        let idx: usize = ((y * (self.width as i32) + x) * 4) as usize;

        let out_left: bool = x < 0;
        let out_right: bool = x > (self.width) as i32 - 1;
        let out_top: bool = y < 0;
        let out_bottom: bool = y > (self.height) as i32 - 1;
        let out_of_range: bool = idx > (self.width * self.height * 4) - 1;

        if out_of_range || out_left || out_right || out_top || out_bottom  { return; }

        // We have to put paraenthesis around the fn() variables or else the compiler will think it's a method.
        (self.pset_op)(self, idx, color);
    }

    /// Draws a pixel to the color buffer, using the Buffers set DrawMode. DrawMode defaults to Opaque.
    /// This variant of pset has no array bounds protections and will trigger a panic if a pixel is placed
    /// outside of the buffer length.
    pub fn pset_panic_oob(&mut self, x: i32, y: i32, color: Color) {
        self.drawn_pixels_since_clear += 1;

        let idx: usize = ((y * (self.width as i32) + x) * 4) as usize;


        // We have to put paraenthesis around the fn() variables or else the compiler will think it's a method.
        (self.pset_op)(self, idx, color);
    }

    /// Gets a color from the color buffer.
    pub fn pget(&self, x: i32, y: i32) -> Color {

        let idx: usize = ((y * (self.width as i32) + x) * 4) as usize;

        let out_left: bool = x < 0;
        let out_right: bool = x > (self.width) as i32 - 1;
        let out_top: bool = y < 0;
        let out_bottom: bool = y > (self.height) as i32 - 1;
        let out_of_range: bool = idx > (self.width * self.height * 4) - 1;

        if out_of_range || out_left || out_right || out_top || out_bottom  { return Color::CLEAR; }

        return Color::new(
            self.color[idx + 0],
            self.color[idx + 1],
            self.color[idx + 2],
            self.color[idx + 3]
        );
    }

    /// Gets a color from the color buffer.
    pub fn pget_wrap(&self, x: i32, y: i32) -> Color {
        let x = x.rem_euclid(self.width as i32);
        let y = y.rem_euclid(self.height as i32);

        let idx: usize = ((y * (self.width as i32) + x) * 4) as usize;

        return Color::new(
            self.color[idx + 0],
            self.color[idx + 1],
            self.color[idx + 2],
            self.color[idx + 3]
        );
    }
    
    /// Draws a line across two points using Brensenham Line algorithm from Wikipedia
    pub fn pline(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, color: Color) {
        let (mut x0, mut y0) = (x0, y0);

        let dx = i32::abs(x1 - x0);
        let sx = if x0 < x1 {1} else {-1};
        let dy = -i32::abs(y1 - y0);
        let sy = if y0 < y1 {1} else {-1};
        let mut error = dx + dy;
        
        loop {
            self.pset(x0, y0, color);
            if x0 == x1 && y0 == y1 { break; }
            let e2 = 2 * error;
            if e2 >= dy {
                if x0 == x1 { break; }
                error = error + dy;
                x0 = x0 + sx;
            }
            if e2 <= dx {
                if y0 == y1 { break; }
                error = error + dx;
                y0 = y0 + sy;
            }
        }
    }

    /// Draws a line across two points using Brensenham Line algorithm from Wikipedia
    pub fn plinetex(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, u: i32, v: i32, du: i32, dv: i32, buffer: &Buffer) {
        let (mut x0, mut y0) = (x0, y0);
        let (mut u, mut v) = (u, v);

        let dx = i32::abs(x1 - x0);
        let sx = if x0 < x1 {1} else {-1};
        let dy = -i32::abs(y1 - y0);
        let sy = if y0 < y1 {1} else {-1};
        let mut error = dx + dy;
        
        loop {
            let color = buffer.pget_wrap(u, v);
            self.pset(x0, y0, color);

            if x0 == x1 && y0 == y1 { break; }
            let e2 = 2 * error;
            if e2 >= dy {
                if x0 == x1 { break; }
                error = error + dy;
                x0 = x0 + sx;
            }
            if e2 <= dx {
                if y0 == y1 { break; }
                error = error + dx;
                y0 = y0 + sy;
            }

            u += du;
            v += dv;
        }
    }
    
    /// Draws a rectangle onto the screen. Can either be filled or outlined.
    pub fn prectangle(&mut self, filled: bool, x: i32, y: i32, w: i32, h: i32, color: Color) {

    
        if filled {
            let x0 = i32::clamp(x, 0, self.width as i32);
            let x1 = i32::clamp(x + w, 0, self.width as i32);
            let y0 = i32::clamp(y, 0, self.height as i32);
            let y1 = i32::clamp(y + h, 0, self.height as i32);

            for py in y0..y1 {
                for px in x0..x1 {
                    self.pset_panic_oob(px, py, color);
                }
            }
        } else {
            self.pline(x, y, x + w, y, color);
            self.pline(x, y + h, x + w, y + h, color);

            self.pline(x + w, y, x + w, y + h, color);
            self.pline(x, y, x, y + h, color);
        }
    }

    /// Draws a triangle directly to the screen.
    /// Implementation found here: https://stackoverflow.com/questions/2049582/how-to-determine-if-a-point-is-in-a-2d-triangle
    pub fn ptriangle(&mut self, filled: bool, x0: i32, y0: i32, x1: i32, y1: i32, x2: i32, y2: i32, color: Color) {
        if filled {
            let xmin = i32::clamp(i32::min(x0, i32::min(x1, x2)), 0, self.width as i32);
            let xmax = i32::clamp(i32::max(x0, i32::max(x1, x2)), 0, self.width as i32);
            let ymin = i32::clamp(i32::min(y0, i32::min(y1, y2)), 0, self.width as i32);
            let ymax = i32::clamp(i32::max(y0, i32::max(y1, y2)), 0, self.width as i32);

            for iy in ymin..ymax {
                for ix in xmin..xmax {

                    let d1 = math::sign3i(ix, iy, x0, y0, x1, y1);
                    let d2 = math::sign3i(ix, iy, x1, y1, x2, y2);
                    let d3 = math::sign3i(ix, iy, x2, y2, x0, y0);

                    let has_neg = (d1 < 0) || (d2 < 0) || (d3 < 0);
                    let has_pos = (d1 > 0) || (d2 > 0) || (d3 > 0);

                    let is_inside: bool = !(has_neg && has_pos);

                    if is_inside {
                        self.pset_panic_oob(ix, iy, color);
                    }
                }
            }
        } else {
            self.pline(x0, y0, x1, y1, color);
            self.pline(x0, y0, x2, y2, color);
            self.pline(x1, y1, x2, y2, color);
        }
    }
    
    /// Draws a circle onto the screen. Can either be filled or outlined.
    pub fn pcircle(&mut self, filled: bool, xc: i32, yc: i32, r: i32, color: Color) { 

        let minx = i32::clamp(xc - r, 0, self.width  as i32);
        let maxx = i32::clamp((xc + r) + 1, 0, self.width  as i32);
        let miny = i32::clamp(yc - r, 0, self.height as i32);
        let maxy = i32::clamp((yc + r)+1, 0, self.height as i32);

        if filled {
            for py in miny..maxy {
                for px in minx..maxx {
                    if ((px - xc) * (px - xc)) + ((py - yc) * (py - yc)) <= r * r {
                        self.pset(px, py, color);
                    }
                }
            }
        } else {
            let mut x: i32 = 0;
            let mut y: i32 = r; 
            let mut d: i32 = 3 - 2 * r;
            
            self.pset(xc+x, yc+y, color); 
            self.pset(xc-x, yc+y, color);
            self.pset(xc+x, yc-y, color); 
            self.pset(xc-x, yc-y, color); 
            self.pset(xc+y, yc+x, color);
            self.pset(xc-y, yc+x, color);
            self.pset(xc+y, yc-x, color); 
            self.pset(xc-y, yc-x, color);
    
            while y >= x
            { 
                x += 1; 
          
                if d > 0  { 
                    y -= 1;  
                    d = d + 4 * (x - y) + 10; 
                } else {
                    d = d + 4 * x + 6;
                } 
                self.pset(xc+x, yc+y, color); 
                self.pset(xc-x, yc+y, color);
                self.pset(xc+x, yc-y, color); 
                self.pset(xc-x, yc-y, color); 
                self.pset(xc+y, yc+x, color);
                self.pset(xc-y, yc+x, color);
                self.pset(xc+y, yc-x, color); 
                self.pset(xc-y, yc-x, color);
            }   
        }
    }

    /// Draws an image directly to the screen.
    pub fn pimg(&mut self, image: &Buffer, x: i32, y: i32) {
        for ly in 0..image.height as i32 {
            for lx in 0..image.width as i32 {
                let pc = image.pget(lx, ly);
                let px = x + lx;
                let py = y + ly;
                
                // Pixel out of bounds
                if pc.a <= 0 || (px < 0 || px > self.width as i32) || (py < 0 || py > self.height as i32) { continue; }

                self.pset(x + lx, y + ly, pc);
            }
        }
    }

    /// Draws a section of an image directly to the screen.
    pub fn pimgrect(&mut self, image: &Buffer, x: i32, y: i32, rx: i32, ry: i32, rw: i32, rh: i32) {
        let range_x = i32::clamp(rx + rw, 0, self.width as i32);
        let range_y = i32::clamp(ry + rh, 0, self.height as i32);
        for ly in ry..range_y {
            for lx in rx..range_x {
                let mlx = lx.rem_euclid(image.width as i32);
                let mly = ly.rem_euclid(image.height as i32);

                let px: i32 = (x + mlx as i32) - rx as i32;
                let py: i32 = (y + mly as i32) - ry as i32;

                self.pset(px, py, image.pget(mlx as i32, mly as i32));
            }
        }
    }

    /// Draws a rotated and scaled image to the screen using matrix multiplication.
    pub fn pimgmtx(&mut self, image: &Buffer, position_x: f32, position_y: f32, rotation: f32, scale_x: f32, scale_y: f32, offset_x: f32, offset_y: f32) {

        // Early out if the image is going to be too small to draw
        let area_x = image.width as f32 * scale_x;
        let area_y = image.height as f32 * scale_y;

        if area_x * area_y < 1.0 {
            return;
        }

        let offset_x = -lerpf(0.0, image.width as f32, offset_x);
        let offset_y = -lerpf(0.0, image.height as f32, offset_y);

        let position: Vector2 = Vector2::new(position_x, position_y);
        let offset: Vector2 = Vector2::new(offset_x, offset_y);
        let scale: Vector2 = Vector2::new(scale_x, scale_y);
        

        // Get sprite matrix setup
        let mtx_o = Matrix3::translated(offset);
        let mtx_r = Matrix3::rotated(rotation);
        let mtx_p = Matrix3::translated(position);
        let mtx_s = Matrix3::scaled(scale);

        let smtx = mtx_p * mtx_r * mtx_s * mtx_o;

        // Combine camera matrix with sprite matrix
        let cmtx = self.camera_matrix * smtx;

        // We have to get the rotated bounding box of the rotated sprite in order to draw it correctly without blank pixels
        let start_center: Vector2 = cmtx.forward(Vector2::ZERO);
        let (mut sx, mut sy, mut ex, mut ey) = (start_center.x, start_center.y, start_center.x, start_center.y);

        // Top-Left Corner
        let p1: Vector2 = cmtx.forward(Vector2::ZERO);
        sx = f32::min(sx, p1.x); sy = f32::min(sy, p1.y);
        ex = f32::max(ex, p1.x); ey = f32::max(ey, p1.y);

        // Bottom-Right Corner
        let p2: Vector2 = cmtx.forward(Vector2::new(image.width as f32, image.height as f32));
        sx = f32::min(sx, p2.x); sy = f32::min(sy, p2.y);
        ex = f32::max(ex, p2.x); ey = f32::max(ey, p2.y);

        // Bottom-Left Corner
        let p3: Vector2 = cmtx.forward(Vector2::new(0.0, image.height as f32));
        sx = f32::min(sx, p3.x); sy = f32::min(sy, p3.y);
        ex = f32::max(ex, p3.x); ey = f32::max(ey, p3.y);

        // Top-Right Corner
        let p4: Vector2 = cmtx.forward(Vector2::new(image.width as f32, 0.0));
        sx = f32::min(sx, p4.x); sy = f32::min(sy, p4.y);
        ex = f32::max(ex, p4.x); ey = f32::max(ey, p4.y);

        // Extend the bounding box by a few pixels to catch clipping errors
        let mut rsx = sx as i32;
        let mut rsy = sy as i32;
        let mut rex = ex as i32+1;
        let mut rey = ey as i32+1;

        // Sprite isn't even in frame, don't draw anything
        if (rex < 0 || rsx > self.width as i32) && (rey < 0 || rsy > self.height as i32) { return; }

        // Okay but clamp the ranges in frame so we're not wasting time on stuff offscreen

        rsx = i32::clamp(rsx, 0, self.width as i32);
        rsy = i32::clamp(rsy, 0, self.height as i32);
        rex = i32::clamp(rex, 0, self.width as i32);
        rey = i32::clamp(rey, 0, self.height as i32);

        let cmtx_inv = cmtx.clone().inv();

		// We can finally draw!
        for ly in rsy..rey {
            for lx in rsx..rex {
                // We have to use the inverted compound matrix (cmtx_inv) in order to get the correct pixel data from the image.
                let ip: Vector2 = cmtx_inv.forward(Vector2::new(lx as f32, ly as f32));

                // Ceil the transformed pixel positions to fix the colot pullingg
                let color: Color = image.pget(f32::ceil(ip.x) as i32, f32::ceil(ip.y) as i32);

                // We skip drawing entirely if the alpha is zero.
                // Otherwise leaves weird grey box
                if color.a <= 0 { continue; }
                self.pset(lx as i32, ly as i32, color);
            }
        }
    }

    /// Draws a triangle directly to the screen, using a texture.
    pub fn ptritex_wrap(&mut self,
        x0: i32, y0: i32, 
        x1: i32, y1: i32, 
        x2: i32, y2: i32,
        image: &Buffer) {

        let xmin = i32::clamp(i32::min(x0, i32::min(x1, x2)), 0, self.width as i32);
        let xmax = i32::clamp(i32::max(x0, i32::max(x1, x2)), 0, self.width as i32);
        let ymin = i32::clamp(i32::min(y0, i32::min(y1, y2)), 0, self.height as i32);
        let ymax = i32::clamp(i32::max(y0, i32::max(y1, y2)), 0, self.height as i32);

        for iy in ymin..ymax {
            for ix in xmin..xmax {

                let d1 = math::sign3i(ix, iy, x0, y0, x1, y1);
                let d2 = math::sign3i(ix, iy, x1, y1, x2, y2);
                let d3 = math::sign3i(ix, iy, x2, y2, x0, y0);

                let has_neg = (d1 < 0) || (d2 < 0) || (d3 < 0);
                let has_pos = (d1 > 0) || (d2 > 0) || (d3 > 0);

                let is_inside: bool = !(has_neg && has_pos);

                if is_inside {
                    // Get where this point would be if transfered to the uv triangle

                    // Get interpolation percent between points
                    let uv_x = ix;
                    let uv_y = iy;

                    self.pset(ix, iy, image.pget_wrap(uv_x, uv_y));
                }
            }
        }
    }

    /// Draws text directly to the screen using a provided font.
    pub fn pprint(&mut self, font: &Font, text: String, x: i32, y: i32, newline_space: i32, wrap_width: Option<u32>) {
        let mut jumpx: i32 = 0;
        let mut jumpy: i32 = 0;
        let chars: Vec<char> = text.chars().collect();

        for i in 0..chars.len() {
            
            if chars[i] == '\n' { jumpy += font.glyph_height as i32 + newline_space; jumpx = 0; continue; }
            if chars[i] == ' ' { jumpx += font.glyph_width as i32; continue; }
            for j in 0..font.glyphidx.len() {
                if font.glyphidx[j] == chars[i] {
                    let rectx: i32 = (j as i32 * font.glyph_width as i32) % (font.fontimg.width as i32);
                    let recty: i32 = ((j as i32 * font.glyph_width as i32) / font.fontimg.width as i32) * font.glyph_height as i32;
                    let rectw: i32 = font.glyph_width as i32;
                    let recth: i32 = font.glyph_height as i32;
                    
                    self.pimgrect(&font.fontimg, x + jumpx as i32, y + jumpy as i32, rectx, recty, rectw, recth);
                    

                    jumpx += font.glyph_width as i32 + font.glyph_spacing as i32;
                }
            }
            if wrap_width.is_some() && (jumpx as u32) > wrap_width.unwrap() { jumpy += font.glyph_height as i32 + newline_space; jumpx = 0; }
        }
    }

    /// Draws a quadratic beizer curve onto the screen.
    pub fn pbeizer(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, mx: i32, my: i32, color: Color) {
        let mut step: f32 = 0.0;

        // Get the maximal number of pixels we will need to use and get its inverse as a step size.
        // Otherwise we don't know how many pixels we will need to draw
        let stride_c1 = self.cline(x0, y0, mx, my) as f32;
        let stride_c2 = self.cline(mx, my, x1, y1) as f32;

        let stride: f32 = (1.0 / (stride_c1 + stride_c2)) * 0.5;

        let x0 = x0 as f32;
        let y0 = x0 as f32;
        let x1 = x1 as f32;
        let y1 = y1 as f32;
        let mx = mx as f32;
        let my = my as f32;

        loop {
            if step > 1.0 { break; }

            let px0 = f32::clamp(lerpf(x0, mx, step), 0.0, self.width as f32);
            let py0 = f32::clamp(lerpf(y0, my, step), 0.0, self.height as f32);

            let px1 = f32::clamp(lerpf(px0, x1, step), 0.0, self.width as f32);
            let py1 = f32::clamp(lerpf(py0, y1, step), 0.0, self.height as f32);

            self.pset(px1 as i32, py1 as i32, color);
            step += stride;
        }
    }

    pub fn pcomposite_opaque(&mut self, buffer: &Buffer) {
        if self.color.len() != buffer.color.len() { return; }

        self.color.par_chunks_exact_mut(4).zip(buffer.color.par_chunks_exact(4)).for_each(|(c1, c2)| {
            let dst = Color::new(c2[0], c2[1], c2[2], c2[3]);

            if c2[3] >= 255 { 
                c1[0] = dst.r;
                c1[1] = dst.g;
                c1[2] = dst.b;
                c1[3] = 255;
            }

            
        });
    }

    pub fn pcomposite_alpha(&mut self, buffer: &Buffer, opacity: u8) {
        if self.color.len() != buffer.color.len() { return; }

        self.color.par_chunks_exact_mut(4).zip(buffer.color.par_chunks_exact(4)).for_each(|(c1, c2)| {
            let src = Color::new(c2[0], c2[1], c2[2], c2[3]);
            let dst = Color::new(c1[0], c1[1], c1[2], c1[3]);

            let fc = Color::blend_fast(dst, src, 255 - opacity);

            c1[0] = fc.r;
            c1[1] = fc.g;
            c1[2] = fc.b;
            c1[3] = fc.a;
        });
    }

    pub fn pcomposite_multiply(&mut self, buffer: &Buffer) {
        if self.color.len() != buffer.color.len() { return; }

        self.color.par_chunks_exact_mut(4).zip(buffer.color.par_chunks_exact(4)).for_each(|(c1, c2)| {
            let src = Color::new(c2[0], c2[1], c2[2], c2[3]);
            let dst = Color::new(c1[0], c1[1], c1[2], c1[3]);

            let fc = Color::blend_fast(dst, src, 255) * src;

            c1[0] = fc.r;
            c1[1] = fc.g;
            c1[2] = fc.b;
            c1[3] = fc.a;
        });
    }

    /// Count pixels in line operation, without drawing anything to the raster.
    pub fn cline(&mut self, x0: i32, y0: i32, x1: i32, y1: i32) -> u32 {

        let mut pixel_count: u32 = 0;

        let (mut x0, mut y0) = (x0, y0);

        let dx = i32::abs(x1 - x0);
        let sx = if x0 < x1 {1} else {-1};
        let dy = -i32::abs(y1 - y0);
        let sy = if y0 < y1 {1} else {-1};
        let mut error = dx + dy;
        
        loop {
            pixel_count += 1;
            if x0 == x1 && y0 == y1 { break; }
            let e2 = 2 * error;
            if e2 >= dy {
                if x0 == x1 { break; }
                error = error + dy;
                x0 = x0 + sx;
            }
            if e2 <= dx {
                if y0 == y1 { break; }
                error = error + dx;
                y0 = y0 + sy;
            }
        }

        return pixel_count;
    }
}

impl Default for Buffer {
    fn default() -> Self {
        let mut missing: Buffer = Buffer::new(32, 32);

        missing.clear_color(Color::RED);

        for iy in 0..16 {
            for ix in 0..16 {
                let color: Color = if iy % 2 == 0 { 
                    if ix % 2 == 1 { 
                        Color::MAGENTA
                    } else {
                        Color::BLACK
                    }
                } else { 
                    if ix % 2 == 0 { 
                        Color::MAGENTA
                    } else {
                        Color::BLACK
                    }
                };
                

                missing.prectangle(true, ix * 4, iy * 4, 4, 4, color);
            }
        }

        missing
    }
}