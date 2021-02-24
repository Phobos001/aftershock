use crate::all::*;

pub type PSetOp = fn(&mut Rasterizer, usize, Color);

/// Heap-Allocated Framebuffer.
#[derive(Debug, Clone)]
pub struct FrameBuffer {
    pub width: usize,
    pub height: usize,
    pub color: Vec<u8>,
}

impl FrameBuffer {
    /// Createds a new framebuffer. Normally created inside a Rasterizer during its initialization
    pub fn new(width: usize, height: usize) -> FrameBuffer {
        FrameBuffer {
            width,
            height,
            color: vec![0; width * height * 4],
        }
    }

    pub fn to_image(&self) -> Image {
        Image {
            buffer: self.color.clone(),
            width: self.width,
            height: self.height,
        }
    }

    pub fn to_image_buffer(&self, buffer: &mut Vec<u8>) {
        buffer.clear();
        if self.color.len() == buffer.len() {
            buffer.copy_from_slice(self.color.as_slice());
        }
    }

    // Depreciated, originally going to be used for a poly-rasterizer but that didn't pan out.
    /*
    pub fn blit_framebuffer(&mut self, fbuf_blit: &FrameBuffer, offset_x: usize, offset_y: usize) {
        let stride = 4;
        // We blit these directly into the color buffer because otherwise we'd just be drawing everything over again and we don't have to worry about depth
        
        // The color array is a 1D row of bytes, so we have to do this in sets of rows
        // Make sure this actually fits inside the buffer
        let extent_width: usize = offset_x + fbuf_blit.width;
        let extent_height: usize = offset_y + fbuf_blit.height;
    
        let src_height: usize = fbuf_blit.height;
        let dst_height: usize = self.height;
    
        // If this goes out of bounds at all we should not draw it
        let not_too_big: bool = self.width * self.height < fbuf_blit.width * self.height;
        let not_out_of_bounds: bool = extent_width > self.width || extent_height > self.height;
        if not_too_big && not_out_of_bounds { 
            println!("ERROR - FRAMEBUFFER BLIT: Does not fit inside target buffer!"); 
            return;
        }
    
        // Lets get an array of rows so we can blit them directly into the color buffer
        let mut rows_src: Vec<&[u8]> = Vec::new();
    
        // Build a list of rows to blit to the screen.
        fbuf_blit.color.chunks_exact(fbuf_blit.width * stride).enumerate().for_each(|(_, row)| {
            rows_src.push(row);
        });
    
        let is_equal_size: bool = self.width == fbuf_blit.width && self.height == fbuf_blit.height;
    
        // Goes through each row of fbuf and split it twice into the slice that fits our rows_src. So we 
        self.color.chunks_exact_mut(self.width * stride).enumerate().for_each(|(i, row_dst)| {
            if i >= dst_height { return; }
            if i >= offset_y && i < offset_y + src_height { 
                if is_equal_size {
                    row_dst.clone_from_slice(rows_src[i]);
                } else {
                    // We need to cut the row into a section that we can just set equal to our row
                    // Make sure that we are actually in the bounds from our source buffer
                    if i >= offset_y && i < (offset_y + rows_src.len()) {
                        // [......|#######]
                        // Split at the stride distance to get the first end
                        let rightsect = row_dst.split_at_mut(offset_x * stride).1;
        
                        // [......|####|...]
                        // Get the second half but left
                        let section = rightsect.split_at_mut((extent_width - offset_x) * stride).0;
        
                        // I HAVE YOU NOW
                        section.clone_from_slice(rows_src[i-offset_y]);
                    }
                }
            }
        });
    }*/
}

/// Controls how a rasterizer should draw incoming pixels.
#[derive(Debug, Clone, Copy)]
pub enum DrawMode {
    Opaque,
    Alpha,
    Addition,
    Subtraction,
    Multiply,
    Divide,
    InvertedAlpha,
    InvertedOpaque,
    Collect,
}

/// Draw pixels if they are fully opaque, otherwise ignore them.
fn pset_opaque(rasterizer: &mut Rasterizer, idx: usize, color: Color) {
    if color.a < 255 { return; }
    let color = color * rasterizer.tint;
    rasterizer.framebuffer.color[idx + 0] = color.r;  // R
    rasterizer.framebuffer.color[idx + 1] = color.g;  // G
    rasterizer.framebuffer.color[idx + 2] = color.b;  // B
    rasterizer.framebuffer.color[idx + 3] = color.a;  // A
    rasterizer.drawn_pixels_since_cls += 1;
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
    rasterizer.drawn_pixels_since_cls += 1;
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
    rasterizer.drawn_pixels_since_cls += 1;
}

/// Multiply incoming pixel with buffer pixel.
fn pset_multiply(rasterizer: &mut Rasterizer, idx: usize, color: Color) {
    if color.a <= 0 { return; }
    let fg = color* rasterizer.tint;
    let bg = Color::new(
        rasterizer.framebuffer.color[idx + 0],
        rasterizer.framebuffer.color[idx + 1],
        rasterizer.framebuffer.color[idx + 2],
        255,
    );

    let c = Color::blend_fast(fg, bg, rasterizer.opacity) * bg;

    rasterizer.framebuffer.color[idx + 0] = c.r;  // R
    rasterizer.framebuffer.color[idx + 1] = c.g;  // G
    rasterizer.framebuffer.color[idx + 2] = c.b;  // B
    rasterizer.framebuffer.color[idx + 3] = c.a;  // A
    rasterizer.drawn_pixels_since_cls += 1;
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
    rasterizer.drawn_pixels_since_cls += 1;
}

/// Draw inverted copy of incoming pixel as opaque
fn pset_inverted_opaque(rasterizer: &mut Rasterizer, idx: usize, color: Color) {
    if color.a < 255 { return; }
    let c = (color * rasterizer.tint).inverted();

    rasterizer.framebuffer.color[idx + 0] = c.r;  // R
    rasterizer.framebuffer.color[idx + 1] = c.g;  // G
    rasterizer.framebuffer.color[idx + 2] = c.b;  // B
    rasterizer.framebuffer.color[idx + 3] = c.a;  // A
    rasterizer.drawn_pixels_since_cls += 1;
}

/// Collect drawn pixels into collected_pixels instead of drawing them to the buffer.
/// This is useful for more advanced graphical effects, for example using a series of ptriangle's to
/// build a polygonal area, then drawing a texture onto the pixels.
fn pset_collect(rasterizer: &mut Rasterizer, idx: usize, color: Color) {
    let idx_unstrided = idx / 4;
    let x = modi(idx_unstrided as i32, rasterizer.framebuffer.width as i32);
    let y = idx_unstrided as i32 / rasterizer.framebuffer.width as i32;
    rasterizer.collected_pixels.push((x, y, color));
}

/// Drawing switchboard that draws directly into a framebuffer. Drawing options like Tint and Opacity must be manually changed by the user.
pub struct Rasterizer {
    pset_op: PSetOp,
    
    pub framebuffer: FrameBuffer,

    pub draw_mode: DrawMode,
    pub tint: Color,
    pub opacity: u8,
    pub wrapping: bool,
    pub use_fast_alpha_blend: bool,

    pub collected_pixels: Vec<(i32, i32, Color)>,

    pub camera_x: i32,
    pub camera_y: i32,
    pub drawn_pixels_since_cls: u64,
    pub time_since_cls: std::time::Duration,
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
            framebuffer: FrameBuffer::new(width, height),

            draw_mode: DrawMode::Opaque,
            tint: Color::white(),
            opacity: 255,
            wrapping: false,
            use_fast_alpha_blend: true,

            collected_pixels: Vec::new(),

            camera_x: 0,
            camera_y: 0,
            drawn_pixels_since_cls: 0,
            time_since_cls: std::time::Duration::new(0, 0),
        }
    }

    /// Sets the rasterizers drawing mode for incoming pixels. Should be defined before every drawing operation.
    /// # Arguments
    /// * 'mode' - Which drawing function should the Rasterizer use.
    pub fn set_draw_mode(&mut self, mode: DrawMode) {
        match mode {
            DrawMode::Opaque => {self.pset_op = pset_opaque;},
            DrawMode::Alpha => {self.pset_op = pset_alpha;},
            DrawMode::Addition => {self.pset_op = pset_addition;},
            DrawMode::Multiply => {self.pset_op = pset_multiply;}
            DrawMode::InvertedAlpha => {self.pset_op = pset_inverted_alpha;}
            DrawMode::InvertedOpaque => {self.pset_op = pset_inverted_opaque;}
            DrawMode::Collect => {self.pset_op = pset_collect;}
            _ => {},
        }
    }

    fn blend_color(&mut self, src: Color, dst: Color) -> Color {
        if self.use_fast_alpha_blend {
            Color::blend_fast(src, dst, self.opacity)
        } else {
            Color::blend(src, dst, self.opacity as f32 / 255.0)
        }
    }

    /// Clears the frame memory directly, leaving a black screen.
    pub fn cls(&mut self) {
        self.framebuffer.color = vec![0; self.framebuffer.width * self.framebuffer.height * 4];
        self.drawn_pixels_since_cls = 0;
    }

    /// Clears the screen to a color.
    /// # Arguments
    /// * 'color' - Color the screen should be cleared too.
    pub fn cls_color(&mut self, color: Color) {
        self.framebuffer.color.chunks_exact_mut(4).for_each(|c| {
            c[0] = color.r;
            c[1] = color.g;
            c[2] = color.b;
            c[3] = color.a;
        });
        self.drawn_pixels_since_cls = 0;
    }

    /// Clears the collected_pixels buffer. Does not resize to zero.
    pub fn cls_collected(&mut self) {
        self.collected_pixels.clear();
    }

    /// Draws a pixel to the color buffer, using the rasterizers set DrawMode. DrawMode defaults to Opaque.
    pub fn pset(&mut self, x: i32, y: i32, color: Color) {
        let mut x = -self.camera_x as i32 + x;
        let mut y = -self.camera_y as i32 + y;

        let mut idx: usize = ((y * (self.framebuffer.width as i32) + x) * 4) as usize;
        if !self.wrapping {
            let out_left: bool = x < 0;
            let out_right: bool = x > (self.framebuffer.width) as i32 - 1;
            let out_top: bool = y < 0;
            let out_bottom: bool = y > (self.framebuffer.height) as i32 - 1;
            let out_of_range: bool = idx > (self.framebuffer.width * self.framebuffer.height * 4) - 1;

            if out_of_range || out_left || out_right || out_top || out_bottom  { return; }
        } else {
            x = modi(x, self.framebuffer.width as i32);
            y = modi(y, self.framebuffer.height as i32);
            idx = ((y * (self.framebuffer.width as i32) + x) * 4) as usize;
            if idx > (self.framebuffer.width * self.framebuffer.height * 4) - 1 { return; }
        }
        
        // We have to put paraenthesis around the fn() variables or else the compiler will think it's a method.
        (self.pset_op)(self, idx, color);
    }

    /// Gets a color from the color buffer.
    pub fn pget(&mut self, x: i32, y: i32) -> Color {
        let idx: usize = (y * (self.framebuffer.width as i32) + x) as usize;

        let out_left: bool = x < 0;
        let out_right: bool = x > (self.framebuffer.width) as i32 - 1;
        let out_top: bool = y < 0;
        let out_bottom: bool = y > (self.framebuffer.height) as i32 - 1;
        let out_of_range: bool = idx > (self.framebuffer.width * self.framebuffer.height * 4) - 1;

        if out_of_range || out_left || out_right || out_top || out_bottom  { return Color::black(); }

        return Color::new(
            self.framebuffer.color[idx + 0],
            self.framebuffer.color[idx + 1],
            self.framebuffer.color[idx + 2],
            self.framebuffer.color[idx + 3]
        );
    }
    
    /// Draws a line using the Bresenham algorithm across two points
    pub fn pline(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, color: Color) {

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
    pub fn prectangle(&mut self, filled: bool, x: i32, y: i32, w: i32, h: i32, color: Color) {
        let x0 = x;
        let x1 = x + w;
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
    pub fn pcircle(&mut self, filled: bool, xc: i32, yc: i32, r: i32, color: Color) { 
        let mut x: i32 = 0;
        let mut y: i32 = r; 
        let mut d: i32 = 3 - 2 * r; 
        
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
    pub fn pimg(&mut self, image: &Image, x: i32, y: i32) {
        for ly in 0..image.height {
            for lx in 0..image.width {
                let pc = image.pget(lx as i32, ly as i32);
                if pc.a <= 0 { continue; }
                self.pset(x + lx as i32, y + ly as i32, pc);
            }
        }
    }

    /// Draws a section of an image directly to the screen.
    pub fn pimgrect(&mut self, image: &Image, x: i32, y: i32, rx: usize, ry: usize, rw: usize, rh: usize) {
        let range_x = rx + rw;
        let range_y = ry + rh;
        for ly in ry..range_y {
            for lx in rx..range_x {
                let mlx = lx % image.width;
                let mly = ly % image.height;

                let px: i32 = (x + mlx as i32) - rx as i32;
                let py: i32 = (y + mly as i32) - ry as i32;
                
                self.pset(px, py, image.pget(mlx as i32, mly as i32));
            }
        }
    }

    /// Draws a rotated and scaled image to the screen using matrix multiplication.
    pub fn pimgmtx(&mut self, image: &Image, position: Vec2, rotation: f32, scale: Vec2, offset: Vec2) {
        let mtx_o = Mat3::translated(offset);
        let mtx_r = Mat3::rotated(rotation);
        let mtx_p = Mat3::translated(position);
        let mtx_s = Mat3::scaled(scale);

        let cmtx = mtx_p * mtx_r * mtx_s * mtx_o;

        // We have to get the rotated bounding box of the rotated sprite in order to draw it correctly without blank pixels
        let start_center: Vec2 = cmtx.forward(Vec2::zero());
        let (mut sx, mut sy, mut ex, mut ey) = (start_center.x, start_center.y, start_center.x, start_center.y);

        // Top-Left Corner
        let p1: Vec2 = cmtx.forward(Vec2::zero());
        sx = f32::min(sx, p1.x); sy = f32::min(sy, p1.y);
        ex = f32::max(ex, p1.x); ey = f32::max(ey, p1.y);

        // Bottom-Right Corner
        let p2: Vec2 = cmtx.forward(Vec2::new(image.width as f32, image.height as f32));
        sx = f32::min(sx, p2.x); sy = f32::min(sy, p2.y);
        ex = f32::max(ex, p2.x); ey = f32::max(ey, p2.y);

        // Bottom-Left Corner
        let p3: Vec2 = cmtx.forward(Vec2::new(0.0, image.height as f32));
        sx = f32::min(sx, p3.x); sy = f32::min(sy, p3.y);
        ex = f32::max(ex, p3.x); ey = f32::max(ey, p3.y);

        // Top-Right Corner
        let p4: Vec2 = cmtx.forward(Vec2::new(image.width as f32, 0.0));
        sx = f32::min(sx, p4.x); sy = f32::min(sy, p4.y);
        ex = f32::max(ex, p4.x); ey = f32::max(ey, p4.y);

        let mut rsx = sx as i32;
        let mut rsy = sy as i32;
        let mut rex = ex as i32;
        let mut rey = ey as i32;

        // Sprite isn't even in frame, don't draw anything
        if (rex < 0 || rsx > self.framebuffer.width as i32) && (rey < 0 || rsy > self.framebuffer.height as i32) { return; }

        // Okay but clamp the ranges in frame so we're not wasting time on stuff offscreen
        if rsx < 0 { rsx = 0;}
        if rsy < 0 { rsy = 0;}
        if rex > self.framebuffer.width as i32 { rex = self.framebuffer.width as i32; }
        if rey > self.framebuffer.height as i32 { rey = self.framebuffer.height as i32; }

        let cmtx_inv = cmtx.clone().inv();

		// We can finally draw!
		// Noticed some weird clipping on the right side of sprites, like the BB isn't big enough? Just gonna add some more pixels down and right just in case
        for ly in rsy..rey+8 {
            for lx in rsx..rex+8 {
                // We have to use the inverted compound matrix (cmtx_inv) in order to get the correct pixel data from the image.
                let ip: Vec2 = cmtx_inv.forward(Vec2::new(lx as f32, ly as f32));
                let color: Color = image.pget(ip.x as i32, ip.y as i32);
                if color.a <= 0 { continue; }
                self.pset(lx as i32, ly as i32, color);
            }
        }
    }

    /// Draws text directly to the screen using a provided font.
    pub fn pprint(&mut self, font: &Font, text: String, x: i32, y: i32) {
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
                    
                    self.pimgrect(&font.fontimg, x + jumpx as i32, y + jumpy as i32, rectx, recty, rectw, recth);
                    

                    jumpx += font.glyph_width as isize + font.glyph_spacing as isize;
                }
            }
        }
    }

    /// Draws a triangle directly to the screen.
    pub fn ptriangle(&mut self, filled: bool, v1x: i32, v1y: i32, v2x: i32, v2y: i32, v3x: i32, v3y: i32, color: Color) {
        if filled {
            // Collect pixels from lines without drawing to the screen
            let vl12 = self.cline(v1x, v1y, v2x, v2y);
            let vl13 = self.cline(v1x, v1y, v3x, v3y);
            let vl23 = self.cline(v2x, v2y, v3x, v3y);

            let mut all_pixels: Vec<(i32, i32)> = Vec::new();
            for p1 in vl12 {
                all_pixels.push(p1);
            }
            for p2 in vl13 {
                all_pixels.push(p2)
            }
            for p3 in vl23 {
                all_pixels.push(p3);
            }

            // Sort by row
            all_pixels.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

            let mut scanline_rows: Vec<Vec<(i32, i32)>> = vec![Vec::new(); self.framebuffer.height];

            for p in all_pixels {
                if p.1 > 0 && p.1 < self.framebuffer.height as i32 - 1 {
                    scanline_rows[p.1 as usize].push(p);
                }
            }

            for row in scanline_rows {
                if row.len() == 0 { continue; }
                let height = row[0].1;
                self.pline(row[0].0, height, row[row.len()-1].0, height, color);
            }
        } else {
            self.pline(v1x, v1y, v2x, v2y, color);
            self.pline(v1x, v1y, v3x, v3y, color);
            self.pline(v2x, v2y, v3x, v3y, color);
        }
    }

    /// Draws a quadratic beizer curve onto the screen.
    pub fn pbeizer(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, mx: i32, my: i32, color: Color) {
        let mut step: f32 = 0.0;

        // Get the maximal number of pixels we will need to use and get its inverse as a step size.
        // Otherwise we don't know how many pixels we will need to draw
        let stride_c1 = self.cline(x0, y0, mx, my).len() as f32;
        let stride_c2 = self.cline(mx, my, x1, y1).len() as f32;

        let stride: f32 = (1.0 / (stride_c1 + stride_c2)) * 0.5;

        let x0 = x0 as f32;
        let y0 = x0 as f32;
        let x1 = x1 as f32;
        let y1 = y1 as f32;
        let mx = mx as f32;
        let my = my as f32;

        loop {
            if step > 1.0 { break; }

            let px0 = lerpf(x0, mx, step);
            let py0 = lerpf(y0, my, step);

            let px1 = lerpf(px0, x1, step);
            let py1 = lerpf(py0, y1, step);

            self.pset(px1 as i32, py1 as i32, color);
            step += stride;
        }
    }

    /// Draws a cubic beizer curve onto the screen.
    pub fn pbeizer_cubic(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, mx0: i32, my0: i32, mx1: i32, my1: i32, color: Color) {
        let mut step: f32 = 0.0;

        // Get the maximal number of pixels we will need to use and get its inverse as a step size.
        // Otherwise we don't know how many pixels we will need to draw
        let stride_c1: f32 = self.cline(x0, y0, mx0, my0).len() as f32;
        let stride_c2: f32 = self.cline(mx0, my0, mx1, my1).len() as f32;
        let stride_c3: f32 = self.cline(mx1, my1, x1, y1).len() as f32;

        let stride = (1.0 / (stride_c1 + stride_c2 + stride_c3)) * 0.5;

        let x0 = x0 as f32;
        let y0 = x0 as f32;
        let x1 = x1 as f32;
        let y1 = y1 as f32;
        let mx0 = mx0 as f32;
        let my0 = my0 as f32;
        let mx1 = mx1 as f32;
        let my1 = my1 as f32;

        loop {
            if step > 1.0 { break; }

            let px0 = lerpf(x0, mx0, step);
            let py0 = lerpf(y0, my0, step);

            let px1 = lerpf(px0, mx1, step);
            let py1 = lerpf(py0, my1, step);

            let px2 = lerpf(px1, x1, step);
            let py2 = lerpf(py1, y1, step);

            self.pset(px2 as i32, py2 as i32, color);
            step += stride;
        }
    }


    // Collecting versions of drawing functions

    /// Returns pixel positions across the line.
    pub fn cline(&mut self, x0: i32, y0: i32, x1: i32, y1: i32) -> Vec<(i32, i32)> {

        let mut pixels: Vec<(i32, i32)> = Vec::new();

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


