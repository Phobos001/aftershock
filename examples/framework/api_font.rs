/* use aftershock::font::*;

use mlua::prelude::*;

#[derive(Clone, Copy, Debug)]
pub struct LuaFont {
    pub font: Font,
}


pub fn register_font(lua: &Lua) {
    println!("Registering API: Font");

    let fn_font_load = lua.create_function(move |_, (path_to, glyph_sequence, glyph_width, glyph_height, glyph_spacing): (String, String, f64, f64, f64)| {
        let font_result = LuaFont { font: Font::new(path_to.as_str(), glyph_sequence.as_str(), glyph_width as usize, glyph_height as usize,  glyph_spacing as i64) };
        if font_result.font.is_ok() {
            Ok(font_result.unwrap())
        } else { /* Handled by Font */ Ok(LuaFont::default()) }
        
    }).unwrap();
    let _ = lua.globals().set("load_font", fn_font_load);
} */