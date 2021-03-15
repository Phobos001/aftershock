use crate::color::*;
use crate::vector2::*;
use crate::assets::*;
use crate::rasterizer::*;

/// Matrix-Transformed image drawing.
/// Allows for scaling, rotation, positioning, and shearing.
pub struct Sprite<'a> {
    pub tint: Color,
    pub opacity: u8,
	pub offset: Vector2,
	pub image: &'a Image,
	pub position: Vector2,
	pub rotation: f32,
	pub scale: Vector2,
    pub shear: Vector2,
}

impl<'a> Sprite<'a> {
	pub fn new(image: &'a Image, x: f32, y: f32, a: f32, sx: f32, sy: f32, tint: Color) -> Sprite {
		Sprite {
            image,
            tint,
            opacity: 255,
			offset: Vector2::new(-(image.width as f32) / 2.0, -(image.height as f32) / 2.0),
			position: Vector2::new(x, y),
			rotation: a,
			scale: Vector2::new(sx, sy),
            shear: Vector2::one(),
		}
	}

	pub fn draw(&self, rasterizer: &mut Rasterizer) {
        rasterizer.tint = self.tint;
        rasterizer.opacity = self.opacity;
        rasterizer.pimgmtx(self.image, self.position, self.rotation, self.scale, self.offset);
        rasterizer.opacity = 255;
        rasterizer.tint = Color::white();
	}
}

pub struct SpriteFontGlyph {
    pub glyph: char,
    pub image: Image,
}

/// Sprite-based font which offers more flexibility for drawing than the standard Font.
/// Currently not feature complete.
pub struct SpriteFont {
    pub glyphs: Vec<SpriteFontGlyph>,
    pub glyphidx: Vec<char>,
    pub text: String,
    pub spacing_horizontal: f32,
    pub spacing_vertical: f32,

    pub tint: Color,
    pub opacity: u8,

    pub position: Vector2,
    pub scale: Vector2,
    pub rotation: f32,
    pub offset: Vector2,
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
            opacity: 255,

            position: Vector2::zero(),
            scale: Vector2::one(),
            rotation: 0.0,
            offset: Vector2::zero(),
        }
    }

    pub fn draw(&self, rasterizer: &mut Rasterizer) {
        let mut jumpx: f32 = 0.0;
        let mut jumpy: f32 = 0.0;
        let chars: Vec<char> = self.text.chars().collect();

        for i in 0..chars.len() {
            if chars[i] == '\n' { jumpy += self.spacing_vertical; jumpx = 0.0; continue; }
            if chars[i] == ' ' { jumpx += self.spacing_horizontal; continue; }
            rasterizer.set_draw_mode(DrawMode::Alpha);
            for j in 0..self.glyphs.len() {
                if self.glyphs[j].glyph == chars[i] {
                    
                    rasterizer.tint = self.tint;
                    rasterizer.opacity = self.opacity;
                    rasterizer.pimgmtx(&self.glyphs[j].image, self.position + Vector2::new(jumpx, jumpy),
                        self.rotation,
                        self.scale,
                        self.offset);
                    
                    rasterizer.tint = Color::white();
                    rasterizer.opacity = 255;
                    

                    jumpx += self.spacing_horizontal;
                }
            }
            rasterizer.set_draw_mode(DrawMode::Opaque);
        }
    }
}