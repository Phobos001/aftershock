use aftershock::color::*;
use mlua::prelude::*;

#[derive(Clone, Copy, Debug)]
pub struct LuaColor {
    pub color: Color,
}

pub fn register_color(lua: &Lua) {
    println!("Registering API: Color");

    // RGB //
    let rgb_constructor = lua.create_function(|_, (r, g, b): (f32, f32, f32)| {
        let r: u8 = f32::clamp(r, 0.0, 255.0) as u8;
        let g: u8 = f32::clamp(g, 0.0, 255.0) as u8;
        let b: u8 = f32::clamp(b, 0.0, 255.0) as u8;
        let a: u8 = 255;
        Ok(LuaColor{ color: Color::new(r, g, b, a) })
    }).unwrap();
    let _ = lua.globals().set("rgb", rgb_constructor);

    // RGBA //
    let rgba_constructor = lua.create_function(|_, (r, g, b, a): (f32, f32, f32, f32)| {
        let r: u8 = f32::clamp(r, 0.0, 255.0) as u8;
        let g: u8 = f32::clamp(g, 0.0, 255.0) as u8;
        let b: u8 = f32::clamp(b, 0.0, 255.0) as u8;
        let a: u8 = f32::clamp(a, 0.0, 255.0) as u8;
        Ok(LuaColor{ color: Color::new(r, g, b, a) })
    }).unwrap();
    let _ = lua.globals().set("rgba", rgba_constructor);

    // HSV //
    let hsv_constructor = lua.create_function(|_, (hue, saturation, value): (f32, f32, f32)| {
        Ok(LuaColor{ color: Color::hsv(hue, saturation, value)} )
    }).unwrap();
    let _ = lua.globals().set("hsv", hsv_constructor);
}

impl LuaUserData for LuaColor {}