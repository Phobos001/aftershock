/// Higher-Level drawing functions to get started with.
use crate::color::*;
use crate::vector2::*;
use crate::font::*;
use crate::rasterizer::*;

pub struct SpriteFontGlyph {
    pub glyph: char,
    pub image: Rasterizer,
}

/// Sprite-based font which offers more flexibility for drawing than the standard Font.
/// Currently not feature complete.
pub struct SpriteFont {
    pub glyphs: Vec<SpriteFontGlyph>,
    pub glyphidx: Vec<char>,
    pub text: String,
    pub spacing_horizontal: f64,
    pub spacing_vertical: f64,

    pub tint: Color,
    pub opacity: u8,

    pub position: Vector2,
    pub scale: Vector2,
    pub rotation: f64,
    pub offset: Vector2,
}

impl SpriteFont {
    pub fn new(path_image: &str, glyphidxstr: &str, glyph_width: usize, glyph_height: usize, glyph_spacing_horizontal: f64, glyph_spacing_vertical: f64) -> Result<SpriteFont, String> {
        let font_result = Font::new(path_image, glyphidxstr, glyph_width, glyph_height, 0);
        if font_result.is_ok() {
            let font = font_result.unwrap();
            
            let mut generated_glpyhs: Vec<SpriteFontGlyph> = Vec::new();
            let glyphidx: Vec<char> = font.glyphidx.clone();

            for c in &font.glyphidx {
                let mut font_splitter: Rasterizer = Rasterizer::new(font.glyph_width, font.glyph_height);
                font_splitter.clear();
                font_splitter.pprint(&font, c.to_string(), 0, 0, glyph_spacing_vertical as i64, None);

                generated_glpyhs.push(
                    SpriteFontGlyph {
                        glyph: *c,
                        image: font_splitter,
                    }
                )
            }
            Ok(SpriteFont {
                glyphs: generated_glpyhs,
                glyphidx,
                text: "".to_string(),
                spacing_horizontal: glyph_spacing_horizontal,
                spacing_vertical: glyph_spacing_vertical,
    
                tint: Color::white(),
                opacity: 255,
    
                position: Vector2::ZERO,
                scale: Vector2::ONE,
                rotation: 0.0,
                offset: Vector2::ZERO,
            })
        } else {
            Err(font_result.err().unwrap())
        }

        
    }

    pub fn draw(&self, rasterizer: &mut Rasterizer) {
        let mut jumpx: f64 = 0.0;
        let mut jumpy: f64 = 0.0;
        let chars: Vec<char> = self.text.chars().collect();

        for i in 0..chars.len() {
            if chars[i] == '\n' { jumpy += self.spacing_vertical; jumpx = 0.0; continue; }
            if chars[i] == ' ' { jumpx += self.spacing_horizontal; continue; }
            rasterizer.set_draw_mode(DrawMode::Alpha);
            for j in 0..self.glyphs.len() {
                if self.glyphs[j].glyph == chars[i] {
                    let newpos: Vector2 = (self.position + Vector2::new(jumpx, jumpy)) * self.scale;
                    
                    rasterizer.tint = self.tint;
                    rasterizer.opacity = self.opacity;
                    rasterizer.pimgmtx(&self.glyphs[j].image, newpos.x, newpos.y,
                        self.rotation,
                        self.scale.x, self.scale.y,
                        self.offset.x, self.offset.y);
                    
                    rasterizer.tint = Color::white();
                    rasterizer.opacity = 255;
                    

                    jumpx += self.spacing_horizontal;
                }
            }
            rasterizer.set_draw_mode(DrawMode::Opaque);
        }
    }
}