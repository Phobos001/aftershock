extern crate minifb;

use aftershock::rasterizer::*;
use aftershock::vectors::*;
use aftershock::color::*;
use aftershock::audio::*;
use aftershock::drawables::*;
use aftershock::assets::*;

use std::time::Instant;

use minifb::*;

const RENDER_WIDTH: usize = 640;
const RENDER_HEIGHT: usize = 360;

pub struct TemplateEngine {
    pub rasterizer: Rasterizer,
    pub audio: Audio,

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
        println!("Mazic: Hiiii!");
        println!("Mazic: This engine uses the following dependencies in much appreciation:");

        println!("        sdl2");
        println!("        bit_field");
        println!("\nThank you all for all your hard work!\n");

		println!("Mazic: Initializing...");

        TemplateEngine {

            rasterizer: Rasterizer::new(RENDER_WIDTH, RENDER_HEIGHT),

            audio: Audio::new(8),
            
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

		// Assets

        self.audio.add("as_boot", "core/boot.wav");
        self.audio.play_oneshot("as_boot");

        let cursor: Image = Image::new("core/cursor.png");
        let default: Image = Image::new("core/default.png");

        let font_glyphidx = "ABCDEFGHIJKLMNOPQRSTuVWXYZ0123456789!?*^&()[]<>-+=/\\\"'`~:;,.%abcdefghijklmnopqrstuvwxyz";
        let sysfont: Font = Font::new("core/tiny_font.png", font_glyphidx, 5, 5, -1);

        let mut printtime: f32 = 0.0;

        let mut sprite = Sprite::new(&default, 256.0, 128.0, 1.718, 4.0, 2.0, Color::white());


        while window.is_open() && !window.is_key_down(Key::Escape) {
    
            self.update_times();

            // == Rendering ==
            self.rasterizer.cls_color(Color::new(128, 0, 0, 255));         


            // Draw cool looking text
            self.rasterizer.set_draw_mode(DrawMode::Alpha);
            self.rasterizer.opacity = (self.realtime * 4.0).sin();
            self.rasterizer.tint = aftershock::color::Color::cyan();
            self.rasterizer.pprint(&sysfont, "CYYYAAAANNNNN!!!!!".to_string(), 256, 128);
            self.rasterizer.tint = aftershock::color::Color::white();
            self.rasterizer.set_draw_mode(DrawMode::Opaque);

            // Draw rotating sprite
            // Sprites have their own tints seperate from the Rasterizers
            self.rasterizer.set_draw_mode(DrawMode::Alpha);
            self.rasterizer.opacity = 0.25;
			
            sprite.rotation = self.realtime;
			sprite.draw(&mut self.rasterizer);
            self.rasterizer.set_draw_mode(DrawMode::Opaque);

			let total_pixels = self.rasterizer.drawn_pixels_since_cls;
            self.rasterizer.pprint(&sysfont, format!("{:.1}ms  ({} FPS) pxd: {}", (self.dt * 100000.0).ceil() / 100.0, self.fps_print, total_pixels), 0, 0);
			
			
			// == Present ==
            
            self.tics += 1;
            self.fps += 1;

            printtime += self.dt_unscaled;
            if printtime > 1.0 {
                self.fps_print = self.fps;
                self.fps = 0;
                printtime = 0.0;
            }
            
            // We unwrap here as we want this code to exit if it fails
            let colors_u32: Vec<u32> = self.rasterizer.framebuffer.color.chunks_exact(4)
                .map(|c| (c[0] as u32) << 16 | (c[1] as u32) << 8 | (c[2] as u32) << 0)
                .collect();

            window
                .update_with_buffer(colors_u32.as_slice(), RENDER_WIDTH, RENDER_HEIGHT)
                .unwrap();

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
    
    let mut ase = TemplateEngine::new();

    // Hardware accelerated windows will update faster than native window compositors, but both perform similarly
    let _error_code = ase.run();
}