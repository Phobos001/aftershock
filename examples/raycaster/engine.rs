use aftershock::buffer::*;
use aftershock::font::*;
use aftershock::color::*;
use aftershock::vector2::*;

use crate::controls::*;
use crate::level::Level;
use crate::renderer::Renderer;


pub struct RebuiltEngine {
    pub renderer: Renderer,
    pub pattern_test: Buffer,

    pub hardware_canvas: bool,
    pub integer_scaling: bool,
    pub stretch_fill: bool,
    pub fullscreen: bool,
    pub exclusive: bool,

    pub level: Level,

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

impl RebuiltEngine {
    pub const TITLE: &str = "Rebuilt Engine";

    pub const RENDER_WIDTH: usize = 960;
    pub const RENDER_HEIGHT: usize = 540;

    pub fn new() -> RebuiltEngine {
        println!("== Raycast Engine ==");

        // Font images will be read left-to-right, top-to-bottom. 
        // This will tell the Font what character goes to what part of the image.
        let tinyfont10_glyphidx = "ABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890!?/\\@#$%^&*()[]_-+=\"';:.";

        let main_font = match Font::new("shared_assets/tiny_font10.png", tinyfont10_glyphidx, 10, 10, 1) {
            Ok(font) => { font },
            Err(_) => { Font::default() },
        };

        RebuiltEngine {
            hardware_canvas: false,
            integer_scaling: true,
            stretch_fill: false,
            fullscreen: true,
            exclusive: false,

            level: Level::new(),

            main_font,

            renderer: Renderer::new(RebuiltEngine::RENDER_WIDTH, RebuiltEngine::RENDER_HEIGHT, 1),
            pattern_test: Buffer::new_from_image("textures/prototype/prototype_grid.png").unwrap(),

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

        if self.controls.is_control_down(ControlKeys::MoveForward) {
            self.level.camera_position += Vector2::new(0.0, -1.0).rotated(-self.level.camera_rotation) * 4.0 * self.dt;
        }

        if self.controls.is_control_down(ControlKeys::MoveBackward) {
            self.level.camera_position += Vector2::new(0.0, 1.0).rotated(-self.level.camera_rotation) * 4.0 * self.dt;
        }

        if self.controls.is_control_down(ControlKeys::StrafeLeft) {
            self.level.camera_position += Vector2::new(-1.0, 0.0).rotated(-self.level.camera_rotation) * 4.0 * self.dt;
        }

        if self.controls.is_control_down(ControlKeys::StrafeRight) {
            self.level.camera_position += Vector2::new(1.0, 0.0).rotated(-self.level.camera_rotation) * 4.0 * self.dt;
        }

        if self.controls.is_control_down(ControlKeys::TurnLeft) {
            self.level.camera_rotation += (90.0 as f32).to_radians() * self.dt;
        }

        if self.controls.is_control_down(ControlKeys::TurnRight) {
            self.level.camera_rotation -= (90.0 as f32).to_radians() * self.dt;
        }
        
        let update_time_after: f64 = aftershock::timestamp();


        self.profiling_update_time = update_time_after - update_time_before;

        self.tics += 1;

        // Give the processor a break
        std::thread::sleep(std::time::Duration::from_micros(1));
    }

    pub fn draw(&mut self) {
        let draw_time_before: f64 = aftershock::timestamp();

        self.renderer.screen.clear();

        self.renderer.position = self.level.camera_position;
        self.renderer.rotation = self.level.camera_rotation;
        self.renderer.height = 1.5;
        self.renderer.look = 0.0;

        self.renderer.draw_sector(&self.level, 0);

        
        let draw_time_after: f64 = aftershock::timestamp();
        self.profiling_draw_time = draw_time_after - draw_time_before;

        self.renderer.screen.set_draw_mode(DrawMode::InvertedBgOpaque);
        self.renderer.screen.pprint(&self.main_font, format!("UPDATE TIME: {:.02}MS\nDRAW TIME: {:.02}MS\nTICS: {}\nRT: {:.02}s", 
        (self.profiling_update_time * 100000.0).round() / 100.0, 
        (self.profiling_draw_time * 100000.0).round() / 100.0,
        self.tics, self.realtime),
        4, 4, 10, None);
        self.renderer.screen.set_draw_mode(DrawMode::Opaque);

        // Cursor
        self.renderer.screen.pcircle(false, self.controls.mouse_position.0, self.controls.mouse_position.1, 4, Color::WHITE);
        self.renderer.screen.pline(self.controls.mouse_position.0, self.controls.mouse_position.1 - 6, self.controls.mouse_position.0, self.controls.mouse_position.1 + 6, Color::WHITE);
        self.renderer.screen.pline(self.controls.mouse_position.0 - 6, self.controls.mouse_position.1, self.controls.mouse_position.0 + 6, self.controls.mouse_position.1, Color::WHITE);
    }

}