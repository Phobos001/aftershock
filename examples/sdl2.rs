use aftershock::rasterizer::*;
use aftershock::vectors::*;
use aftershock::color::*;
use aftershock::audio::*;
use aftershock::drawables::*;
use aftershock::assets::*;

use std::time::Instant;

use sdl2::event::Event;

use sdl2::pixels::{PixelFormatEnum};

const RENDER_WIDTH: usize = 640;
const RENDER_HEIGHT: usize = 360;

pub enum VideoMode {
    Exclusive,
    Fullscreen,
    Windowed,
}

pub struct TemplateEngine {
    pub rasterizer: Rasterizer,
    pub audio: Audio,

    pub video_mode: VideoMode,

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

            video_mode: VideoMode::Fullscreen,
            
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

    pub fn run(&mut self, hardware_accelerated: bool) -> u8 {

        // Init SDL Stuff
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        sdl_context.mouse().show_cursor(false);
        sdl_context.mouse().set_relative_mouse_mode(true);

        println!("SDL Version: {}", sdl2::version::version());

        let window = {
            match self.video_mode {
                VideoMode::Exclusive => {
                    video_subsystem
                    .window("Aftershock!", RENDER_WIDTH as u32, RENDER_HEIGHT as u32)
                    .fullscreen()
                    .position_centered()
                    .build()
                    .unwrap()
                },
                VideoMode::Fullscreen => {
                    video_subsystem
                    .window("Aftershock!", RENDER_WIDTH as u32, RENDER_HEIGHT as u32)
                    .fullscreen_desktop()
                    .position_centered()
                    .build()
                    .unwrap()
                },
                VideoMode::Windowed => {
                    video_subsystem
                    .window("Aftershock!", RENDER_WIDTH as u32, RENDER_HEIGHT as u32)
                    .resizable()
                    .position_centered()
                    .build()
                    .unwrap()
                },
            }
        };

        let mut canvas = {
            if hardware_accelerated {
                window.into_canvas().build().map_err(|e| e.to_string()).unwrap()
            } else {
                window.into_canvas().software().build().map_err(|e| e.to_string()).unwrap()
            }
        };

        let _ = canvas.set_logical_size(RENDER_WIDTH as u32, RENDER_HEIGHT as u32);
        let texture_creator = canvas.texture_creator();

        let mut screentex = texture_creator.create_texture_streaming(PixelFormatEnum::RGBA32, RENDER_WIDTH as u32, RENDER_HEIGHT as u32)
            .map_err(|e| e.to_string()).unwrap();

        canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();
        let mut event_pump = sdl_context.event_pump().unwrap();

		// Assets

        self.audio.add("as_boot", "core/boot.wav");
        self.audio.play_oneshot("as_boot");

        let cursor: Image = Image::new("core/cursor.png");
        let default: Image = Image::new("core/default.png");

        let font_glyphidx = "ABCDEFGHIJKLMNOPQRSTuVWXYZ0123456789!?*^&()[]<>-+=/\\\"'`~:;,.%abcdefghijklmnopqrstuvwxyz";
        let sysfont: Font = Font::new("core/tiny_font.png", font_glyphidx, 5, 5, -1);

        let mut printtime: f32 = 0.0;

        let mut sprite = Sprite::new(&default, 256.0, 128.0, 1.718, 2.0, 1.0, 8.0);
        let mut spr_cursor = Sprite::new(&cursor, 128.0, 128.0, 0.0, 1.0, 1.0, 1024.0);
        spr_cursor.offset = Vec2::new(0.0, 0.0);

        'running: loop {
            self.update_times();
            

			canvas.clear();
			
			for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..} => {
                        break 'running
                    },
                    _ => {}
                }
            }

            // == Rendering ==
            self.rasterizer.cls_color(Color::new(128, 0, 0, 255));         


            // Draw cool looking text
            self.rasterizer.set_draw_mode(DrawMode::Alpha);
            self.rasterizer.opacity = (self.realtime * 4.0).sin();
            self.rasterizer.tint = aftershock::color::Color::new(0, 255, 255, 255);
            self.rasterizer.pprint(&sysfont, "CYYYAAAANNNNN!!!!!".to_string(), 256, 128);
            self.rasterizer.tint = aftershock::color::Color::new(255, 255, 255, 255);
            self.rasterizer.set_draw_mode(DrawMode::Opaque);

            // Draw rotating sprite
            self.rasterizer.set_draw_mode(DrawMode::InvertedAlpha);
            self.rasterizer.opacity = 0.25;
			self.rasterizer.tint = aftershock::color::Color::new(128, 64, 255, 255);
			
            sprite.rotation = self.realtime;
			sprite.draw(&mut self.rasterizer);
			
            self.rasterizer.tint = aftershock::color::Color::white();
            self.rasterizer.opacity = 1.0;
            self.rasterizer.set_draw_mode(DrawMode::Opaque);

			let total_pixels = self.rasterizer.drawn_pixels_since_cls;
            self.rasterizer.pprint(&sysfont, format!("{:.1}ms  ({} FPS) pxd: {}", (self.dt * 100000.0).ceil() / 100.0, self.fps_print, total_pixels), 0, 0);
			
			
			// == Present ==
            self.draw_rasterizer_to_screen(&mut screentex);
            let _ = canvas.copy(&screentex, None, None);

            canvas.present();
            
            self.tics += 1;
            self.fps += 1;

            printtime += self.dt_unscaled;
            if printtime > 1.0 {
                self.fps_print = self.fps;
                self.fps = 0;
                printtime = 0.0;
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

    pub fn draw_rasterizer_to_screen(&mut self, screentex: &mut sdl2::render::Texture) {
        let _ = screentex.update(None, &self.rasterizer.framebuffer.color, (RENDER_WIDTH * 4) as usize);
    }

}

pub fn main() {
    
    let mut ase = TemplateEngine::new();
    let mut hardware_accelerated: bool = true;

    let args: Vec<_> = std::env::args().collect();
    for arg in args {
        match arg.as_str() {
            "--exclusive" => { ase.video_mode = VideoMode::Exclusive; },
            "--fullscreen" => { ase.video_mode = VideoMode::Fullscreen; },
            "--windowed" => { ase.video_mode = VideoMode::Windowed; },
            "--software-canvas" => { hardware_accelerated = false; }
            _ => {}
        }
    }

    // Hardware accelerated windows will update faster than native window compositors, but both perform similarly
    let _error_code = ase.run(hardware_accelerated);
}