use crate::color::*;
use crate::vector2::*;
use crate::matrix3::*;
use crate::image::*;
use crate::font::*;
use crate::math::*;
use crate::framebuffer::*;

// Draw Mode Definition
pub type PSetOp = fn(&mut Rasterizer, usize, Color);

// Calls functions in other structures with each pixel drawn. Good for 
pub type OuterOp = fn(i64, i64, Color);

/// Controls how a rasterizer should draw incoming pixels.
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
    InvertedAlpha,
    InvertedOpaque,
    AlphaSlow,
    AdditionSlow,
    SubtractionSlow,
    MultiplySlow,
    DivideSlow,
    InvertedAlphaSlow,
    // Collect,
}

fn pset_noop(rasterizer: &mut Rasterizer, idx: usize, color: Color) {
    rasterizer.drawn_pixels_since_clear += 1;
}

fn pset_noalpha(rasterizer: &mut Rasterizer, idx: usize, color: Color) {
    let color = color * rasterizer.tint;
    rasterizer.framebuffer.color[idx + 0] = color.r;  // R
    rasterizer.framebuffer.color[idx + 1] = color.g;  // G
    rasterizer.framebuffer.color[idx + 2] = color.b;  // B
    rasterizer.framebuffer.color[idx + 3] = color.a;  // A
    rasterizer.drawn_pixels_since_clear += 1;
}

/// Draw pixels if they are fully opaque, otherwise ignore them.
fn pset_opaque(rasterizer: &mut Rasterizer, idx: usize, color: Color) {
    if color.a < 255 { return; }
    let color = color * rasterizer.tint;
    rasterizer.framebuffer.color[idx + 0] = color.r;  // R
    rasterizer.framebuffer.color[idx + 1] = color.g;  // G
    rasterizer.framebuffer.color[idx + 2] = color.b;  // B
    rasterizer.framebuffer.color[idx + 3] = color.a;  // A
    rasterizer.drawn_pixels_since_clear += 1;
}

/// Draw pixels and blend them with the background based on the alpha channel
fn pset_alpha(rasterizer: &mut Rasterizer, idx: usize, color: Color) {
    
    let fg = color * rasterizer.tint;
    let bg = Color::new(
        rasterizer.framebuffer.color[idx + 0],
        rasterizer.framebuffer.color[idx + 1],
        rasterizer.framebuffer.color[idx + 2],
        255,
    );

    let c = Color::blend_fast(fg, bg, rasterizer.opacity);

    rasterizer.framebuffer.color[idx + 0] = c.r;  // R
    rasterizer.framebuffer.color[idx + 1] = c.g;  // G
    rasterizer.framebuffer.color[idx + 2] = c.b;  // B
    rasterizer.framebuffer.color[idx + 3] = c.a;  // A
    rasterizer.drawn_pixels_since_clear += 1;
}

/// Add incoming and buffer pixels together and draw to screen
fn pset_addition(rasterizer: &mut Rasterizer, idx: usize, color: Color) {
    if color.a <= 0 { return; }
    let fg = color * rasterizer.tint;
    let bg = Color::new(
        rasterizer.framebuffer.color[idx + 0],
        rasterizer.framebuffer.color[idx + 1],
        rasterizer.framebuffer.color[idx + 2],
        255,
    );

    let c = Color::blend_fast(fg, bg, rasterizer.opacity) + bg;

    rasterizer.framebuffer.color[idx + 0] = c.r;  // R
    rasterizer.framebuffer.color[idx + 1] = c.g;  // G
    rasterizer.framebuffer.color[idx + 2] = c.b;  // B
    rasterizer.framebuffer.color[idx + 3] = c.a;  // A
    rasterizer.drawn_pixels_since_clear += 1;
}

/// Multiply incoming pixel with buffer pixel.
fn pset_multiply(rasterizer: &mut Rasterizer, idx: usize, color: Color) {
    if color.a <= 0 { return; }
    let fg = color * rasterizer.tint;
    let bg = Color::new(
        rasterizer.framebuffer.color[idx + 0],
        rasterizer.framebuffer.color[idx + 1],
        rasterizer.framebuffer.color[idx + 2],
        255,
    );

    let c = Color::blend_fast(fg.inverted(), bg, rasterizer.opacity) * bg;

    rasterizer.framebuffer.color[idx + 0] = c.r;  // R
    rasterizer.framebuffer.color[idx + 1] = c.g;  // G
    rasterizer.framebuffer.color[idx + 2] = c.b;  // B
    rasterizer.framebuffer.color[idx + 3] = c.a;  // A
    rasterizer.drawn_pixels_since_clear += 1;
}

/// Draw inverted copy of incoming pixel with alpha blending
fn pset_inverted_alpha(rasterizer: &mut Rasterizer, idx: usize, color: Color) {
    if color.a <= 0 { return; }
    let fg = color* rasterizer.tint;
    let bg = Color::new(
        rasterizer.framebuffer.color[idx + 0],
        rasterizer.framebuffer.color[idx + 1],
        rasterizer.framebuffer.color[idx + 2],
        255,
    );

    let c = Color::blend_fast(fg.inverted(), bg, rasterizer.opacity);
    

    rasterizer.framebuffer.color[idx + 0] = c.r;  // R
    rasterizer.framebuffer.color[idx + 1] = c.g;  // G
    rasterizer.framebuffer.color[idx + 2] = c.b;  // B
    rasterizer.framebuffer.color[idx + 3] = c.a;  // A
    rasterizer.drawn_pixels_since_clear += 1;
}

/// Draw inverted copy of incoming pixel as opaque
fn pset_inverted_opaque(rasterizer: &mut Rasterizer, idx: usize, color: Color) {
    if color.a < 255 { return; }
    let c = (color * rasterizer.tint).inverted();

    rasterizer.framebuffer.color[idx + 0] = c.r;  // R
    rasterizer.framebuffer.color[idx + 1] = c.g;  // G
    rasterizer.framebuffer.color[idx + 2] = c.b;  // B
    rasterizer.framebuffer.color[idx + 3] = c.a;  // A
    rasterizer.drawn_pixels_since_clear += 1;
}

/// Draw pixels and blend them with the background based on the alpha channel
fn pset_alpha_slow(rasterizer: &mut Rasterizer, idx: usize, color: Color) {
    
    let fg = color * rasterizer.tint;
    let bg = Color::new(
        rasterizer.framebuffer.color[idx + 0],
        rasterizer.framebuffer.color[idx + 1],
        rasterizer.framebuffer.color[idx + 2],
        255,
    );

    let c = Color::blend_slow(fg, bg, rasterizer.opacity as f64 / 255.0);

    rasterizer.framebuffer.color[idx + 0] = c.r;  // R
    rasterizer.framebuffer.color[idx + 1] = c.g;  // G
    rasterizer.framebuffer.color[idx + 2] = c.b;  // B
    rasterizer.framebuffer.color[idx + 3] = c.a;  // A
    rasterizer.drawn_pixels_since_clear += 1;
}

/// Add incoming and buffer pixels together and draw to screen
fn pset_addition_slow(rasterizer: &mut Rasterizer, idx: usize, color: Color) {
    if color.a <= 0 { return; }
    let fg = color * rasterizer.tint;
    let bg = Color::new(
        rasterizer.framebuffer.color[idx + 0],
        rasterizer.framebuffer.color[idx + 1],
        rasterizer.framebuffer.color[idx + 2],
        255,
    );

    let c = Color::blend_slow(fg, bg, rasterizer.opacity as f64 / 255.0) + bg;

    rasterizer.framebuffer.color[idx + 0] = c.r;  // R
    rasterizer.framebuffer.color[idx + 1] = c.g;  // G
    rasterizer.framebuffer.color[idx + 2] = c.b;  // B
    rasterizer.framebuffer.color[idx + 3] = c.a;  // A
    rasterizer.drawn_pixels_since_clear += 1;
}

/// Multiply incoming pixel with buffer pixel.
fn pset_multiply_slow(rasterizer: &mut Rasterizer, idx: usize, color: Color) {
    if color.a <= 0 { return; }
    let fg = color * rasterizer.tint;
    let bg = Color::new(
        rasterizer.framebuffer.color[idx + 0],
        rasterizer.framebuffer.color[idx + 1],
        rasterizer.framebuffer.color[idx + 2],
        255,
    );

    let c = Color::blend_slow(fg.inverted(), bg, rasterizer.opacity as f64 / 255.0) * bg;

    rasterizer.framebuffer.color[idx + 0] = c.r;  // R
    rasterizer.framebuffer.color[idx + 1] = c.g;  // G
    rasterizer.framebuffer.color[idx + 2] = c.b;  // B
    rasterizer.framebuffer.color[idx + 3] = c.a;  // A
    rasterizer.drawn_pixels_since_clear += 1;
}

/// Draw inverted copy of incoming pixel with alpha blending
fn pset_inverted_alpha_slow(rasterizer: &mut Rasterizer, idx: usize, color: Color) {
    if color.a <= 0 { return; }
    let fg = color* rasterizer.tint;
    let bg = Color::new(
        rasterizer.framebuffer.color[idx + 0],
        rasterizer.framebuffer.color[idx + 1],
        rasterizer.framebuffer.color[idx + 2],
        255,
    );

    let c = Color::blend_slow(fg.inverted(), bg, rasterizer.opacity as f64 / 255.0);
    

    rasterizer.framebuffer.color[idx + 0] = c.r;  // R
    rasterizer.framebuffer.color[idx + 1] = c.g;  // G
    rasterizer.framebuffer.color[idx + 2] = c.b;  // B
    rasterizer.framebuffer.color[idx + 3] = c.a;  // A
    rasterizer.drawn_pixels_since_clear += 1;
}

/* /// Collect drawn pixels into collected_pixels instead of drawing them to the buffer.
/// This is useful for more advanced graphical effects, for example using a series of ptriangle's to
/// build a polygonal area, then drawing a texture onto the pixels.
fn pset_collect(rasterizer: &mut Rasterizer, idx: usize, color: Color) {
    let idx_unstrided = idx / 4;
    let x = modi(idx_unstrided as i64, rasterizer.framebuffer.width as i64);
    let y = idx_unstrided as i64 / rasterizer.framebuffer.width as i64;
    rasterizer.collected_pixels.push((x, y, color));
} */

/// Drawing switchboard that draws directly into a framebuffer. Drawing options like Tint and Opacity must be manually changed by the user.
#[derive(Clone)]
pub struct Rasterizer {
    pset_op: PSetOp,
    render_next_frame_as_animation: bool,
    render_next_frame_folder: String,
    
    pub framebuffer: FrameBuffer,

    pub draw_mode: DrawMode,
    pub tint: Color,
    pub opacity: u8,

    pub camera: Vector2,

    pub drawn_pixels_since_clear: u64,
    pub time_since_clear: std::time::Duration,

}

impl Rasterizer {

    /// Makes a new Rasterizer to draw to a screen-sized buffer
    ///
    /// # Arguments
    /// * 'width' - Horizontal size of the framebuffer.
    /// * 'height' - Vertical size of the framebuffer.
    pub fn new(width: usize, height: usize) -> Rasterizer {
        //println!("Rasterizer: {} x {} x {}, Memory: {}B", width, height, 4, (width * height * 4));
        Rasterizer {
            pset_op: pset_opaque,
            render_next_frame_as_animation: false,
            render_next_frame_folder: "./".to_string(),


            framebuffer: FrameBuffer::new(width, height),
            

            draw_mode: DrawMode::Opaque,
            tint: Color::white(),
            opacity: 255,

            camera: Vector2::zero(),

            drawn_pixels_since_clear: 0,
            time_since_clear: std::time::Duration::new(0, 0),
            
        }
    }

    /// Clears the framebuffer and changes its width and height to new values.
    pub fn resize_framebuffer(&mut self, width: usize, height: usize) {
        self.framebuffer = FrameBuffer::new(width, height);
    }

    /// Sets the rasterizers drawing mode for incoming pixels. Should be defined before every drawing operation.
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
            DrawMode::InvertedAlpha         => {self.pset_op = pset_inverted_alpha;}
            DrawMode::InvertedOpaque        => {self.pset_op = pset_inverted_opaque;}
            DrawMode::AlphaSlow             => {self.pset_op = pset_alpha_slow;},
            DrawMode::AdditionSlow          => {self.pset_op = pset_addition_slow;},
            DrawMode::MultiplySlow          => {self.pset_op = pset_multiply_slow;}
            DrawMode::InvertedAlphaSlow     => {self.pset_op = pset_inverted_alpha_slow;}
            //DrawMode::Collect => {self.pset_op = pset_collect;}
            _ => {},
        }
    }

    pub fn save_next_frame_draw_process_until_clear(&mut self, path_to: &str) {
        self.render_next_frame_as_animation = true;
        self.render_next_frame_folder = path_to.to_string();
    }

    /// Clears the frame memory directly, leaving a black screen.
    pub fn clear(&mut self) {
        self.framebuffer.color = vec![0; self.framebuffer.width * self.framebuffer.height * 4];
        self.drawn_pixels_since_clear = 0;
        self.render_next_frame_as_animation = false;
    }

    /// Clears the screen to a color.
    /// # Arguments
    /// * 'color' - Color the screen should be cleared too.
    pub fn clear_color(&mut self, color: Color) {
        self.framebuffer.color.chunks_exact_mut(4).for_each(|c| {
            c[0] = color.r;
            c[1] = color.g;
            c[2] = color.b;
            c[3] = color.a;
        });
        self.drawn_pixels_since_clear = 0;
        self.render_next_frame_as_animation = false;
    }

    /// Draws a pixel to the color buffer, using the rasterizers set DrawMode. DrawMode defaults to Opaque.
    pub fn pset(&mut self, x: i64, y: i64, color: Color) {

        let x = x - f64::round(self.camera.x) as i64;
        let y = y - f64::round(self.camera.y) as i64;

        let idx: usize = ((y * (self.framebuffer.width as i64) + x) * 4) as usize;

        let out_left: bool = x < 0;
        let out_right: bool = x > (self.framebuffer.width) as i64 - 1;
        let out_top: bool = y < 0;
        let out_bottom: bool = y > (self.framebuffer.height) as i64 - 1;
        let out_of_range: bool = idx > (self.framebuffer.width * self.framebuffer.height * 4) - 1;

        if out_of_range || out_left || out_right || out_top || out_bottom  { return; }
        
        // We have to put paraenthesis around the fn() variables or else the compiler will think it's a method.
        (self.pset_op)(self, idx, color);
        if self.render_next_frame_as_animation {
            self.framebuffer.to_image().save(format!("{}{}.png", self.render_next_frame_folder, self.drawn_pixels_since_clear).as_str());
            // Let the OS think
            std::thread::sleep(std::time::Duration::from_micros(10));
        }

    }

    /// Gets a color from the color buffer.
    pub fn pget(&mut self, x: i64, y: i64) -> Color {
        let idx: usize = ((y * (self.framebuffer.width as i64) + x) * 4) as usize;

        let out_left: bool = x < 0;
        let out_right: bool = x > (self.framebuffer.width) as i64 - 1;
        let out_top: bool = y < 0;
        let out_bottom: bool = y > (self.framebuffer.height) as i64 - 1;
        let out_of_range: bool = idx > (self.framebuffer.width * self.framebuffer.height * 4) - 1;

        if out_of_range || out_left || out_right || out_top || out_bottom  { return Color::black(); }

        return Color::new(
            self.framebuffer.color[idx + 0],
            self.framebuffer.color[idx + 1],
            self.framebuffer.color[idx + 2],
            self.framebuffer.color[idx + 3]
        );
    }
    
    /// Draws a line across two points
    pub fn pline(&mut self, x0: i64, y0: i64, x1: i64, y1: i64, color: Color) {
        // Cant find original source but it's been modified for Rust from C or C++

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

    /// Draws a line using the Bresenham algorithm across two points. This second variation uses thickness
    pub fn pline2(&mut self, x0: i64, y0: i64, x1: i64, y1: i64, thickness: i64, color: Color) {

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
            // Set pixel, but as a circle
            self.pcircle(true, x0, y0, thickness, color);
    
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
        let x0 = x;
        let x1 = x + (w-1);
        let y0 = y;
        let y1 = y + h;
    
        if filled {
            for i in y0..y1 {
                self.pline(x0, i, x1, i, color);
            }
        } else {
            self.pline(x0, y0, x1, y0, color);
            self.pline(x0, y0, x0, y1, color);
            self.pline(x0, y1, x1, y1, color);
            self.pline(x1, y0, x1, y1, color);
        }
    }
    
    /// Draws a circle onto the screen. Can either be filled or outlined.
    pub fn pcircle(&mut self, filled: bool, xc: i64, yc: i64, r: i64, color: Color) { 
        let mut x: i64 = 0;
        let mut y: i64 = r; 
        let mut d: i64 = 3 - 2 * r; 
        
        if !filled {
            self.pset(xc+x, yc+y, color); 
            self.pset(xc-x, yc+y, color);
            self.pset(xc+x, yc-y, color); 
            self.pset(xc-x, yc-y, color); 
            self.pset(xc+y, yc+x, color);
            self.pset(xc-y, yc+x, color);
            self.pset(xc+y, yc-x, color); 
            self.pset(xc-y, yc-x, color);
        } else {
            self.pline(xc+x, yc+y, xc-x, yc+y, color);
            self.pline(xc+x, yc-y, xc-x, yc-y, color);
            self.pline(xc+y, yc+x, xc-y, yc+x, color);
            self.pline(xc+y, yc-x, xc-y, yc-x, color);
        }

        while y >= x
        { 
            x += 1; 
      
            if d > 0  { 
                y -= 1;  
                d = d + 4 * (x - y) + 10; 
            } else {
                d = d + 4 * x + 6;
            } 
            if !filled {
                self.pset(xc+x, yc+y, color); 
                self.pset(xc-x, yc+y, color);
                self.pset(xc+x, yc-y, color); 
                self.pset(xc-x, yc-y, color); 
                self.pset(xc+y, yc+x, color);
                self.pset(xc-y, yc+x, color);
                self.pset(xc+y, yc-x, color); 
                self.pset(xc-y, yc-x, color);
            } else {
                self.pline(xc+x, yc+y, xc-x, yc+y, color);
                self.pline(xc+x, yc-y, xc-x, yc-y, color);
                self.pline(xc+y, yc+x, xc-y, yc+x, color);
                self.pline(xc+y, yc-x, xc-y, yc-x, color);
            }
        }
    }

    /// Draws an image directly to the screen.
    pub fn pimg(&mut self, image: &Image, x: i64, y: i64) {
        for ly in 0..image.height {
            for lx in 0..image.width {
                let pc = image.pget(lx as i64, ly as i64);
                if pc.a <= 0 { continue; }
                self.pset(x + lx as i64, y + ly as i64, pc);
            }
        }
    }

    /// Draws a section of an image directly to the screen.
    pub fn pimgrect(&mut self, image: &Image, x: i64, y: i64, rx: usize, ry: usize, rw: usize, rh: usize) {
        let range_x = rx + rw;
        let range_y = ry + rh;
        for ly in ry..range_y {
            for lx in rx..range_x {
                let mlx = lx % image.width;
                let mly = ly % image.height;

                let px: i64 = (x + mlx as i64) - rx as i64;
                let py: i64 = (y + mly as i64) - ry as i64;
                
                self.pset(px, py, image.pget(mlx as i64, mly as i64));
            }
        }
    }

    /// Draws a rotated and scaled image to the screen using matrix multiplication.
    pub fn pimgmtx(&mut self, image: &Image, position_x: f64, position_y: f64, rotation: f64, scale_x: f64, scale_y: f64, offset_x: f64, offset_y: f64) {
        let position: Vector2 = Vector2::new(position_x, position_y);
        let offset: Vector2 = Vector2::new(offset_x, offset_y);
        let scale: Vector2 = Vector2::new(scale_x, scale_y);

        let mtx_o = Matrix3::translated(offset);
        let mtx_r = Matrix3::rotated(rotation);
        let mtx_p = Matrix3::translated(position);
        let mtx_s = Matrix3::scaled(scale);

        let cmtx = mtx_p * mtx_r * mtx_s * mtx_o;

        // We have to get the rotated bounding box of the rotated sprite in order to draw it correctly without blank pixels
        let start_center: Vector2 = cmtx.forward(Vector2::zero());
        let (mut sx, mut sy, mut ex, mut ey) = (start_center.x, start_center.y, start_center.x, start_center.y);

        // Top-Left Corner
        let p1: Vector2 = cmtx.forward(Vector2::zero());
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

        let mut rsx = sx as i64;
        let mut rsy = sy as i64;
        let mut rex = ex as i64;
        let mut rey = ey as i64;

        // Sprite isn't even in frame, don't draw anything
        if (rex < 0 || rsx > self.framebuffer.width as i64) && (rey < 0 || rsy > self.framebuffer.height as i64) { return; }

        // Okay but clamp the ranges in frame so we're not wasting time on stuff offscreen
        if rsx < 0 { rsx = 0;}
        if rsy < 0 { rsy = 0;}
        if rex > self.framebuffer.width as i64 { rex = self.framebuffer.width as i64; }
        if rey > self.framebuffer.height as i64 { rey = self.framebuffer.height as i64; }

        let cmtx_inv = cmtx.clone().inv();

		// We can finally draw!
		// An added 8 pixel boundry is created due to a bug(?) that clips the image drawing too early depending on rotation.
        for ly in rsy-8..rey+8 {
            for lx in rsx-8..rex+8 {
                // We have to use the inverted compound matrix (cmtx_inv) in order to get the correct pixel data from the image.
                let ip: Vector2 = cmtx_inv.forward(Vector2::new(lx as f64, ly as f64));
                let color: Color = image.pget(ip.x as i64, ip.y as i64);
                // We skip drawing entirely if the alpha is zero.
                if color.a <= 0 { continue; }
                self.pset(lx as i64, ly as i64, color);
            }
        }
    }

    /// Draws text directly to the screen using a provided font.
    pub fn pprint(&mut self, font: &Font, text: String, x: i64, y: i64) {
        let mut jumpx: isize = 0;
        let mut jumpy: isize = 0;
        let chars: Vec<char> = text.chars().collect();

        for i in 0..chars.len() {
            if chars[i] == '\n' { jumpy += font.glyph_height as isize; jumpx = 0; continue; }
            if chars[i] == ' ' { jumpx += font.glyph_width as isize; continue; }
            for j in 0..font.glyphidx.len() {
                if font.glyphidx[j] == chars[i] {
                    let rectx: usize = (j * font.glyph_width) % (font.fontimg.width);
                    let recty: usize = ((j * font.glyph_width) / font.fontimg.width) * font.glyph_height;
                    let rectw: usize = font.glyph_width;
                    let recth: usize = font.glyph_height;
                    
                    self.pimgrect(&font.fontimg, x + jumpx as i64, y + jumpy as i64, rectx, recty, rectw, recth);
                    

                    jumpx += font.glyph_width as isize + font.glyph_spacing as isize;
                }
            }
        }
    }

    /// Draws a triangle directly to the screen.
    pub fn ptriangle(&mut self, filled: bool, v1x: i64, v1y: i64, v2x: i64, v2y: i64, v3x: i64, v3y: i64, color: Color) {
        if filled {

            // Collect pixels from lines without drawing to the screen
            let vl12 = self.cline(v1x, v1y, v2x, v2y);
            let vl13 = self.cline(v1x, v1y, v3x, v3y);
            let vl23 = self.cline(v2x, v2y, v3x, v3y);

            let mut all_pixels: Vec<(i64, i64)> = Vec::new();
            for p1 in vl12 {
                all_pixels.push(p1);
            }
            for p2 in vl13 {
                all_pixels.push(p2)
            }
            for p3 in vl23 {
                all_pixels.push(p3);
            }

            let mut scanline_rows: Vec<Vec<(i64, i64)>> = vec![Vec::new(); self.framebuffer.height];

            for p in all_pixels {
                if p.1 > 0 && p.1 < self.framebuffer.height as i64 - 1{
                    scanline_rows[p.1 as usize].push(p);
                }
            }

            for row in scanline_rows {
                if row.len() == 0 { continue; }
                let height = row[0].1;
                self.pline(row[0].0, height, row[row.len()-1].0, height, color);
            }

            // Draw edges
            self.pline(v1x, v1y, v2x, v2y, color);
            self.pline(v1x, v1y, v3x, v3y, color);
            self.pline(v2x, v2y, v3x, v3y, color);
        } else {
            self.pline(v1x, v1y, v2x, v2y, color);
            self.pline(v1x, v1y, v3x, v3y, color);
            self.pline(v2x, v2y, v3x, v3y, color);
        }
    }

    /* /// Untested but comes from OneLoneCoders 3D Software Rendering series. Some help would be wonderful, I'm still very confused.
    pub fn ptritex(&mut self, image: &Image, x1: i64, y1: i64, u1: f64, v1: f64, w1: f64,
        x2: i64, y2: i64, u2: f64, v2: f64, w2: f64,
        x3: i64, y3: i64, u3: f64, v3: f64, w3: f64)                                        
    {

            // We need to put all this stuff into local scope so it doesn't break
            let mut x1: i64 = x1;
            let mut y1: i64 = y1;
            let mut u1: f64 = u1;
            let mut v1: f64 = v1;
            let mut w1: f64 = w1;

            let mut x2: i64 = x2;
            let mut y2: i64 = y2;
            let mut u2: f64 = u2;
            let mut v2: f64 = v2;
            let mut w2: f64 = w2;

            let mut x3: i64 = x3;
            let mut y3: i64 = y3;
            let mut u3: f64 = u3;
            let mut v3: f64 = v3;
            let mut w3: f64 = w3;


            if y2 < y1 {
                std::mem::swap(&mut y1, &mut y2);
                std::mem::swap(&mut x1, &mut x2);
                std::mem::swap(&mut u1, &mut u2);
                std::mem::swap(&mut v1, &mut v2);
                std::mem::swap(&mut w1, &mut w2);
            }
            if y3 < y1 {
                std::mem::swap(&mut y1, &mut y3);
                std::mem::swap(&mut x1, &mut x3);
                std::mem::swap(&mut u1, &mut u3);
                std::mem::swap(&mut v1, &mut v3);
                std::mem::swap(&mut w1, &mut w3);
            }
            if y3 < y2 {
                std::mem::swap(&mut y2, &mut y3);
                std::mem::swap(&mut x2, &mut x3);
                std::mem::swap(&mut u2, &mut u3);
                std::mem::swap(&mut v2, &mut v3);
                std::mem::swap(&mut w2, &mut w3);
            }

            let mut dy1: i64 = y2 - y1;
            let mut dx1: i64 = x2 - x1;
            let mut dv1: f64 = v2 - v1;
            let mut du1: f64 = u2 - u1;
            let mut dw1: f64 = w2 - w1;
            let mut dy2: i64 = y3 - y1;
            let mut dx2: i64 = x3 - x1;
            let mut du2: f64 = u3 - u1;
            let mut dv2: f64 = v3 - v1;
            let mut dw2: f64 = u3 - u1;
            let mut sdw2: f64 = w3 - w1;

            let mut tex_u: f64 = 0.0;
            let mut tex_v: f64 = 0.0;
            let mut tex_w: f64 = 0.0;

            let mut dax_step: f64 = 0.0;
            let mut dbx_step: f64 = 0.0;
            let mut du1_step: f64 = 0.0;
            let mut dv1_step: f64 = 0.0;
            let mut du2_step: f64 = 0.0;
            let mut dv2_step: f64 = 0.0;
            let mut dw1_step: f64 = 0.0;
            let mut dw2_step: f64 = 0.0;

            if dy1 > 0 { dax_step = dx1 as f64  / dy1.abs() as f64; }
            if dy2 > 0 { dbx_step = dx2 as f64  / dy2.abs() as f64; }
            if dy1 > 0 { du1_step = du1 as f64  / dy1.abs() as f64; }
            if dy1 > 0 { dv1_step = dv1 as f64  / dy1.abs() as f64; }
            if dy1 > 0 { dw1_step = dw1 as f64  / dy1.abs() as f64; }
            if dy2 > 0 { du2_step = du2 as f64  / dy2.abs() as f64; }
            if dy2 > 0 { dv2_step = dv2 as f64  / dy2.abs() as f64; }
            if dy2 > 0 { dw2_step = dw2 as f64  / dy2.abs() as f64; }
             // Drawing top half of triangle
            if dy1 > 0 {
                for i in y1..y2 {
                    let mut ax: i64 = (x1 as f64 + (i - y1) as f64 * dax_step) as i64;
                    let mut bx: i64 = (x1 as f64 + (i - y1) as f64 * dbx_step) as i64;
                    let mut tex_su: f64 = u1 as f64 + (i - y1) as f64 * du1_step;
                    let mut tex_sv: f64 = v1 as f64 + (i - y1) as f64 * dv1_step;
                    let mut tex_sw: f64 = w1 as f64 + (i - y1) as f64 * dw1_step;
                    let mut tex_eu: f64 = u1 as f64 + (i - y1) as f64 * du2_step;
                    let mut tex_ev: f64 = v1 as f64 + (i - y1) as f64 * dv2_step;
                    let mut tex_ew: f64 = w1 as f64 + (i - y1) as f64 * dw2_step;
                    if ax > bx {
                        std::mem::swap(&mut ax, &mut bx);
                        std::mem::swap(&mut tex_su, &mut tex_eu);
                        std::mem::swap(&mut tex_sv, &mut tex_ev);
                        std::mem::swap(&mut tex_sw, &mut tex_ew);
                    }

                    tex_u = tex_su;
                    tex_v = tex_sv;
                    tex_w = tex_sw;

                    let mut tstep: f64 = 1.0 / (bx - ax) as f64;
                    let mut t: f64 = 0.0;
                    for j in ax..bx {
                        tex_u = (1.0 - t) * tex_su + t * tex_eu;
                        tex_v = (1.0 - t) * tex_sv + t * tex_ev;
                        tex_w = (1.0 - t) * tex_sw + t * tex_ew;
                        //if tex_w > self.dget(j, i) {
                            let px: i64 = (tex_u / tex_w) as i64;
                            let py: i64 = (tex_v / tex_w) as i64;
                            let color = image.pget(px, py);

                            self.pset(j as i64, i as i64, color);
                            //self.dset(j, i, tex_w);
                        //}
                        t += tstep;
                    }
                }
            }

            // Drawing bottom half of triangle
            dy1 = y3 - y2;
            dx1 = x3 - x2;
            dv1 = v3 - v2;
            du1 = u3 - u2;
            dw1 = w3 - w2;
            if dy1 > 0 { dax_step = dx1 as f64 / dy1.abs() as f64; }
            if dy2 > 0 { dbx_step = dx2 as f64 / dy2.abs() as f64; }

            du1_step = 0.0;
            dv1_step = 0.0;

            if dy1 > 0 { du1_step = du1 / dy1.abs() as f64; }
            if dy1 > 0 { dv1_step = dv1 / dy1.abs() as f64; }
            if dy1 > 0 { dw1_step = dw1 / dy1.abs() as f64; }
            if dy1 > 0 {
                for i in y2..y3 {
                    let mut ax: i64 = ((x2 as f64 + (i - y2) as f64) * dax_step) as i64;
                    let mut bx: i64 = ((x1 as f64 + (i - y1) as f64) * dbx_step) as i64;
                    let mut tex_su: f64 = u2 + ((i - y2) as f64) * du1_step;
                    let mut tex_sv: f64 = v2 + ((i - y2) as f64) * dv1_step;
                    let mut tex_sw: f64 = w2 + ((i - y2) as f64) * dw1_step;
                    let mut tex_eu: f64 = u1 + ((i - y1) as f64) * du2_step;
                    let mut tex_ev: f64 = v1 + ((i - y1) as f64) * dv2_step;
                    let mut tex_ew: f64 = w1 + ((i - y1) as f64) * dw2_step;
                    if ax > bx {
                        std::mem::swap(&mut ax, &mut bx);
                        std::mem::swap(&mut tex_su, &mut tex_eu);
                        std::mem::swap(&mut tex_sv, &mut tex_ev);
                        std::mem::swap(&mut tex_sw, &mut tex_ew);
                    }
                    tex_u = tex_su;
                    tex_v = tex_sv;
                    tex_w = tex_sw;
                    let mut tstep: f64 = 1.0 / (bx - ax) as f64;
                    let mut t: f64 = 0.0;
                    for j in ax..bx {
                        tex_u = (1.0 - t) * tex_su + t * tex_eu;
                        tex_v = (1.0 - t) * tex_sv + t * tex_ev;
                        tex_w = (1.0 - t) * tex_sw + t * tex_ew;
                        //if tex_w > self.dget(j, i) {
                            let px: i64 = (tex_u / tex_w) as i64;
                            let py: i64 = (tex_v / tex_w) as i64;
                            let color = image.pget(px, py);

                            self.pset(j as i64, i as i64, color);
                            //self.dset(j, i, tex_w);
                        //}
                        t += tstep;
                    }
                }
            }
    } */

    /// Draws a quadratic beizer curve onto the screen.
    pub fn pbeizer(&mut self, thickness: i64, x0: i64, y0: i64, x1: i64, y1: i64, mx: i64, my: i64, color: Color) {
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

            let px0 = lerpf(x0, mx, step);
            let py0 = lerpf(y0, my, step);

            let px1 = lerpf(px0, x1, step);
            let py1 = lerpf(py0, y1, step);

            self.pcircle(true, px1 as i64, py1 as i64, thickness, color);
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

            let px0 = lerpf(x0, mx0, step);
            let py0 = lerpf(y0, my0, step);

            let px1 = lerpf(px0, mx1, step);
            let py1 = lerpf(py0, my1, step);

            let px2 = lerpf(px1, x1, step);
            let py2 = lerpf(py1, y1, step);

            self.pset(px2 as i64, py2 as i64, color);
            step += stride;
        }
    }


    // Collecting versions of drawing functions

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