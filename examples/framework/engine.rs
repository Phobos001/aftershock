use aftershock::buffer::*;
use aftershock::font::*;
use mlua::prelude::*;

use crate::lua::LuaScript;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum EngineVideoMode {
    Exclusive,
    Fullscreen,
    Windowed,
}

pub struct VideoData {
    pub screen_resolution: (usize, usize),
    pub window_title:   String,
    pub mode: EngineVideoMode,
    pub stretch_fill: bool,
}

pub struct FrameworkEngine {
    pub error: Option<LuaError>,
    pub screen: Buffer,
    pub lua_main: Option<LuaScript>,

    pub pattern_test: Buffer,

    pub hardware_canvas: bool,
    pub integer_scaling: bool,
    pub stretch_fill: bool,
    pub fullscreen: bool,
    pub exclusive: bool,

    pub paused: bool,

    pub main_font: Font,

    pub tics: u64,
    pub realtime: f32,
    pub timescale: f32,

    pub dt: f32,
    pub dt_unscaled: f32,

    pub profiling_update_time: f64,
    pub profiling_draw_time: f64,
    
    pub present_time: f32,

    pub is_quitting: bool,

}

impl FrameworkEngine {
    pub const TITLE: &str = "Aftershock Framework Engine";

    pub const RENDER_WIDTH: usize = 960;
    pub const RENDER_HEIGHT: usize = 540;

    pub fn new() -> FrameworkEngine {
        println!("== {} ==", Self::TITLE);

        // Font images will be read left-to-right, top-to-bottom. 
        // This will tell the Font what character goes to what part of the image.
        let tinyfont10_glyphidx = "ABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890!?/\\@#$%^&*()[]_-+=\"';:.";

        let main_font = match Font::new("shared_assets/tiny_font10.png", tinyfont10_glyphidx, 10, 10, 1) {
            Ok(font) => { font },
            Err(_) => { Font::default() },
        };

        FrameworkEngine {
            error: None,
            screen: Buffer::new(Self::RENDER_WIDTH, Self::RENDER_HEIGHT),
            lua_main: None,

            hardware_canvas: false,
            integer_scaling: true,
            stretch_fill: false,
            fullscreen: true,
            exclusive: false,

            main_font,

            pattern_test: Buffer::new_from_image("shared_assets/patterntest.png").unwrap(),


            paused: false,
            
            dt: 0.0,
            dt_unscaled: 0.0,
            realtime: 0.0,
            timescale: 1.0,

            tics: 0,

            profiling_update_time: 0.0,
            profiling_draw_time: 0.0,

            present_time: 0.0,

            is_quitting: false,
        }

        
	}

    pub fn setup_lua(&mut self, script: &String) -> (bool, String) {
        let lua_main_result = LuaScript::new(script, 300.0, 120.0);
        if lua_main_result.is_ok() {
            self.lua_main = Some(lua_main_result.unwrap());
            (false, "Lua Initialized!".into())
        } else {
            let err = lua_main_result.err().unwrap();
            println!("{}", err);
            (true, err)
        }
    }

    pub fn update(&mut self, dt: f64) -> Result<(), LuaError> {
        let update_time_before: f64 = aftershock::timestamp();
        
        if self.lua_main.is_some() {
            let lua = self.lua_main.as_mut().unwrap();

            let update_result = lua.update(dt);
            if update_result.is_err() {
                return update_result;
            }
        }
        
        let update_time_after: f64 = aftershock::timestamp();


        self.profiling_update_time = update_time_after - update_time_before;

        self.tics += 1;

        // Give the processor a break
        std::thread::sleep(std::time::Duration::from_micros(1));

        Ok(())
    }

    pub fn draw(&mut self) -> Result<(), LuaError> {
        let draw_time_before: f64 = aftershock::timestamp();

        if self.lua_main.is_some() {
            let lua = self.lua_main.as_mut().unwrap();

            let draw_result = lua.draw();
            if draw_result.is_err() {
                return draw_result;
            }
        }
        
        
        let draw_time_after: f64 = aftershock::timestamp();
        self.profiling_draw_time = draw_time_after - draw_time_before;

        self.screen.set_draw_mode(DrawMode::InvertedBgOpaque);
        self.screen.pprint(&self.main_font, format!("UPDATE TIME: {:.02}MS\nDRAW TIME: {:.02}MS\nTICS: {}\nRT: {:.02}s", 
        (self.profiling_update_time * 100000.0).round() / 100.0, 
        (self.profiling_draw_time * 100000.0).round() / 100.0,
        self.tics, self.realtime),
        4, 4, 10, None);
        self.screen.set_draw_mode(DrawMode::Opaque);

        Ok(())

        
    }

}