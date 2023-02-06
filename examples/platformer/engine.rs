use aftershock::buffer::*;
use aftershock::font::*;
use aftershock::color::*;

use crate::gamestate::*;
use crate::controls::*;



pub struct PlatformerEngine {
    pub screen: Buffer,
    pub gamestate: Option<GameState>,

    pub hardware_canvas: bool,
    pub integer_scaling: bool,
    pub stretch_fill: bool,
    pub fullscreen: bool,
    pub exclusive: bool,

    pub controls: Controls,

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

impl PlatformerEngine {
    pub const TITLE: &str = "Platformer Example";

    pub const RENDER_WIDTH: usize = 640;
    pub const RENDER_HEIGHT: usize = 360;

    pub fn new() -> PlatformerEngine {
        println!("== Platformer Example ==");

        // Font images will be read left-to-right, top-to-bottom. 
        // This will tell the Font what character goes to what part of the image.
        let tinyfont10_glyphidx = "ABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890!?/\\@#$%^&*()[]_-+=\"';:.";

        let main_font = match Font::new("shared_assets/tiny_font10.png", tinyfont10_glyphidx, 10, 10, 1) {
            Ok(font) => { font },
            Err(_) => { Font::default() },
        };

        PlatformerEngine {
            hardware_canvas: false,
            integer_scaling: true,
            stretch_fill: false,
            fullscreen: true,
            exclusive: false,

            main_font,

            screen: Buffer::new(PlatformerEngine::RENDER_WIDTH, PlatformerEngine::RENDER_HEIGHT),
            gamestate: Some(GameState::new()),

            controls: Controls::new(),

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

    pub fn update(&mut self) {
        let update_time_before: f64 = aftershock::timestamp();
        self.controls.update();

        if self.gamestate.is_some() {
            let gamestate = self.gamestate.as_mut().unwrap();
            gamestate.update(&self.controls, self.dt);
        }
        
        let update_time_after: f64 = aftershock::timestamp();
        


        



        self.profiling_update_time = update_time_after - update_time_before;

        self.tics += 1;

        // Give the processor a break
        std::thread::sleep(std::time::Duration::from_micros(1));
    }

    pub fn draw(&mut self) {
        let draw_time_before: f64 = aftershock::timestamp();

        if self.gamestate.is_some() {
            let gamestate = self.gamestate.as_mut().unwrap();
            gamestate.draw(&mut self.screen);
        }

        
        let draw_time_after: f64 = aftershock::timestamp();
        self.profiling_draw_time = draw_time_after - draw_time_before;

        self.screen.pprint(&self.main_font, format!("UPDATE TIME: {:.02}MS\nDRAW TIME: {:.02}MS\nTICS: {}\nRT: {:.02}s", 
        (self.profiling_update_time * 100000.0).round() / 100.0, 
        (self.profiling_draw_time * 100000.0).round() / 100.0,
        self.tics, self.realtime),
        4, 4, 10, None);

        // Crosshair
        self.screen.pcircle(false, self.controls.mouse_position.0, self.controls.mouse_position.1, 4, Color::WHITE);
        self.screen.pline(self.controls.mouse_position.0, self.controls.mouse_position.1 - 6, self.controls.mouse_position.0, self.controls.mouse_position.1 + 6, Color::WHITE);
        self.screen.pline(self.controls.mouse_position.0 - 6, self.controls.mouse_position.1, self.controls.mouse_position.0 + 6, self.controls.mouse_position.1, Color::WHITE);
    }

}