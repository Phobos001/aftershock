use crate::all::*;

pub type PSetOp = fn(&mut Rasterizer, usize, Color);

#[derive(Debug, Clone)]
pub struct FrameBuffer {
    pub width: usize,
    pub height: usize,
    pub color: Vec<u8>,
}

impl FrameBuffer {
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

    pub fn blit(&mut self, fbuf_blit: &FrameBuffer, offset_x: usize, offset_y: usize) {
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
    }
}

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
}

fn pset_opaque(rasterizer: &mut Rasterizer, idx: usize, color: Color) {
    if color.a < 255 { return; }
    let color = color * rasterizer.tint;
    rasterizer.framebuffer.color[idx + 0] = color.r;  // R
    rasterizer.framebuffer.color[idx + 1] = color.g;  // G
    rasterizer.framebuffer.color[idx + 2] = color.b;  // B
    rasterizer.framebuffer.color[idx + 3] = color.a;  // A
    rasterizer.drawn_pixels_since_cls += 1;
}

fn pset_alpha(rasterizer: &mut Rasterizer, idx: usize, color: Color) {
    if color.a <= 0 { return; }
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

fn pset_inverted_opaque(rasterizer: &mut Rasterizer, idx: usize, color: Color) {
    if color.a < 255 { return; }
    let c = (color * rasterizer.tint).inverted();

    rasterizer.framebuffer.color[idx + 0] = c.r;  // R
    rasterizer.framebuffer.color[idx + 1] = c.g;  // G
    rasterizer.framebuffer.color[idx + 2] = c.b;  // B
    rasterizer.framebuffer.color[idx + 3] = c.a;  // A
    rasterizer.drawn_pixels_since_cls += 1;
}

#[derive(Clone)]
pub struct Rasterizer {
    pset_op: PSetOp,
    pub framebuffer: FrameBuffer,
    pub draw_mode: DrawMode,
    pub tint: Color,
    pub mtx: Mat3,
    pub opacity: f32,
    pub rotation: f32,
    pub camera_x: f32,
    pub camera_y: f32,
    pub scale_x: f32,
    pub scale_y: f32,
    pub camera_enabled: bool,
    pub drawn_pixels_since_cls: u64,
    pub time_since_cls: std::time::Duration,
}




/// Fullscreen drawing surface with immediate-mode drawing. More than one Rasterizer can be made and combined together
/// later for more advanced graphical applications.
///
/// # Examples
///
/// ```
/// use aftershock::rasterizer::Rasterizer;
/// 
/// let rasterizer = aftershock::Rasterizer::new(384, 216);
/// let color: [u8; 4] = [0, 255, 255, 255]
/// rasterizer.pset(128, 128, color);
/// let drawn_color: [u8; 4] = rasterizer.pget(128, 128);
///
/// assert_eq!(color, drawn_color);
/// ```
impl Rasterizer {

    /// Makes a new Rasterizer to draw to a screen-sized buffer
    ///
    /// # Examples
    ///
    /// ```
    /// use aftershock::rasterizer::Rasterizer;
    /// let rasterizer = Rasterizer::new(384, 216, false);
    /// ```
    pub fn new(width: usize, height: usize) -> Rasterizer {
        println!("Rasterizer: {} x {} x {}, Memory: {}B", width, height, 4, (width * height * 4) + { width * height * 8 });
        Rasterizer {
            pset_op: pset_opaque,
            framebuffer: FrameBuffer::new(width, height),
            draw_mode: DrawMode::Alpha,
            tint: Color::white(),
            mtx: Mat3::identity(),
            opacity: 1.0,
            camera_x: 0.0,
            camera_y: 0.0,
            camera_enabled: true,
            rotation: 0.0,
            scale_x: 1.0,
            scale_y: 1.0,
            drawn_pixels_since_cls: 0,
            time_since_cls: std::time::Duration::new(0, 0),
        }
    }

    // We don't use this match directly inside pset because it helps reduce branching in hot code
    pub fn set_draw_mode(&mut self, mode: DrawMode) {
        match mode {
            DrawMode::Opaque => {self.pset_op = pset_opaque;},
            DrawMode::Alpha => {self.pset_op = pset_alpha;},
            DrawMode::Addition => {self.pset_op = pset_addition;},
            DrawMode::Multiply => {self.pset_op = pset_multiply;}
            DrawMode::InvertedAlpha => {self.pset_op = pset_inverted_alpha;}
            DrawMode::InvertedOpaque => {self.pset_op = pset_inverted_opaque;}
            _ => {},
        }
    }

    pub fn set_custom_draw_mode(&mut self, op: fn(&mut Rasterizer, usize, Color)) {
        self.pset_op = op;
    }

    pub fn update_mtx(&mut self) {
        let rmtx_r = Mat3::rotated(self.rotation);
        let rmtx_o = Mat3::translated(Vec2::new(-(self.framebuffer.width as f32) / 2.0, -(self.framebuffer.height as f32) / 2.0));
        let rmtx_no = Mat3::translated(Vec2::new((self.framebuffer.width as f32) / 2.0, (self.framebuffer.height as f32) / 2.0));
        let rmtx_s = Mat3::scaled(Vec2::new(self.scale_x, self.scale_y));
        let rmtx_p = Mat3::translated(Vec2::new(self.camera_x, self.camera_y));
        // Offset -> Scaling -> Rotation -> Position -> Reverse Offset
        self.mtx = rmtx_no * rmtx_p * rmtx_r * rmtx_s * rmtx_o;
    }

    pub fn cls(&mut self) {
        self.framebuffer.color = vec![0; self.framebuffer.width * self.framebuffer.height * 4];
        self.drawn_pixels_since_cls = 0;
    }

    pub fn cls_color(&mut self, color: Color) {
        self.framebuffer.color.chunks_exact_mut(4).for_each(|c| {
            c[0] = color.r;
            c[1] = color.g;
            c[2] = color.b;
            c[3] = color.a;
        });
        self.drawn_pixels_since_cls = 0;
    }

    

    /// Draws a pixel to the color buffer.
    ///
    /// # Examples
    ///
    /// ```
    /// use aftershock::rasterizer::Rasterizer;
    /// 
    /// let mut rasterizer = Rasterizer::new(256, 256, true);
    /// let color: [u8; 4] = [255, 128, 64, 255];
    /// rasterizer.pset(128, 128, color);
    /// assert_eq!(rasterizer.pget(128, 128)[1], color[1]);
    /// ```
    pub fn pset(&mut self, x: i32, y: i32, color: Color) {
        /* let x = if self.camera_enabled { -self.camera_x as i32 + x } else { x };
        let y = if self.camera_enabled { -self.camera_y as i32 + y } else { y }; */
        
        let idx: usize = ((y * (self.framebuffer.width as i32) + x) * 4) as usize;
        
        let out_left: bool = x < 0;
        let out_right: bool = x > (self.framebuffer.width) as i32 - 1;
        let out_top: bool = y < 0;
        let out_bottom: bool = y > (self.framebuffer.height) as i32 - 1;
        let out_of_range: bool = idx > (self.framebuffer.width * self.framebuffer.height * 4) - 1;

        if out_of_range || out_left || out_right || out_top || out_bottom  { return; }
        
        // We have to put paraenthesis around the fn() variables or else the compiler will think it's a method.
        (self.pset_op)(self, idx, color);
    }

    /// Gets a color from the color buffer, defined in [u8; 4]
    ///
    /// # Examples
    ///
    /// ```
    /// let rasterizer = aftershock::Rasterizer::new(800, 600);
    /// let color = [255, 128, 64, 255];
    /// rasterizer.pset(128, 128, color);
    /// assert_eq!(rasterizer.pget(128, 128), color);
    /// ```
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
    
    // Thanks michael and bresenham!
    // https://stackoverflow.com/questions/34440429/draw-a-line-in-a-bitmap-possibly-with-piston
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

    // Returns array of pixels from line
    pub fn pline_collect(&mut self, x0: i32, y0: i32, x1: i32, y1: i32) -> Vec<(i32, i32)> {

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
    
    
    
    pub fn prectfill(&mut self,x: i32, y: i32, w: i32, h: i32, color: Color) {
        let x0 = x;
        let x1 = x + w;
        let y0 = y;
        let y1 = y + h;
    
        for i in y0..y1 {
            self.pline(x0, i, x1, i, color);
        }
    }
    
    pub fn prectline(&mut self, x: i32, y: i32, w: i32, h: i32, color: Color) {
        let x0 = x;
        let x1 = x + w;
        let y0 = y;
        let y1 = y + h;
    
        self.pline(x0, y0, x1, y0, color);
        self.pline(x0, y0, x0, y1, color);
        self.pline(x0, y1, x1, y1, color);
        self.pline(x1, y0, x1, y1, color);
    
    }
    
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

        while (y >= x)
        { 
            x += 1; 
      
            if (d > 0)  { 
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

    pub fn pimg(&mut self, image: &Image, x: i32, y: i32) {
        for ly in 0..image.height {
            for lx in 0..image.width {
                let pc = image.pget(lx as i32, ly as i32);
                if pc.a <= 0 { continue; }
                self.pset(x + lx as i32, y + ly as i32, pc);
            }
        }
    }

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

    /// Pre-rasterizes printable text, returning an image that can be redisplayed, attached to a sprite, or transformed
    /// in any other way. This allocates a rasterizer with a framebuffer in function scope and you should not use this every frame.
    /// For constantly changing text, you should have a dedicated rasterizer and use FrameBuffers to_image_buffer instead.
    pub fn pprint_to_img(&mut self, font: &Font, text: String, x: i32, y: i32) -> Option<Image> {
        let mut jumpx: isize = 0;
        let mut jumpy: isize = 0;
        let chars: Vec<char> = text.chars().collect();

        for i in 0..chars.len() {
            if chars[i] == '\n' { jumpy += font.glyph_height as isize; jumpx = 0; continue; }
            if chars[i] == ' ' { jumpx += font.glyph_width as isize; continue; }
        }

        let max_width = jumpx as i32 + font.glyph_width as i32;
        let max_height = jumpy as i32 + font.glyph_height as i32;

        if max_width <= 0 || max_height <= 0 { return None; }

        let mut textrast = Rasterizer::new(max_width as usize, max_height as usize);
        textrast.pprint(font, text, x, y);
        let image = textrast.framebuffer.to_image();
        return Some(image);
    }

    pub fn ptriline(&mut self,v1x: i32, v1y: i32, v2x: i32, v2y: i32, v3x: i32, v3y: i32, color: Color) {
        self.pline(v1x, v1y, v2x, v2y, color);
        self.pline(v1x, v1y, v3x, v3y, color);
        self.pline(v2x, v2y, v3x, v3y, color);
    }
    
    pub fn ptrifill(&mut self, v1x: i32, v1y: i32, v2x: i32, v2y: i32, v3x: i32, v3y: i32, color: Color) {
        // Collect pixels from lines without drawing to the screen
        let vl12 = self.pline_collect(v1x, v1y, v2x, v2y);
        let vl13 = self.pline_collect(v1x, v1y, v3x, v3y);
        let vl23 = self.pline_collect(v2x, v2y, v3x, v3y);

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
        
    }

    // Looks like shit but kinda works. Gonna bring back the OLC code form before
    pub fn ptritex2(&mut self, texture: &Image, v1x: i32, v1y: i32, v1u: f32, v1v: f32,
         v2x: i32, v2y: i32, v2u: f32, v2v: f32,
          v3x: i32, v3y: i32, v3u: f32, v3v: f32) {
        // Collect pixels from lines without drawing to the screen
        let vl12 = self.pline_collect(v1x, v1y, v2x, v2y);
        let vl13 = self.pline_collect(v1x, v1y, v3x, v3y);
        let vl23 = self.pline_collect(v2x, v2y, v3x, v3y);

        let mut all_pixels: Vec<(i32, i32, f32, f32)> = Vec::new();

        let vl12_len = vl12.len();
        let vl13_len = vl13.len();
        let vl23_len = vl23.len();

        for i in 0..vl12_len {
            all_pixels.push((vl12[i].0, vl12[i].1, lerpf(v1u, v2u, (i as f32 / vl12_len as f32)), lerpf(v1v, v2v, (i as f32 / vl12_len as f32))));
        }
        for i in 0..vl13_len {
            all_pixels.push((vl13[i].0, vl13[i].1, lerpf(v1u, v3u, (i as f32 / vl13_len as f32)), lerpf(v1v, v3v, (i as f32 / vl13_len as f32))));
        }
        for i in 0..vl23_len {
            all_pixels.push((vl23[i].0, vl23[i].1, lerpf(v3u, v3u, (i as f32 / vl23_len as f32)), lerpf(v2v, v3v, (i as f32 / vl23_len as f32))));
        }

        // Sort by row
        all_pixels.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

        let mut scanline_rows: Vec<Vec<(i32, i32, f32, f32)>> = vec![Vec::new(); self.framebuffer.height];

        for p in all_pixels {
            if p.1 > 0 && p.1 < self.framebuffer.height as i32 - 1 {
                scanline_rows[p.1 as usize].push(p);
            }
        }

        for row in scanline_rows {
            if row.len() == 0 { continue; }
            let height = row[0].1;

            let rowlast = row.len()-1;

            let utex0 = row[0].2 * texture.width as f32;
            let vtex0 = row[0].3 * texture.height as f32;

            let utex1 = row[rowlast].2 * texture.width as f32;
            let vtex1 = row[rowlast].3 * texture.height as f32;
            
            
            let rx1 = row[rowlast].0;
            for rx0 in row[0].0..rx1 {

                let step = rx0 as f32 / rx1 as f32;
                let px = lerpf(utex0, utex1, step) as i32;
                let py = lerpf(vtex0, vtex1, step) as i32;
                let color = texture.pget(px, py);
                self.pset(rx0, height, color);
            }
        }
        
    }
    
}


