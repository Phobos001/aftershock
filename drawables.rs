use crate::all::*;

pub struct Sprite<'a> {
    pub tint: Color,
    pub opacity: f32,
	pub offset: Vec2,
	pub image: &'a Image,
	pub position: Vec2,
	pub rotation: f32,
	pub scale: Vec2,
}

impl<'a> Sprite<'a> {
	pub fn new(image: &'a Image, x: f32, y: f32, a: f32, sx: f32, sy: f32, tint: Color) -> Sprite {
		Sprite {
            image,
            tint,
            opacity: 1.0,
			offset: Vec2::new(-(image.width as f32) / 2.0, -(image.height as f32) / 2.0),
			position: Vec2::new(x, y),
			rotation: a,
			scale: Vec2::new(sx, sy),
		}
	}

	pub fn draw(&self, rasterizer: &mut Rasterizer) {
        rasterizer.tint = self.tint;
        rasterizer.opacity = self.opacity;
        rasterizer.pimgmtx(self.image, self.position, self.rotation, self.scale, self.offset, true);
        rasterizer.opacity = 1.0;
        rasterizer.tint = Color::white();
	}
}

pub struct SpriteFontGlyph {
    pub glyph: char,
    pub image: Image,
}

pub struct SpriteFont {
    pub glyphs: Vec<SpriteFontGlyph>,
    pub glyphidx: Vec<char>,
    pub text: String,
    pub spacing_horizontal: f32,
    pub spacing_vertical: f32,

    pub tint: Color,
    pub opacity: f32,

    pub position: Vec2,
    pub scale: Vec2,
    pub rotation: f32,
    pub offset: Vec2,
}

impl SpriteFont {
    pub fn new(path_image: &str, glyphidxstr: &str, glyph_width: usize, glyph_height: usize, glyph_spacing_horizontal: f32, glyph_spacing_vertical: f32) -> SpriteFont {
        let font = Font::new(path_image, glyphidxstr, glyph_width, glyph_height, 0);
        if font.fontimg.buffer.len() <= 0 {
            println!("ERROR - SPRITEFONT: Font {} could not be loaded due to a missing image!", path_image);
        }

        let mut font_splitter: Rasterizer = Rasterizer::new(font.glyph_width, font.glyph_height);
        let mut generated_glpyhs: Vec<SpriteFontGlyph> = Vec::new();
        let glyphidx: Vec<char> = font.glyphidx.clone();
        for c in &font.glyphidx {
            font_splitter.cls();
            font_splitter.pprint(&font, c.to_string(), 0, 0);
            let rasterized_char = font_splitter.framebuffer.to_image();
            generated_glpyhs.push(
                SpriteFontGlyph {
                    glyph: *c,
                    image: rasterized_char,
                }
            )
        }
        SpriteFont {
            glyphs: generated_glpyhs,
            glyphidx,
            text: "".to_string(),
            spacing_horizontal: glyph_spacing_horizontal,
            spacing_vertical: glyph_spacing_vertical,

            tint: Color::white(),
            opacity: 1.0,

            position: Vec2::zero(),
            scale: Vec2::one(),
            rotation: 0.0,
            offset: Vec2::zero(),
        }
    }

    pub fn draw(&self, rasterizer: &mut Rasterizer) {
        let mut jumpx: f32 = 0.0;
        let mut jumpy: f32 = 0.0;
        let chars: Vec<char> = self.text.chars().collect();

        for i in 0..chars.len() {
            if chars[i] == '\n' { jumpy += self.spacing_vertical; jumpx = 0.0; continue; }
            if chars[i] == ' ' { jumpx += self.spacing_horizontal; continue; }
            for j in 0..self.glyphs.len() {
                if self.glyphs[j].glyph == chars[i] {
                    rasterizer.set_draw_mode(DrawMode::Alpha);
                    rasterizer.tint = self.tint;
                    rasterizer.opacity = self.opacity;
                    rasterizer.pimgmtx(&self.glyphs[j].image, 
                self.position + Vec2::new(jumpx, jumpy),
                        self.rotation,
                        self.scale,
                        self.offset,
            true);
                    rasterizer.set_draw_mode(DrawMode::Alpha);
                    rasterizer.tint = Color::white();
                    rasterizer.opacity = 1.0;
                    

                    jumpx += self.glyphs[j].image.width as f32 + self.spacing_horizontal;
                }
            }
        }
    }
}

pub struct Line {
	p1: Vec2,
	p2: Vec2,
	color: Color,
}

impl Line {
	pub fn new() -> Line {
		Line {
			p1: Vec2::zero(),
			p2: Vec2::one(),
			color: Color::new(255, 255, 255, 255),
		}
	}

	pub fn draw(&self, rasterizer: &mut Rasterizer) {
		let p1 = rasterizer.mtx.forward(self.p1);
		let p2 = rasterizer.mtx.forward(self.p2);
		rasterizer.pline(p1.x as i32, p1.y as i32, p2.x as i32, p2.y as i32, self.color);
	}
}