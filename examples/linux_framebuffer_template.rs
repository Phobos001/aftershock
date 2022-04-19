/// This needs to be booted in a TTY terminal to work

extern crate framebuffer;

use framebuffer::{Framebuffer, KdMode};

use std::time::Instant;

use aftershock::rasterizer::*;
use aftershock::color::*;
use aftershock::drawables::*;
use aftershock::image::*;
use aftershock::font::*;
use aftershock::vector2::*;

pub struct TemplateEngine {
    pub render_width: usize,
    pub render_height: usize,

    pub fb0: Framebuffer,
    pub rast0: Rasterizer,
    pub rast1: Rasterizer,

    pub controls: u8,
    pub controls_last: u8,

    pub realtime: f32,
    pub timescale: f32,
    pub tics: u64,
    pub fps: u64,
    pub fps_print: u64,
    pub dt: f32,
    pub dt_unscaled: f32,

    dt_before: Instant,

}

impl TemplateEngine {
    pub fn new() -> TemplateEngine {
        let mut framebuffer = Framebuffer::new("/dev/fb0").unwrap();
        let width = framebuffer.var_screen_info.width as usize;
        let height = framebuffer.var_screen_info.height as usize;

        let render_width: usize = 640;
        let render_height: usize = 360;

        TemplateEngine {
            render_width,
            render_height,

            fb0: framebuffer,
            rast0: Rasterizer::new(render_width, render_height),
            rast1: Rasterizer::new(render_width, render_height),

            controls: 0,
            controls_last: 0,
            
            dt: 0.0,
            dt_unscaled: 0.0,
            dt_before: Instant::now(),
            realtime: 0.0,
            timescale: 1.0,
            tics: 0,
            fps: 0,
            fps_print: 0,
		}
	}

    pub fn run(&mut self) -> u8 {

        let mut rast1_img: Image = Image::new_with_size(self.rast1.framebuffer.width, self.rast1.framebuffer.height);

        // ==== Actual engine stuff ====
		// Font for drawing FPS and such

        let font_glyphidx = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!?*^&()[]<>-+=/\\\"'`~:;,.%abcdefghijklmnopqrstuvwxyz";
        let sysfont: Font = Font::new("core/tiny_font.png", font_glyphidx, 5, 5, -1);

        // Spritefont example. More flexible than the standard pprint but also more expensive.
        let mut spritefont_test = SpriteFont::new("core/tiny_font.png", font_glyphidx, 5, 5, 7.0, 8.0);
        spritefont_test.position = Vector2::new(256.0, 128.0);

        // Image example
        let scotty = Image::new("core/scotty_transparent.png");

        // Image example for transparency
        let graphics_and_shit = Image::new("core/default.png");

        let mut printtime: f32 = 0.0;

        'running: loop {
            self.update_times();

            // Print FPS and DT info
            printtime += self.dt_unscaled;
            if printtime > 1.0 {
                self.fps_print = self.fps;
                self.fps = 0;
                printtime = 0.0;
            }

            // == GRAPHICS ==
            self.rast1.clear_color(Color::hsv(self.realtime * 20.0, 1.0, 0.5));

            // High level spritefont example
            spritefont_test.tint = Color::hsv(self.realtime * 360.0, 1.0, 1.0);
            spritefont_test.scale = Vector2::one() * self.realtime.cos();
            spritefont_test.text = "THIS IS MY BROTHER SCOTTY\nHE IS THE BEST BROTHER EVER!".to_string();
            spritefont_test.draw(&mut self.rast1);
            

            // Image Drawing but T R A N S P A R E N T
            self.rast1.set_draw_mode(DrawMode::Alpha);
            self.rast1.opacity = 128;
            //
            self.rast1.pimg(&graphics_and_shit, 256 + ((self.realtime.cos()) * 128.0) as i32, 160);
            self.rast1.pimg(&graphics_and_shit, 256 + ((-self.realtime.cos()) * 128.0) as i32, 160);
            
            self.rast1.opacity = 255;
            self.rast1.set_draw_mode(DrawMode::Opaque);

            let total_pixels = self.rast1.drawn_pixels_since_clear;
            self.rast1.pprint(&sysfont, format!("{:.1}ms  ({} UPS) pxd: {}", (self.dt * 100000.0).ceil() / 100.0, self.fps_print, total_pixels), 0, 0);

            
            
            // Present to screen
            self.rast1.framebuffer.to_image_buffer(&mut rast1_img.buffer);

            // We need to scale our back rasterizer to our front one, since the screen size will be different than the rasterizer (Most of the time)
            self.rast0.pimgmtx(&rast1_img, 0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0);

            println!("{}x{} vs {}x{}", self.render_width, self.render_height, self.fb0.var_screen_info.width, self.fb0.var_screen_info.height);
            self.fb0.write_frame(&self.rast0.framebuffer.color);
            
            // Book keeping
            self.tics += 1;
            self.fps += 1;

            std::thread::sleep(std::time::Duration::from_micros(1));
        }


        
        return 0;
    }

    pub fn update_times(&mut self) {
        let now = Instant::now();
        let now_s = (now.elapsed().as_secs() as f32) + (now.elapsed().subsec_nanos() as f32 * 1.0e-9);
        let before_s = (self.dt_before.elapsed().as_secs() as f32) + (self.dt_before.elapsed().subsec_nanos() as f32 * 1.0e-9);
        self.dt_unscaled = before_s - now_s;
        
        self.dt_before = Instant::now();
        if self.dt_unscaled < 0.0 {
            self.dt_unscaled = 0.0;
        }
        self.dt = self.dt_unscaled * self.timescale;
        self.realtime += self.dt_unscaled;
    }

}

pub fn main() {
    
    let mut engine = TemplateEngine::new();

    let args: Vec<_> = std::env::args().collect();
    for arg in args {
        match arg.as_str() {
            //"--exclusive" => { engine.video_mode = VideoMode::Exclusive; },
            _ => {}
        }
    }

    

    let _error_code = engine.run();

    //Reenable text mode in current tty
    let _ = Framebuffer::set_kd_mode(KdMode::Text).unwrap();
}
