use aftershock::rasterizer::*;
use aftershock::vectors::*;
use aftershock::matricies::*;
use aftershock::math::*;
use aftershock::color::*;
use aftershock::drawables::*;
use aftershock::random::*;
use aftershock::assets::*;

use std::time::Instant;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::{PixelFormatEnum};

const RENDER_WIDTH: usize = 512;
const RENDER_HEIGHT: usize = 512;

const CONTROL_ROTATE_LEFT: u8   = 0;
const CONTROL_ROTATE_RIGHT: u8  = 1;
const CONTROL_THRUST_FORWARD: u8     = 2;
const CONTROL_THRUST_BACKWARD: u8   = 3;
const CONTROL_FIRE: u8   = 4;
const CONTROL_PAUSE: u8  = 5;
const CONTROL_DEBUG_COLLISION: u8 = 6;
const CONTROL_DEBUG_INFO: u8 = 7;

pub enum VideoMode {
    Exclusive,
    Fullscreen,
    Windowed,
}

pub struct Player {
    pub velocity: Vec2,
    pub position: Vec2,
    pub rotation: f32,
    pub radius: f32,
    pub scale: Vec2,
}

pub struct Asteroids {
    pub velocity: Vec<Vec2>,
    pub position: Vec<Vec2>,
    pub rotation: Vec<f32>,
    pub radius: Vec<f32>,
    pub scale: Vec<Vec2>,
}

impl Asteroids {
    pub fn new() -> Asteroids {
        Asteroids {
            velocity: Vec::new(),
            position: Vec::new(),
            rotation: Vec::new(),
            radius: Vec::new(),
            scale: Vec::new(),
        }
    }

    pub fn spawn(&mut self, position: Vec2, velocity: Vec2, rotation: f32,  radius: f32, scale: Vec2) {
        self.position.push(position);
        self.velocity.push(velocity);
        self.rotation.push(rotation);
        self.radius.push(radius);
        self.scale.push(scale);
    }
}

impl Bullets {
    pub fn new() -> Bullets {
        Bullets {
            velocity: Vec::new(),
            position: Vec::new(),
            rotation: Vec::new(),
            radius: Vec::new(),
            scale: Vec::new(),
        }
    }
}

pub struct Bullets {
    pub velocity: Vec<Vec2>,
    pub position: Vec<Vec2>,
    pub rotation: Vec<f32>,
    pub radius: Vec<f32>,
    pub scale: Vec<Vec2>,
}

pub struct AsteroidsEngine {

    pub player: Player,
    pub asteroids: Asteroids,
    pub bullets: Bullets,

    pub rasterizer: Rasterizer,

    pub video_mode: VideoMode,

    pub controls: u8,
    pub controls_last: u8,

    pub paused: bool,

    pub rng: Random,
    pub rng_number: f32,

    pub debug_collision: bool,
    pub debug_info: bool,

    pub realtime: f32,
    pub timescale: f32,
    pub tics: u64,
    pub fps: u64,
    pub fps_print: u64,
    pub dt: f32,
    pub dt_unscaled: f32,

    dt_before: Instant,
}

impl AsteroidsEngine {
    pub fn new() -> AsteroidsEngine {
        println!("== OH BOY ITS ANOTHER ASTEROIDS EXAMPLE ==");

        AsteroidsEngine {
            rng: Random::new(0, 0),
            rng_number: 0.0,

            player: Player { velocity: Vec2::new(0.0, 0.0), position: Vec2::new(256.0, 256.0), rotation: 0.0, radius: 8.0, scale: Vec2::one() },
            asteroids: Asteroids::new(),
            bullets: Bullets::new(),

            rasterizer: Rasterizer::new(RENDER_WIDTH, RENDER_HEIGHT),

            video_mode: VideoMode::Windowed,

            controls: 0,
            controls_last: 0,

            debug_collision: false,
            debug_info: false,

            paused: false,
            
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

        let title = "Asteroids!";
        let window = {
            match self.video_mode {
                VideoMode::Exclusive => {
                    video_subsystem
                    .window(title, RENDER_WIDTH as u32, RENDER_HEIGHT as u32)
                    .fullscreen()
                    .position_centered()
                    .build()
                    .unwrap()
                },
                VideoMode::Fullscreen => {
                    video_subsystem
                    .window(title, RENDER_WIDTH as u32, RENDER_HEIGHT as u32)
                    .fullscreen_desktop()
                    .position_centered()
                    .build()
                    .unwrap()
                },
                VideoMode::Windowed => {
                    video_subsystem
                    .window(title, RENDER_WIDTH as u32, RENDER_HEIGHT as u32)
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

        self.rasterizer.wrapping = true;

        // ==== Actual engine stuff ====
		// Font for drawing FPS and such

		let font_glyphidx = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!?*^&()[]<>-+=/\\\"'`~:;,.%abcdefghijklmnopqrstuvwxyz";
		let mut sys_spritefont: SpriteFont = SpriteFont::new("core/tiny_font.png", font_glyphidx, 5, 5, 7.0, 14.0);

        let mut printtime: f32 = 0.0;
        'running: loop {
            self.update_times();
            
			for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..} => {
                        break 'running
                    },
                    _ => {},
                }
            }

            printtime += self.dt_unscaled;
            if printtime > 1.0 {
                self.fps_print = self.fps;
                self.fps = 0;
                printtime = 0.0;
                self.rng_number = self.rng.randf();
            }

            // == CONTROLS ==
            self.update_controls(&event_pump);

            if self.is_control_pressed(CONTROL_PAUSE) {
                self.paused = !self.paused;
                if !self.paused {
                    self.timescale = 1.0;
                } else {
                    self.timescale = 0.0;
                }
            }

            if self.is_control_pressed(CONTROL_DEBUG_INFO) {
                self.debug_info = !self.debug_info;
            }

            if self.is_control_pressed(CONTROL_DEBUG_COLLISION) {
                self.debug_collision = !self.debug_collision;
            }

            // == UPDATING ==
            // We HAVE to do this in case the window is resized, otherwise the screen texture would override anything in the window anyways
            canvas.clear();
            
            if !self.paused {
                if self.is_control_down(CONTROL_ROTATE_LEFT) {
                    self.player.rotation += self.dt * 3.0;
                }
    
                if self.is_control_down(CONTROL_ROTATE_RIGHT) {
                    self.player.rotation -= self.dt * 3.0;
                }
    
                if self.is_control_down(CONTROL_THRUST_FORWARD) {
                    let direction = Mat3::rotated(self.player.rotation);
    
                    // We accelerate instead of speed up, so we multiply by dt here as well as when we update the position
                    // Also we assume right is the starting rotation direction because of how we draw the player
                    self.player.velocity += direction.forward(Vec2::right() * 64.0) * self.dt;
                }
    
                if self.is_control_down(CONTROL_THRUST_BACKWARD) {
                    let direction = Mat3::rotated(self.player.rotation);
    
                    // We accelerate instead of speed up, so we multiply by dt here as well as when we update the position
                    // Also we assume right is the starting rotation direction because of how we draw the player
                    self.player.velocity -= direction.forward(Vec2::right() * 64.0) * self.dt;
                }

                self.player.position += self.player.velocity * self.dt;
                self.player.position %= RENDER_WIDTH as f32;
            }

            // == DRAWING ==
            self.rasterizer.cls();
            self.draw_player();

            if self.paused {
                self.rasterizer.set_draw_mode(DrawMode::Alpha);
                self.rasterizer.opacity = if modf(self.realtime, 0.5) > 0.25 { 255 } else { 0 };
                self.rasterizer.prectangle(true, 232, 232, 16, 32, Color::white());
                self.rasterizer.prectangle(true, 256, 232, 16, 32, Color::white());
                self.rasterizer.opacity = 255;
                self.rasterizer.set_draw_mode(DrawMode::Opaque);
            }

            if self.debug_info {
                self.draw_performance_text(&mut sys_spritefont);
            };

            if self.debug_collision {
                self.draw_debug_collision();
            }


            // == END OF GAME LOOP ==
            
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

    pub fn is_control_down(&mut self, control: u8) -> bool {
        return self.controls & (1 << control) != 0;
    }

    pub fn is_control_pressed(&mut self, control: u8) -> bool {
        !(self.controls_last & (1 << control) != 0) && (self.controls & (1 << control) != 0)
    }

    pub fn is_control_released(&mut self, control: u8) -> bool {
        (self.controls_last & (1 << control) != 0) && !(self.controls & (1 << control) != 0)
    }

    pub fn update_controls(&mut self, event_pump: &sdl2::EventPump) {
        let keys: Vec<Keycode> = event_pump.keyboard_state().pressed_scancodes().filter_map(Keycode::from_scancode).collect();

        self.controls_last = self.controls;
        self.controls = 0;
        
        for key in keys.iter() {
            match key {
                Keycode::Left => { self.controls     |= 1 << CONTROL_ROTATE_LEFT; },
                Keycode::Right => { self.controls    |= 1 << CONTROL_ROTATE_RIGHT; },
                Keycode::Up => { self.controls       |= 1 << CONTROL_THRUST_FORWARD; },
                Keycode::Down => { self.controls     |= 1 << CONTROL_THRUST_BACKWARD; },
                Keycode::Space => { self.controls    |= 1 << CONTROL_FIRE; },
                Keycode::Escape => { self.controls   |= 1 << CONTROL_PAUSE; },
                Keycode::F1 => { self.controls       |= 1 << CONTROL_DEBUG_COLLISION; },
                Keycode::F2 => { self.controls       |= 1 << CONTROL_DEBUG_INFO; },
                _ => {},
            }
        }
    }

    pub fn draw_player(&mut self) {
        
        // Prepare a transformation chain to get our final transformation matrix
        let translated: Mat3 = Mat3::translated(self.player.position);
        let rotated: Mat3 = Mat3::rotated(self.player.rotation);
        let scaled: Mat3 = Mat3::scaled(self.player.scale);

        // Transformations are done in the order of right to left
        let mtx = translated * rotated * scaled;

        // Defines an arrow looking thing to represent the player
        let player_points = [
            Vec2::new(8.0, 0.0),
            Vec2::new(-8.0, 8.0),
            Vec2::new(-5.0, 0.0),
            Vec2::new(-8.0, -8.0),
        ];

        // Transform points into world space
        let mtx_line0 = mtx.forward(player_points[0]);
        let mtx_line1 = mtx.forward(player_points[1]);
        let mtx_line2 = mtx.forward(player_points[2]);
        let mtx_line3 = mtx.forward(player_points[3]);

        // Draw lines to rasterizer with wrapping
        self.rasterizer.wrapping = true;
        self.rasterizer.pline(mtx_line0.x as i32, mtx_line0.y as i32, mtx_line1.x as i32, mtx_line1.y as i32, Color::white());
        self.rasterizer.pline(mtx_line1.x as i32, mtx_line1.y as i32, mtx_line2.x as i32, mtx_line2.y as i32, Color::white());
        self.rasterizer.pline(mtx_line2.x as i32, mtx_line2.y as i32, mtx_line3.x as i32, mtx_line3.y as i32, Color::white());
        self.rasterizer.pline(mtx_line3.x as i32, mtx_line3.y as i32, mtx_line0.x as i32, mtx_line0.y as i32, Color::white());
        self.rasterizer.wrapping = false;
    }

    pub fn draw_asteroids(&mut self) {

    }

    pub fn draw_performance_text(&mut self, spritefont: &mut SpriteFont) {
        let total_pixels = self.rasterizer.drawn_pixels_since_cls;
        spritefont.text = format!("{:.1}ms  ({} UPS) pxd: {}\ncontrols: {}\n{}", (self.dt_unscaled * 100000.0).ceil() / 100.0, self.fps_print, total_pixels, self.controls, self.rng_number);
        spritefont.scale = Vec2::new(2.0, 2.0);
        spritefont.position = Vec2::new(8.0, 8.0);
        spritefont.opacity = 128;
        spritefont.draw(&mut self.rasterizer);
    }

    pub fn draw_debug_collision(&mut self) {
        self.rasterizer.pcircle(false, self.player.position.x as i32, self.player.position.y as i32, self.player.radius as i32, Color::green());
    }

    /// If the squared distance between the two points is smaller than the combined radius's squared, then the two circles are overlapping!
    pub fn circle_overlap(p1: Vec2, r1: f32, p2: Vec2, r2: f32) -> bool {
        (p2.x - p1.x) * (p2.x - p1.x) + (p2.y - p1.y) * (p2.y - p1.y) < (r1*r2) + (r1*r2)
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
    
    let mut engine = AsteroidsEngine::new();
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

    let _error_code = engine.run(hardware_accelerated);
}