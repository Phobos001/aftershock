extern crate minifb;

use aftershock::rasterizer::*;
use aftershock::vectors::*;
use aftershock::color::*;
use aftershock::drawables::*;
use aftershock::assets::*;

use std::time::Instant;

use minifb::*;

const RENDER_WIDTH: usize = 640;
const RENDER_HEIGHT: usize = 360;

pub struct TemplateEngine {
    pub rasterizer: Rasterizer,

    pub realtime: f32,
    pub timescale: f32,
    pub tics: u64,
    pub fps: u64,
    pub fps_print: u64,
    pub dt: f32,
    pub dt_unscaled: f32,

    pub game_hz: u32,
    game_hz_timer: f32,

    dt_before: Instant,

}

impl TemplateEngine {
    pub fn new() -> TemplateEngine {
        TemplateEngine {
            rasterizer: Rasterizer::new(RENDER_WIDTH, RENDER_HEIGHT),
            
            dt: 0.0,
            dt_unscaled: 0.0,
            dt_before: Instant::now(),
            realtime: 0.0,
            timescale: 1.0,
            tics: 0,
            fps: 0,
            fps_print: 0,

            game_hz: 9999, // Controls how often things are updated
            game_hz_timer: 0.0,
		}
	}

    pub fn run(&mut self) -> u8 {

        // Init MINIFB stuff
        let mut window = match Window::new(
            "Aftershock!",
            RENDER_WIDTH,
            RENDER_HEIGHT,
            WindowOptions {
                resize: false,
                scale: Scale::FitScreen,
                ..WindowOptions::default()
            },
        ) {
            Ok(win) => win,
            Err(err) => {
                println!("Unable to create window {}", err);
                return 1;
            }
        };

        // ==== Actual engine stuff ====
		// Font for drawing FPS and such

        let font_glyphidx = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!?*^&()[]<>-+=/\\\"'`~:;,.%abcdefghijklmnopqrstuvwxyz";
        let sysfont: Font = Font::new("core/tiny_font.png", font_glyphidx, 5, 5, -1);

        // Spritefont example. More flexible than the standard pprint but also more expensive.
        let mut spritefont_test = SpriteFont::new("core/tiny_font.png", font_glyphidx, 5, 5, 0.0, 8.0);
        spritefont_test.position = Vec2::new(256.0, 128.0);

        let mut printtime: f32 = 0.0;

        while window.is_open() && !window.is_key_down(Key::Escape) {
            self.update_times();

            printtime += self.dt_unscaled;
            if printtime > 1.0 {
                self.fps_print = self.fps;
                self.fps = 0;
                printtime = 0.0;
            }

            // Only update the game at a certain rate
            self.game_hz_timer -= self.dt_unscaled;
            if self.game_hz_timer <= 0.0 {
                self.rasterizer.cls_color(Color::hsv(self.realtime * 20.0, 1.0, 0.5));

                // High level spritefont example
                spritefont_test.tint = Color::hsv(self.realtime * 360.0, 1.0, 1.0);
                spritefont_test.scale = Vec2::one() * self.realtime.cos();
                spritefont_test.text = "WEEEEEEE WOOOOW WEEEEEEE\nDAAAAMMNNNNNNNSONNNNN\nWOOAAHHHHHHHH!!!!!!!!11!!1!!!".to_string();
                spritefont_test.draw(&mut self.rasterizer);

                let total_pixels = self.rasterizer.drawn_pixels_since_cls;
                self.rasterizer.pprint(&sysfont, format!("{:.1}ms  ({} UPS) pxd: {}", (self.dt * 100000.0).ceil() / 100.0, self.fps_print, total_pixels), 0, 0);
                
                // combine the colors into u32 instead of 4's of u8
                let colors_u32: Vec<u32> = self.rasterizer.framebuffer.color.chunks_exact(4)
                    .map(|c| (c[0] as u32) << 16 | (c[1] as u32) << 8 | (c[2] as u32) << 0)
                    .collect();

                // Present
                window
                    .update_with_buffer(colors_u32.as_slice(), RENDER_WIDTH, RENDER_HEIGHT)
                    .unwrap();
                
                // Book keeping
                self.tics += 1;
                self.fps += 1;

                // Set the new update delay
                self.game_hz_timer = 1.0 / self.game_hz as f32;
            }

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

    // Hardware accelerated windows will update faster than native window compositors, but both perform similarly
    let _error_code = engine.run();
}