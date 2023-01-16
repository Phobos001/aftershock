use rayon::prelude::*;

use crate::color::*;
use crate::partitioned_rasterizer::PartitionedRasterizer;
use crate::vector2::*;
use crate::matrix3::*;
use crate::font::*;
use crate::math::*;

// Draw Mode Definition
pub type PSetOp = fn(&mut Rasterizer, usize, Color);

/// Controls how a Rasterizer should draw incoming pixels.
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
    PatternOpaque,
    PatternAlpha,
    PatternAddition,
    PatternSubtraction,
    PatternMultiply,
    PatternDivide,
    PatternInvertedAlpha,
    PatternInvertedOpaque,
    PatternInvertedBgAlpha,
    PatternInvertedBgOpaque,
    // Collect,
}

fn pset_noop(rasterizer: &mut Rasterizer, idx: usize, color: Color) {
    rasterizer.color[idx + 0] = color.r;  // R
    rasterizer.color[idx + 1] = color.g;  // G
    rasterizer.color[idx + 2] = color.b;  // B
    rasterizer.color[idx + 3] = color.a;  // A
    rasterizer.drawn_pixels_since_clear += 1;
}

fn pset_noalpha(rasterizer: &mut Rasterizer, idx: usize, color: Color) {
    let color = color * rasterizer.tint;
    rasterizer.color[idx + 0] = color.r;  // R
    rasterizer.color[idx + 1] = color.g;  // G
    rasterizer.color[idx + 2] = color.b;  // B
    rasterizer.color[idx + 3] = color.a;  // A
    rasterizer.drawn_pixels_since_clear += 1;
}

/// Draw pixels if they are fully opaque, otherwise ignore them.
fn pset_opaque(rasterizer: &mut Rasterizer, idx: usize, color: Color) {
    if color.a < 255 { return; }
    let color = color * rasterizer.tint;
    rasterizer.color[idx + 0] = color.r;  // R
    rasterizer.color[idx + 1] = color.g;  // G
    rasterizer.color[idx + 2] = color.b;  // B
    rasterizer.color[idx + 3] = color.a;  // A
    rasterizer.drawn_pixels_since_clear += 1;
}

/// Draw pixels if they are fully opaque, otherwise ignore them. Forces them to be the tint color.
/// Useful for flashes or making masks
fn pset_force_tint(rasterizer: &mut Rasterizer, idx: usize, color: Color) {
    if color.a < 255 { return; }
    rasterizer.color[idx + 0] = rasterizer.tint.r;  // R
    rasterizer.color[idx + 1] = rasterizer.tint.g;  // G
    rasterizer.color[idx + 2] = rasterizer.tint.b;  // B
    rasterizer.color[idx + 3] = rasterizer.tint.a;  // A
    rasterizer.drawn_pixels_since_clear += 1;
}

/// Draw pixels and blend them with the background based on the alpha channel
fn pset_alpha(rasterizer: &mut Rasterizer, idx: usize, color: Color) {
    
    let fg = color * rasterizer.tint;
    let bg = Color::new(
        rasterizer.color[idx + 0],
        rasterizer.color[idx + 1],
        rasterizer.color[idx + 2],
        255,
    );

    let c = Color::blend_fast(fg, bg, rasterizer.opacity);

    rasterizer.color[idx + 0] = c.r;  // R
    rasterizer.color[idx + 1] = c.g;  // G
    rasterizer.color[idx + 2] = c.b;  // B
    rasterizer.color[idx + 3] = c.a;  // A
    rasterizer.drawn_pixels_since_clear += 1;
}

/// Add incoming and buffer pixels together and draw to screen
fn pset_addition(rasterizer: &mut Rasterizer, idx: usize, color: Color) {
    if color.a <= 0 { return; }
    let fg = color * rasterizer.tint;
    let bg = Color::new(
        rasterizer.color[idx + 0],
        rasterizer.color[idx + 1],
        rasterizer.color[idx + 2],
        255,
    );

    let c = Color::blend_fast(fg, bg, rasterizer.opacity) + bg;

    rasterizer.color[idx + 0] = c.r;  // R
    rasterizer.color[idx + 1] = c.g;  // G
    rasterizer.color[idx + 2] = c.b;  // B
    rasterizer.color[idx + 3] = c.a;  // A
    rasterizer.drawn_pixels_since_clear += 1;
}

/// Multiply incoming pixel with buffer pixel.
fn pset_multiply(rasterizer: &mut Rasterizer, idx: usize, color: Color) {
    if color.a <= 0 { return; }
    let fg = color * rasterizer.tint;
    let bg = Color::new(
        rasterizer.color[idx + 0],
        rasterizer.color[idx + 1],
        rasterizer.color[idx + 2],
        255,
    );

    let c = Color::blend_fast(fg.inverted(), bg, rasterizer.opacity) * bg;

    rasterizer.color[idx + 0] = c.r;  // R
    rasterizer.color[idx + 1] = c.g;  // G
    rasterizer.color[idx + 2] = c.b;  // B
    rasterizer.color[idx + 3] = c.a;  // A
    rasterizer.drawn_pixels_since_clear += 1;
}

/// Draw inverted copy of incoming pixel with alpha blending
fn pset_inverted_alpha(rasterizer: &mut Rasterizer, idx: usize, color: Color) {
    if color.a <= 0 { return; }
    let fg = color* rasterizer.tint;
    let bg = Color::new(
        rasterizer.color[idx + 0],
        rasterizer.color[idx + 1],
        rasterizer.color[idx + 2],
        255,
    );

    let c = Color::blend_fast(fg.inverted(), bg, rasterizer.opacity);

    rasterizer.color[idx + 0] = c.r;  // R
    rasterizer.color[idx + 1] = c.g;  // G
    rasterizer.color[idx + 2] = c.b;  // B
    rasterizer.color[idx + 3] = c.a;  // A
    rasterizer.drawn_pixels_since_clear += 1;
}

/// Draw inverted copy of incoming pixel as opaque
fn pset_inverted_opaque(rasterizer: &mut Rasterizer, idx: usize, color: Color) {
    if color.a < 255 { return; }

    let bg = Color::new(
        rasterizer.color[idx + 0],
        rasterizer.color[idx + 1],
        rasterizer.color[idx + 2],
        255,
    );

    let c = (bg * rasterizer.tint).inverted();

    rasterizer.color[idx + 0] = c.r;  // R
    rasterizer.color[idx + 1] = c.g;  // G
    rasterizer.color[idx + 2] = c.b;  // B
    rasterizer.color[idx + 3] = c.a;  // A
    rasterizer.drawn_pixels_since_clear += 1;
}

/// Draw inverted copy of incoming pixel as opaque
fn pset_inverted_bg_opaque(rasterizer: &mut Rasterizer, idx: usize, color: Color) {
    if color.a < 255 { return; }

    let bg = Color::new(
        rasterizer.color[idx + 0],
        rasterizer.color[idx + 1],
        rasterizer.color[idx + 2],
        255,
    );

    let c = (bg * rasterizer.tint).inverted();

    rasterizer.color[idx + 0] = c.r;  // R
    rasterizer.color[idx + 1] = c.g;  // G
    rasterizer.color[idx + 2] = c.b;  // B
    rasterizer.color[idx + 3] = c.a;  // A
    rasterizer.drawn_pixels_since_clear += 1;
}

/// Draw inverted copy of incoming pixel as opaque
fn pset_inverted_bg_alpha(rasterizer: &mut Rasterizer, idx: usize, color: Color) {
    if color.a < 255 { return; }

    let bg = Color::new(
        rasterizer.color[idx + 0],
        rasterizer.color[idx + 1],
        rasterizer.color[idx + 2],
        255,
    );

    let c = Color::blend_fast(bg, (bg * rasterizer.tint).inverted(), rasterizer.opacity);

    rasterizer.color[idx + 0] = c.r;  // R
    rasterizer.color[idx + 1] = c.g;  // G
    rasterizer.color[idx + 2] = c.b;  // B
    rasterizer.color[idx + 3] = c.a;  // A
    rasterizer.drawn_pixels_since_clear += 1;
}

/// Drawing switchboard that draws directly into a  Drawing options like Tint and Opacity must be manually changed by the user.
#[derive(Clone)]
pub struct Rasterizer {
    pset_op: PSetOp,

    render_next_frame_as_animation: bool,
    render_next_frame_folder: String,

    // For Partitioned Rasterizer
    pub offset_x: usize,
    pub offset_y: usize,
    
    pub width: usize,
    pub height: usize,
    pub color: Vec<u8>,

    pub camera_position: Vector2,
    pub camera_rotation: f64,
    pub camera_scale: Vector2,
    pub camera_matrix: Matrix3,

    pub draw_mode: DrawMode,
    pub tint: Color,
    pub opacity: u8,

    pub drawn_pixels_since_clear: u64,
}

impl Rasterizer {

    /// Makes a new Rasterizer to draw to a screen-sized buffer
    ///
    /// # Arguments
    /// * 'width' - Horizontal size of the 
    /// * 'height' - Vertical size of the 
    pub fn new(width: usize, height: usize) -> Rasterizer {
        //println!("Rasterizer: {} x {} x {}, Memory: {}B", width, height, 4, (width * height * 4));
        Rasterizer {
            pset_op: pset_opaque,

            render_next_frame_as_animation: false,
            render_next_frame_folder: "./".to_string(),

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
            tint: Color::white(),
            opacity: 255,

            drawn_pixels_since_clear: 0,
        }
    }

    pub fn new_from_image(path_to: &str) -> Result<Rasterizer, String> {
		match lodepng::decode32_file(path_to) {
			Ok(image) => {
				//println!("Image: {}, Res: {} x {}, Size: {}B", path_to, image.width, image.height, image.buffer.len());
				//let buffer_new: Vec<u8> =  image.buffer.as_bytes().to_vec();
                use rgb::*;

                // Convert to atomics for parallelism

				Ok(Rasterizer {
                    pset_op: pset_opaque,

                    render_next_frame_as_animation: false,
                    render_next_frame_folder: "./".to_string(),

                    width: image.width,
                    height: image.height,
                    color: image.buffer.as_bytes().to_vec(),

                    camera_position: Vector2::ZERO,
                    camera_rotation: 0.0,
                    camera_scale: Vector2::ONE,
                    camera_matrix: Matrix3::identity(),

                    draw_mode: DrawMode::Opaque,
                    tint: Color::white(),
                    opacity: 255,

                    offset_x: 0,
                    offset_y: 0,

                    drawn_pixels_since_clear: 0,
                })
			},
			Err(reason) => {
				println!("ERROR - IMAGE: Could not load {} | {}", path_to, reason);
				Err(format!("ERROR - IMAGE: Could not load {} | {}", path_to, reason))
			}
		}
    }

    /// Clears the framebuffer and changes its width and height to new values.
    pub fn resize(&mut self, width: usize, height: usize) {
        self.width = width;
        self.height = height;
        self.color = vec![0; width * height * 4];
    }

    /// Sets the Rasterizers drawing mode for incoming pixels. Should be defined before every drawing operation.
    /// # Arguments
    /// * 'mode' - Which drawing function should the Rasterizer use.
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

    pub fn save_next_frame_draw_process_until_clear(&mut self, path_to: &str) {
        self.render_next_frame_as_animation = true;
        self.render_next_frame_folder = path_to.to_string();
    }

    pub fn into_partitioned(&self) -> PartitionedRasterizer {
        let mut pr = PartitionedRasterizer::new(self.width, self.height, 0);
        pr.rasterizer.blit(self, 0, 0);
        pr
    }

    /// Create a copy of a region 
    /*pub fn blit_copy(&self, x: i64, y: i64, width: usize, height: usize) -> Rasterizer {
        let stride = 4;

        let mut rasterizer: Rasterizer = Rasterizer::new(width, height);

        // Go down in rows. i is the current row.
        self.color.chunks_exact(self.width * 4).enumerate().for_each(|(row_idx, pixel)| {
            if (row_idx as i64) > y && (row_idx as i64) <= (y + height as i64) {
                //let sx = i64::clamp(x, 0, self.width)l
            }
        });
    }*/

    pub fn blit(&mut self, src: &Rasterizer, x: i64, y: i64) {
        let is_equal_size: bool = self.width == src.width && self.height == src.height;
        if is_equal_size {
            self.color.copy_from_slice(&src.color);
            return;
        }

        let stride = 4;
        // We blit these directly into the color buffer because otherwise we'd just be drawing everything over again and we don't have to worry about depth
        
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
        self.render_next_frame_as_animation = false;
    }

    /// Clears the screen to a color.
    /// # Arguments
    /// * 'color' - Color the screen should be cleared too.
    pub fn clear_color(&mut self, color: Color) {
        let parallel_threshold = 129600;
        if self.width * self.height > parallel_threshold {
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
        self.render_next_frame_as_animation = false;
    }

    pub fn update_camera(&mut self) {
        // Camera is usually in the top left corner so we need to change the zoom scaling so it fits in the middle of the screen
        let camera_offset: Vector2 = Vector2::new(
            -lerpf(0.0, self.width as f64, 0.5),
            -lerpf(0.0, self.height as f64, 0.5),
        );

        let camera_mtx_o = Matrix3::translated(camera_offset);
        let camera_mtx_r = Matrix3::rotated(self.camera_rotation);
        let camera_mtx_p = Matrix3::translated(-self.camera_position + Vector2::new(self.width as f64 / 2.0, self.height as f64 / 2.0));
        let camera_mtx_s = Matrix3::scaled(self.camera_scale);

        // Combine matricies using matrix multiplication
        self.camera_matrix = camera_mtx_p * camera_mtx_r * camera_mtx_s * camera_mtx_o;
    }

    /// Draws a pixel to the color buffer, using the Rasterizers set DrawMode. DrawMode defaults to Opaque.
    pub fn pset(&mut self, x: i64, y: i64, color: Color) {
        self.drawn_pixels_since_clear += 1;
        //let x = x.rem_euclid(self.width as i64);
        //let y = y.rem_euclid(self.height as i64);
        let idx: usize = ((y * (self.width as i64) + x) * 4) as usize;

        let out_left: bool = x < 0;
        let out_right: bool = x > (self.width) as i64 - 1;
        let out_top: bool = y < 0;
        let out_bottom: bool = y > (self.height) as i64 - 1;
        let out_of_range: bool = idx > (self.width * self.height * 4) - 1;

        if out_of_range || out_left || out_right || out_top || out_bottom  { return; }

        // We have to put paraenthesis around the fn() variables or else the compiler will think it's a method.
        (self.pset_op)(self, idx, color);
    }

    /// Gets a color from the color buffer.
    pub fn pget(&self, x: i64, y: i64) -> Color {

        let idx: usize = ((y * (self.width as i64) + x) * 4) as usize;

        let out_left: bool = x < 0;
        let out_right: bool = x > (self.width) as i64 - 1;
        let out_top: bool = y < 0;
        let out_bottom: bool = y > (self.height) as i64 - 1;
        let out_of_range: bool = idx > (self.width * self.height * 4) - 1;

        if out_of_range || out_left || out_right || out_top || out_bottom  { return Color::clear(); }

        return Color::new(
            self.color[idx + 0],
            self.color[idx + 1],
            self.color[idx + 2],
            self.color[idx + 3]
        );
    }
    
    /// Draws a line across two points
    pub fn pline(&mut self, x0: i64, y0: i64, x1: i64, y1: i64, color: Color) {
        // Cant find original source but it's been modified for Rust from C or C++

        let x0 = i64::clamp(x0, 0, self.width as i64);
        let x1 = i64::clamp(x1, 0, self.width as i64);
        let y0 = i64::clamp(y0, 0, self.height as i64);
        let y1 = i64::clamp(y1, 0, self.height as i64);

        // Create local variables for moving start point
        let mut x0 = x0;
        let mut y0 = y0;
    
        // Get absolute x/y offset
        let dx = if x0 > x1 { x0 - x1 } else { x1 - x0 };
        let dy = if y0 > y1 { y0 - y1 } else { y1 - y0 };
    
        // Get slopes
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
    
        // Initialize error
        let mut err = if dx > dy { dx } else {-dy} / 2;
        let mut err2;
    
        loop {
            // Set pixel
            self.pset(x0, y0, color);
    
            // Check end condition
            if x0 == x1 && y0 == y1 { break };
    
            // Store old error
            err2 = 2 * err;
    
            // Adjust error and start position
            if err2 > -dx { err -= dy; x0 += sx; }
            if err2 < dy { err += dx; y0 += sy; }
        }
    }
    
        /// Draws a rectangle onto the screen. Can either be filled or outlined.
    pub fn prectangle(&mut self, filled: bool, x: i64, y: i64, w: i64, h: i64, color: Color) {
        let x0 = i64::clamp(x, 0, self.width as i64);
        let x1 = i64::clamp(x + w, 0, self.width as i64);
        let y0 = i64::clamp(y, 0, self.height as i64);
        let y1 = i64::clamp(y + h, 0, self.height as i64);
    
        if filled {
            for py in y0..y1 {
                for px in x0..x1 {
                    self.pset(px, py, color);
                }
            }
        } else {
            for tops in x0..x1+1 {
                self.pset(tops, y0, color);
                self.pset(tops, y1, color);
            }

            for sides in y0..y1 {
                self.pset(x0, sides, color);
                self.pset(x1, sides, color);
            }
        }
    }
    
    /// Draws a circle onto the screen. Can either be filled or outlined.
    pub fn pcircle(&mut self, filled: bool, xc: i64, yc: i64, r: i64, color: Color) { 

        let minx = i64::clamp(xc - r, 0, self.width  as i64);
        let maxx = i64::clamp((xc + r) + 1, 0, self.width  as i64);
        let miny = i64::clamp(yc - r, 0, self.height as i64);
        let maxy = i64::clamp((yc + r)+1, 0, self.height as i64);

        if filled {
            for py in miny..maxy {
                for px in minx..maxx {
                    if ((px - xc) * (px - xc)) + ((py - yc) * (py - yc)) <= r * r {
                        self.pset(px, py, color);
                    }
                }
            }
        } else {
            let mut x: i64 = 0;
            let mut y: i64 = r; 
            let mut d: i64 = 3 - 2 * r;
            
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
    pub fn pimg(&mut self, image: &Rasterizer, x: i64, y: i64) {
        for ly in 0..image.height as i64 {
            for lx in 0..image.width as i64 {
                let pc = image.pget(lx, ly);
                let px = x + lx;
                let py = y + ly;
                
                // Pixel out of bounds
                if pc.a <= 0 || (px < 0 || px > self.width as i64) || (py < 0 || py > self.height as i64) { continue; }

                self.pset(x + lx, y + ly, pc);
            }
        }
    }

    /// Draws a section of an image directly to the screen.
    pub fn pimgrect(&mut self, image: &Rasterizer, x: i64, y: i64, rx: i64, ry: i64, rw: i64, rh: i64) {
        let range_x = i64::clamp(rx + rw, 0, self.width as i64);
        let range_y = i64::clamp(ry + rh, 0, self.height as i64);
        for ly in ry..range_y {
            for lx in rx..range_x {
                let mlx = lx.rem_euclid(image.width as i64);
                let mly = ly.rem_euclid(image.height as i64);

                let px: i64 = (x + mlx as i64) - rx as i64;
                let py: i64 = (y + mly as i64) - ry as i64;

                self.pset(px, py, image.pget(mlx as i64, mly as i64));
            }
        }
    }

    /// Draws a rotated and scaled image to the screen using matrix multiplication.
    pub fn pimgmtx(&mut self, image: &Rasterizer, position_x: f64, position_y: f64, rotation: f64, scale_x: f64, scale_y: f64, offset_x: f64, offset_y: f64) {

        // Early out if the image is going to be too small to draw
        let area_x = image.width as f64 * scale_x;
        let area_y = image.height as f64 * scale_y;

        if area_x * area_y < 1.0 {
            return;
        }

        let offset_x = -lerpf(0.0, image.width as f64, offset_x);
        let offset_y = -lerpf(0.0, image.height as f64, offset_y);

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
        sx = f64::min(sx, p1.x); sy = f64::min(sy, p1.y);
        ex = f64::max(ex, p1.x); ey = f64::max(ey, p1.y);

        // Bottom-Right Corner
        let p2: Vector2 = cmtx.forward(Vector2::new(image.width as f64, image.height as f64));
        sx = f64::min(sx, p2.x); sy = f64::min(sy, p2.y);
        ex = f64::max(ex, p2.x); ey = f64::max(ey, p2.y);

        // Bottom-Left Corner
        let p3: Vector2 = cmtx.forward(Vector2::new(0.0, image.height as f64));
        sx = f64::min(sx, p3.x); sy = f64::min(sy, p3.y);
        ex = f64::max(ex, p3.x); ey = f64::max(ey, p3.y);

        // Top-Right Corner
        let p4: Vector2 = cmtx.forward(Vector2::new(image.width as f64, 0.0));
        sx = f64::min(sx, p4.x); sy = f64::min(sy, p4.y);
        ex = f64::max(ex, p4.x); ey = f64::max(ey, p4.y);

        // Extend the bounding box by a few pixels to catch clipping errors
        let mut rsx = sx as i64;
        let mut rsy = sy as i64;
        let mut rex = ex as i64+1;
        let mut rey = ey as i64+1;

        // Sprite isn't even in frame, don't draw anything
        if (rex < 0 || rsx > self.width as i64) && (rey < 0 || rsy > self.height as i64) { return; }

        // Okay but clamp the ranges in frame so we're not wasting time on stuff offscreen

        rsx = i64::clamp(rsx, 0, self.width as i64);
        rsy = i64::clamp(rsy, 0, self.height as i64);
        rex = i64::clamp(rex, 0, self.width as i64);
        rey = i64::clamp(rey, 0, self.height as i64);

        let cmtx_inv = cmtx.clone().inv();

		// We can finally draw!
        for ly in rsy..rey {
            for lx in rsx..rex {
                // We have to use the inverted compound matrix (cmtx_inv) in order to get the correct pixel data from the image.
                let ip: Vector2 = cmtx_inv.forward(Vector2::new(lx as f64, ly as f64));

                // Ceil the transformed pixel positions to fix the colot pullingg
                let color: Color = image.pget(f64::ceil(ip.x) as i64, f64::ceil(ip.y) as i64);

                // We skip drawing entirely if the alpha is zero.
                // Otherwise leaves weird grey box
                if color.a <= 0 { continue; }
                self.pset(lx as i64, ly as i64, color);
            }
        }
    }

    /// Draws text directly to the screen using a provided font.
    pub fn pprint(&mut self, font: &Font, text: String, x: i64, y: i64, newline_space: i64, wrap_width: Option<u32>) {
        let mut jumpx: i64 = 0;
        let mut jumpy: i64 = 0;
        let chars: Vec<char> = text.chars().collect();

        for i in 0..chars.len() {
            
            if chars[i] == '\n' { jumpy += font.glyph_height as i64 + newline_space; jumpx = 0; continue; }
            if chars[i] == ' ' { jumpx += font.glyph_width as i64; continue; }
            for j in 0..font.glyphidx.len() {
                if font.glyphidx[j] == chars[i] {
                    let rectx: i64 = (j as i64 * font.glyph_width as i64) % (font.fontimg.width as i64);
                    let recty: i64 = ((j as i64 * font.glyph_width as i64) / font.fontimg.width as i64) * font.glyph_height as i64;
                    let rectw: i64 = font.glyph_width as i64;
                    let recth: i64 = font.glyph_height as i64;
                    
                    self.pimgrect(&font.fontimg, x + jumpx as i64, y + jumpy as i64, rectx, recty, rectw, recth);
                    

                    jumpx += font.glyph_width as i64 + font.glyph_spacing as i64;
                }
            }
            if wrap_width.is_some() && (jumpx as u32) > wrap_width.unwrap() { jumpy += font.glyph_height as i64 + newline_space; jumpx = 0; }
        }
    }

    /// Draws a triangle directly to the screen.
    /// Algorithm written by nusan for the PICO-8 3D Renderer 
    pub fn ptriangle(&mut self, filled: bool, x1: i64, y1: i64, x2: i64, y2: i64, x3: i64, y3: i64, color: Color) {
        if filled {
            // Collect pixels from lines without drawing to the screen
            let vl12 = self.cline(x1, y1, x2, y2);
            let vl23 = self.cline(x2, y2, x3, y3);
            let vl31 = self.cline(x3, y3, x1, y1);

            let mut edge_pixels: Vec<(i64, i64)> = Vec::new();
            edge_pixels.extend(vl12);
            edge_pixels.extend(vl23);
            edge_pixels.extend(vl31);
  

            let mut scanline_rows: Vec<Vec<(i64, i64)>> = vec![Vec::new(); self.height];

            for p in edge_pixels {
                if p.1 >= 0 && p.1 < self.height as i64{
                    scanline_rows[p.1 as usize].push(p);
                }
            }

            for row in scanline_rows {
                if row.len() == 0 { continue; }
                let height = row[0].1;
                self.pline(row[0].0, height, row[row.len()-1].0, height, color);
            }
        } else {
            self.pline(x1, y1, x2, y2, color);
            self.pline(x1, y1, x3, y3, color);
            self.pline(x2, y2, x3, y3, color);
        }
    }

    /// Draws a quadratic beizer curve onto the screen.
    pub fn pbeizer(&mut self, x0: i64, y0: i64, x1: i64, y1: i64, mx: i64, my: i64, color: Color) {
        let mut step: f64 = 0.0;

        // Get the maximal number of pixels we will need to use and get its inverse as a step size.
        // Otherwise we don't know how many pixels we will need to draw
        let stride_c1 = self.cline(x0, y0, mx, my).len() as f64;
        let stride_c2 = self.cline(mx, my, x1, y1).len() as f64;

        let stride: f64 = (1.0 / (stride_c1 + stride_c2)) * 0.5;

        let x0 = x0 as f64;
        let y0 = x0 as f64;
        let x1 = x1 as f64;
        let y1 = y1 as f64;
        let mx = mx as f64;
        let my = my as f64;

        loop {
            if step > 1.0 { break; }

            let px0 = f64::clamp(lerpf(x0, mx, step), 0.0, self.width as f64);
            let py0 = f64::clamp(lerpf(y0, my, step), 0.0, self.height as f64);

            let px1 = f64::clamp(lerpf(px0, x1, step), 0.0, self.width as f64);
            let py1 = f64::clamp(lerpf(py0, y1, step), 0.0, self.height as f64);

            self.pset(px1 as i64, py1 as i64, color);
            step += stride;
        }
    }

    /// Draws a cubic beizer curve onto the screen.
    pub fn pbeizer_cubic(&mut self, x0: i64, y0: i64, x1: i64, y1: i64, mx0: i64, my0: i64, mx1: i64, my1: i64, color: Color) {
        let mut step: f64 = 0.0;

        // Get the maximal number of pixels we will need to use and get its inverse as a step size.
        // Otherwise we don't know how many pixels we will need to draw
        let stride_c1: f64 = self.cline(x0, y0, mx0, my0).len() as f64;
        let stride_c2: f64 = self.cline(mx0, my0, mx1, my1).len() as f64;
        let stride_c3: f64 = self.cline(mx1, my1, x1, y1).len() as f64;

        let stride = (1.0 / (stride_c1 + stride_c2 + stride_c3)) * 0.5;

        let x0 = x0 as f64;
        let y0 = x0 as f64;
        let x1 = x1 as f64;
        let y1 = y1 as f64;
        let mx0 = mx0 as f64;
        let my0 = my0 as f64;
        let mx1 = mx1 as f64;
        let my1 = my1 as f64;

        loop {
            if step > 1.0 { break; }

            let px0 = f64::clamp(lerpf(x0, mx0, step), 0.0, self.width as f64);
            let py0 = f64::clamp(lerpf(y0, my0, step), 0.0, self.height as f64);

            let px1 = f64::clamp(lerpf(px0, mx1, step), 0.0, self.width as f64);
            let py1 = f64::clamp(lerpf(py0, my1, step), 0.0, self.height as f64);

            let px2 = f64::clamp(lerpf(px1, x1, step), 0.0, self.width as f64);
            let py2 = f64::clamp(lerpf(py1, y1, step), 0.0, self.height as f64);

            self.pset(px2 as i64, py2 as i64, color);
            step += stride;
        }
    }

    /// Returns pixel positions across the line.
    pub fn cline(&mut self, x0: i64, y0: i64, x1: i64, y1: i64) -> Vec<(i64, i64)> {

        let mut pixels: Vec<(i64, i64)> = Vec::new();

        // Create local variables for moving start point
        let mut x0 = x0;
        let mut y0 = y0;

        // Get absolute x/y offset
        let dx = if x0 > x1 { x0 - x1 } else { x1 - x0 };
        let dy = if y0 > y1 { y0 - y1 } else { y1 - y0 };

        // Get slopes
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };

        // Initialize error
        let mut err = if dx > dy { dx } else {-dy} / 2;
        let mut err2;

        loop {
            // Set pixel
            pixels.push((x0, y0));

            // Check end condition
            if x0 == x1 && y0 == y1 { break };

            // Store old error
            err2 = 2 * err;

            // Adjust error and start position
            if err2 > -dx { err -= dy; x0 += sx; }
            if err2 < dy { err += dx; y0 += sy; }
        }

        return pixels;
    }
}