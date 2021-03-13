use aftershock::rasterizer::*;
use aftershock::vector2::*;
use aftershock::color::*;
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
        TemplateEngine {

            rasterizer: Rasterizer::new(RENDER_WIDTH, RENDER_HEIGHT),

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

        // Init SDL and surface texture
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

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

        // This is what we update our buffers to
        let mut screentex = texture_creator.create_texture_streaming(PixelFormatEnum::RGBA32, RENDER_WIDTH as u32, RENDER_HEIGHT as u32)
            .map_err(|e| e.to_string()).unwrap();

        canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();
        let mut event_pump = sdl_context.event_pump().unwrap();

        // ==== Actual engine stuff ====
		// Font for drawing FPS and such

        let font_glyphidx = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!?*^&()[]<>-+=/\\\"'`~:;,.%abcdefghijklmnopqrstuvwxyz";
        let sysfont: Font = Font::new("core/tiny_font.png", font_glyphidx, 5, 5, -1);

        // Spritefont example. More flexible than the standard pprint but also more expensive.
        let mut spritefont_test = SpriteFont::new("core/tiny_font.png", font_glyphidx, 5, 5, 7.0, 8.0);
        spritefont_test.position = Vec2::new(256.0, 128.0);

        // Image example
        let scotty = Image::new("core/scotty_transparent.png");

        // Image example for transparency
        let graphics_and_shit = Image::new("core/default.png");

        let mut printtime: f32 = 0.0;

        let mut mouse_x: f32 = 0.0;
        let mut mouse_y: f32 = 0.0;

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

            let mouse_state = event_pump.mouse_state();

            // Mouse Updating
            let display_mode = video_subsystem.current_display_mode(0).unwrap();
            let (window_width, window_height) = canvas.window().size();
            let screen_width = display_mode.w as f32;
            let screen_height = display_mode.h as f32;

            mouse_x = mouse_state.x() as f32;
            mouse_y = mouse_state.y() as f32;

            let mouse_scalar_x = RENDER_WIDTH as f32 / window_width as f32;
            let mouse_scalar_y = RENDER_HEIGHT as f32 / window_height as f32;

            mouse_x *= mouse_scalar_x;
            mouse_y *= mouse_scalar_y;

            // Print FPS and DT info
            printtime += self.dt_unscaled;
            if printtime > 1.0 {
                self.fps_print = self.fps;
                self.fps = 0;
                printtime = 0.0;
            }

            // == GRAPHICS ==
            self.rasterizer.cls_color(Color::hsv(self.realtime * 20.0, 1.0, 0.5));

            // High level spritefont example
            spritefont_test.tint = Color::hsv(self.realtime * 360.0, 1.0, 1.0);
            spritefont_test.scale = Vec2::one() * self.realtime.cos();
            spritefont_test.text = "THIS IS MY BROTHER SCOTTY\nHE IS THE BEST BROTHER EVER!".to_string();
            spritefont_test.draw(&mut self.rasterizer);

            
            // Image Drawing
            self.rasterizer.pimg(&scotty, mouse_x as i32, mouse_y as i32);

            // Image Drawing but T R A N S P A R E N T
            self.rasterizer.set_draw_mode(DrawMode::Alpha);
            self.rasterizer.opacity = 128;
            self.rasterizer.pimg(&graphics_and_shit, 256 + ((self.realtime.cos()) * 128.0) as i32, 160);
            self.rasterizer.pimg(&graphics_and_shit, 256 + ((-self.realtime.cos()) * 128.0) as i32, 160);
            self.rasterizer.opacity = 255;
            self.rasterizer.set_draw_mode(DrawMode::Opaque);

            let total_pixels = self.rasterizer.drawn_pixels_since_cls;
            self.rasterizer.pprint(&sysfont, format!("{:.1}ms  ({} UPS) pxd: {}", (self.dt * 100000.0).ceil() / 100.0, self.fps_print, total_pixels), 0, 0);
            
            // Present to screen
            let _ = screentex.update(None, &self.rasterizer.framebuffer.color, (RENDER_WIDTH * 4) as usize);
            let _ = canvas.copy(&screentex, None, None);
            canvas.present();
            
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
    let mut hardware_accelerated: bool = true;

    let args: Vec<_> = std::env::args().collect();
    for arg in args {
        match arg.as_str() {
            "--exclusive" => { engine.video_mode = VideoMode::Exclusive; },
            "--fullscreen" => { engine.video_mode = VideoMode::Fullscreen; },
            "--windowed" => { engine.video_mode = VideoMode::Windowed; },
            "--software-canvas" => { hardware_accelerated = false; }
            _ => {}
        }
    }

    // Hardware accelerated windows will update faster than native window compositors, but both perform similarly
    let _error_code = engine.run(hardware_accelerated);
}
