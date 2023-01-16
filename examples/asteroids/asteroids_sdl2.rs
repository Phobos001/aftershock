// ECS would be perfect for this but I'm trying to keep things simple.

// !!! Wrapping was removed from Aftershock on 12/13/2021 due to a noticable decrease in performance from branching

use aftershock::rasterizer::*;
use aftershock::vector2::*;
use aftershock::matrix3::*;
use aftershock::math::*;
use aftershock::color::*;
use aftershock::font::*;

use squares_rng::SquaresRNG;

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

#[derive(Debug, Clone, Copy)]
pub struct Player {
    pub active: bool,
    pub velocity: Vector2,
    pub position: Vector2,
    pub rotation: f64,
    pub radius: f64,
    pub scale: Vector2,
}

#[derive(Debug, Clone, Copy)]
pub struct Bullet {
    pub active: bool,
    pub velocity: Vector2,
    pub position: Vector2,
    pub rotation: f64,
    pub radius: f64,
    pub scale: Vector2,
    pub lifetime: f64,
}

impl Bullet {
    pub fn new() -> Bullet {
        Bullet {
            active: false,
            velocity: Vector2::ZERO,
            position: Vector2::ZERO,
            rotation: 0.0,
            radius: 0.0,
            scale: Vector2::ONE,
            lifetime: 0.0,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Asteroid {
    pub active: bool,
    pub shape: [Vector2; 8],
    pub velocity: Vector2,
    pub position: Vector2,
    pub rotation: f64,
    pub radius: f64,
    pub scale: Vector2,
    pub health: u8,
}

impl Asteroid {
    pub fn new() -> Asteroid {
        Asteroid {
            active: false,
            shape: [Vector2::ZERO; 8],
            velocity: Vector2::ZERO,
            position: Vector2::ZERO,
            rotation: 0.0,
            radius: 0.0,
            scale: Vector2::ONE,
            health: 3,
        }
    }

    pub fn generate_shape(radius: f64, rng: &mut SquaresRNG) -> [Vector2; 8] {
        let mut points: [Vector2; 8] = [
            Vector2::new(1.0, 0.0), // Right
            Vector2::new(0.5, 0.5), // Bottom Right
            Vector2::new(0.0, 1.0), // Bottom
            Vector2::new(-0.5, 0.5), // Bottom Left
            Vector2::new(-1.0, 0.0), // Left
            Vector2::new(-0.5, -0.5), // Top Left
            Vector2::new(0.0, -1.0), // Top
            Vector2::new(0.5, -0.5), // Top Right
        ];

        // Some noise to make them look more natrual
        // Does not effect collisions
        for p in points.iter_mut() {
            *p *= radius;
            *p += Vector2::new(rng.rangef64(-2.0, 2.0), rng.rangef64(-2.0, 2.0));
        }

        points
    }   
}

#[derive(Debug, Clone, Copy)]
pub struct ExplosionParticle {
    pub position: Vector2,
    pub velocity: Vector2,
    pub radius: f64,
}

impl ExplosionParticle {
    pub fn new() -> ExplosionParticle {
        ExplosionParticle {
            position: Vector2::ZERO,
            velocity: Vector2::ZERO,
            radius: 0.0,
        }
    }
}

/// If the squared distance between the two points is smaller than the combined diameter's squared, then the two circles are overlapping!
pub fn circle_overlap(p1: Vector2, r1: f64, p2: Vector2, r2: f64) -> bool {
    // Double the incoming radius's so they are diameter's instead.
    let (r1, r2) = (r1 * 2.0, r2 * 2.0);
    (p2.x - p1.x) * (p2.x - p1.x) + (p2.y - p1.y) * (p2.y - p1.y) < (r1*r2) + (r1*r2)
}

pub struct AsteroidsEngine {
    pub camera: Matrix3,
    pub camera_boomzoom: f64,

    pub player: Player,
    pub asteroids: [Asteroid; 128],
    pub bullets: [Bullet; 128],

    pub explosion_particles: [ExplosionParticle; 8192],

    pub score: i64,

    pub uidx_asteroids: usize,
    pub uidx_bullets: usize,

    pub rasterizer: Rasterizer,

    pub video_mode: VideoMode,

    pub controls: u8,
    pub controls_last: u8,

    pub paused: bool,

    pub rng: SquaresRNG,
    pub rng_number: f64,

    pub font_score: Font,

    pub debug_collision: bool,
    pub debug_info: bool,

    pub realtime: f64,
    pub timescale: f64,
    pub tics: u64,
    pub fps: u64,
    pub fps_print: u64,
    pub dt: f64,
    pub dt_unscaled: f64,

    dt_before: Instant,
}

impl AsteroidsEngine {
    pub fn new() -> AsteroidsEngine {
        println!("== OH BOY ITS ANOTHER ASTEROIDS EXAMPLE ==");

        let rng_seedcounter = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).expect("DONT. FUCK. WITH TIME.").as_secs();
        let tinyfont_glyphidx = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!?*^&()[]<>-+=/\\\"'`~:;,.%abcdefghijklmnopqrstuvwxyz";
        let tinyfont10_glyphidx = "ABCDEFGHIJKLMNOPQRSTUVWZYZ1234567890!?/\\@#$%^&*()[]_-+=\"';:.";

        let rng_key: u64 = 0xd2f6ae576fced215; // Key requires even distribution of 0's and 1's

        AsteroidsEngine {
            camera: Matrix3::identity(),
            camera_boomzoom: 1.0,

            // Use time in seconds as counter seed, and use the first RNG key in the table.
            rng: SquaresRNG::new_with_key(rng_seedcounter, rng_key), 
            rng_number: 0.0,

            player: Player { active: true, velocity: Vector2::new(0.0, 0.0), position: Vector2::new(256.0, 256.0), rotation: 0.0, radius: 4.0, scale: Vector2::ONE},
            asteroids: [Asteroid::new(); 128],
            bullets: [Bullet::new(); 128],

            explosion_particles: [ExplosionParticle::new(); 8192],

            score: 0,
            font_score: Font::new("core/tiny_font10.png", tinyfont10_glyphidx, 10, 10, 1).unwrap(),

            uidx_asteroids: 0,
            uidx_bullets: 0,

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
        let _ = canvas.set_integer_scale(true);
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
		let mut sysfont: Font = Font::new("core/tiny_font.png", font_glyphidx, 5, 5, 1).unwrap();
    

        let mut printtime: f64 = 0.0;
        let mut present_time: f64 = 0.0;

        'running: loop {
            self.update_times();
            present_time -= self.dt_unscaled;
            
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
                self.rng_number = self.rng.randf64();
            }

            self.update_controls(&event_pump);
            

            if self.is_control_pressed(CONTROL_PAUSE) {
                self.paused = !self.paused;
                if !self.paused {
                    self.timescale = 1.0;
                } else {
                    self.timescale = 0.0;
                }
            }

            if self.is_control_pressed(CONTROL_DEBUG_COLLISION) {
                self.debug_collision = !self.debug_collision;
            }

            if self.is_control_pressed(CONTROL_DEBUG_INFO) {
                self.debug_info = !self.debug_info;
            }

            // == UPDATING ==
            
            if !self.paused {
                self.update_player();
                self.update_asteroids();
                self.update_bullets();
                self.update_explosion_particles();
                self.update_camera();
            }

            // == DRAWING ==


            // We HAVE to do this in case the window is resized, otherwise the screen texture would override anything in the window anyways
            

            if present_time <= 0.0 {
                self.rasterizer.clear();

                self.draw_explosion_particles();

                self.draw_score();
                self.draw_bullets();
                self.draw_asteroids();
                self.draw_player();

                if self.paused {
                    self.rasterizer.set_draw_mode(DrawMode::Alpha);
                    self.rasterizer.opacity = if self.realtime.rem_euclid(0.5) > 0.25 { 255 } else { 0 };
                    self.rasterizer.prectangle(true, 232, 232, 16, 32, Color::white());
                    self.rasterizer.prectangle(true, 256, 232, 16, 32, Color::white());
                    self.rasterizer.opacity = 255;
                    self.rasterizer.set_draw_mode(DrawMode::Opaque);
                }

                if self.debug_info {
                    self.draw_performance_text();
                };

                if self.debug_collision {
                    self.draw_debug_collision();
                }


                // == END OF GAME LOOP ==
                
                // Present to screen
                let _ = screentex.update(None, &self.rasterizer.color, (RENDER_WIDTH * 4) as usize);
                let _ = canvas.copy(&screentex, None, None);
                canvas.present();

                present_time = 1.0 / match hardware_accelerated { true => { 240.0 } false => { 120.0 } };
            }
            
            
            // Book keeping
            self.tics += 1;
            self.fps += 1;

            // Give the processor a break
            std::thread::sleep(std::time::Duration::from_micros(1));
        }
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
                Keycode::Left   => { self.controls  |= 1 << CONTROL_ROTATE_LEFT; },
                Keycode::Right  => { self.controls  |= 1 << CONTROL_ROTATE_RIGHT; },
                Keycode::Up     => { self.controls  |= 1 << CONTROL_THRUST_FORWARD; },
                Keycode::Down   => { self.controls  |= 1 << CONTROL_THRUST_BACKWARD; },

                // WASD Alternative
                Keycode::A      => { self.controls  |= 1 << CONTROL_ROTATE_LEFT; },
                Keycode::D      => { self.controls  |= 1 << CONTROL_ROTATE_RIGHT; },
                Keycode::W      => { self.controls  |= 1 << CONTROL_THRUST_FORWARD; },
                Keycode::S      => { self.controls  |= 1 << CONTROL_THRUST_BACKWARD; },

                Keycode::Space  => { self.controls  |= 1 << CONTROL_FIRE; },
                Keycode::Escape => { self.controls  |= 1 << CONTROL_PAUSE; },
                Keycode::F1     => { self.controls  |= 1 << CONTROL_DEBUG_COLLISION; },
                Keycode::F2     => { self.controls  |= 1 << CONTROL_DEBUG_INFO; },
                _ => {},
            }
        }
    }

    ///// ====== PLAYER ====== /////

    pub fn kill_player(&mut self) {
        self.player.active = false;
    }

    pub fn update_player(&mut self) {
        if self.player.active {
            if self.is_control_down(CONTROL_ROTATE_LEFT) {
                self.player.rotation += self.dt * 3.0;
            }
    
            if self.is_control_down(CONTROL_ROTATE_RIGHT) {
                self.player.rotation -= self.dt * 3.0;
            }
    
            if self.is_control_down(CONTROL_THRUST_FORWARD) {
                let direction = Matrix3::rotated(self.player.rotation);
    
                // We accelerate instead of speed up, so we multiply by dt here as well as when we update the position
                // Also we assume right is the starting rotation direction because of how we draw the player
                self.player.velocity += direction.forward(Vector2::RIGHT * 64.0) * self.dt;
            }
    
            if self.is_control_down(CONTROL_THRUST_BACKWARD) {
                let direction = Matrix3::rotated(self.player.rotation);
    
                // We accelerate instead of speed up, so we multiply by dt here as well as when we update the position
                // Also we assume right is the starting rotation direction because of how we draw the player
                self.player.velocity -= direction.forward(Vector2::RIGHT * 64.0) * self.dt;
            }
    
            if self.is_control_pressed(CONTROL_FIRE) {
                let direction = Matrix3::rotated(self.player.rotation).forward(Vector2::RIGHT);
                let offset = Matrix3::translated(self.player.position).forward(direction * 16.0);
                
                self.spawn_bullet(offset, direction.normalized(), self.player.velocity.magnitude() + 128.0);
            }
    
            self.player.position += self.player.velocity * self.dt;
            self.player.position.x = self.player.position.x.rem_euclid(RENDER_WIDTH as f64);
            self.player.position.y = self.player.position.y.rem_euclid(RENDER_HEIGHT as f64);

            // Check if we are touching any asteroids or bullets. Escape if player is already dead
            for i in 0..self.asteroids.len() {
                if self.player.active && self.asteroids[i].active {
                    if circle_overlap(self.player.position, self.player.radius, self.asteroids[i].position, self.asteroids[i].radius) {
                        self.kill_player();
                        break;
                    }
                }
            }

            for i in 0..self.bullets.len() {
                if self.player.active && self.bullets[i].active {
                    if circle_overlap(self.player.position, self.player.radius, self.bullets[i].position, self.bullets[i].radius) {
                        self.kill_player();
                        self.bullets[i].active = false;
                        break;
                    }
                }
            }
        } else {
            if self.is_control_pressed(CONTROL_FIRE) {
                self.restart_game();
            }
        }
        
    }

    pub fn draw_player(&mut self) {
        // Prepare a transformation chain to get our final transformation matrix
        let translated: Matrix3 = Matrix3::translated(self.player.position);
        let rotated: Matrix3 = Matrix3::rotated(self.player.rotation);
        let scaled: Matrix3 = Matrix3::scaled(self.player.scale);

        // Transformations are done in the order of right to left
        let mtx = self.camera * translated * rotated * scaled;

        // Defines an arrow looking thing to represent the player
        let player_points = [
            Vector2::new(8.0, 0.0),
            Vector2::new(-8.0, 8.0),
            Vector2::new(-5.0, 0.0),
            Vector2::new(-8.0, -8.0),
        ];

        // Transform points into world space
        let mtx_line0 = mtx.forward(player_points[0]);
        let mtx_line1 = mtx.forward(player_points[1]);
        let mtx_line2 = mtx.forward(player_points[2]);
        let mtx_line3 = mtx.forward(player_points[3]);

        // Draw lines to rasterizer with wrapping

        let player_color: Color = {
            if self.player.active {
                Color::white()
            } else {
                Color::red()
            }
        };

        //self.rasterizer.wrapping = true;
        self.rasterizer.pline(mtx_line0.x as i64, mtx_line0.y as i64, mtx_line1.x as i64, mtx_line1.y as i64, player_color);
        self.rasterizer.pline(mtx_line1.x as i64, mtx_line1.y as i64, mtx_line2.x as i64, mtx_line2.y as i64, player_color);
        self.rasterizer.pline(mtx_line2.x as i64, mtx_line2.y as i64, mtx_line3.x as i64, mtx_line3.y as i64, player_color);
        self.rasterizer.pline(mtx_line3.x as i64, mtx_line3.y as i64, mtx_line0.x as i64, mtx_line0.y as i64, player_color);
        //self.rasterizer.wrapping = false;
    }

    ///// ====== PLAYER ====== /////

    ///// ====== EFFECTS ====== /////

    pub fn spawn_explosion(&mut self, position: Vector2) {
        let mut spawn_counter = 32;
        for i in 0..self.explosion_particles.len() {
            if spawn_counter > 0 {
                if self.explosion_particles[i].radius <= 0.1 {
                    self.explosion_particles[i].radius = self.rng.rangef64(4.0, 16.0);
                    self.explosion_particles[i].position = position;
                    self.explosion_particles[i].velocity = Vector2::new(
                        self.rng.rangef64(-1.0, 1.0),
                        self.rng.rangef64(-1.0, 1.0)
                    ) * self.rng.rangef64(8.0, 256.0);
                    spawn_counter -= 1;
                }
            } else {
                break;
            }
        }
    }

    pub fn update_explosion_particles(&mut self) {
        for i in 0..self.explosion_particles.len() {
            if self.explosion_particles[i].radius > 0.1 {
                self.explosion_particles[i].position += self.explosion_particles[i].velocity * self.dt;
                self.explosion_particles[i].radius = lerpf(self.explosion_particles[i].radius, 0.0, 0.5 * self.dt);
                self.explosion_particles[i].velocity = Vector2::lerp(self.explosion_particles[i].velocity, Vector2::UP, 1.0 * self.dt);
            }
            
        }
    }

    pub fn draw_explosion_particles(&mut self) {
        for i in 0..self.explosion_particles.len() {
            if self.explosion_particles[i].radius > 0.1 {
                let mtx_position = self.camera.forward(self.explosion_particles[i].position);
                self.rasterizer.pcircle(false, 
                    mtx_position.x as i64,
                    mtx_position.y as i64,
                    (self.explosion_particles[i].radius * self.camera_boomzoom) as i64, Color::hsv(0.1, 1.0, self.explosion_particles[i].radius / 8.0)
                );
            }
        }
    }

    ///// ====== EFFECTS ====== /////

    ///// ====== CAMERA ====== /////

    pub fn update_camera(&mut self) {
        self.camera_boomzoom = lerpf(self.camera_boomzoom, 1.0, 5.0 * self.dt);
        let camera_scaled = Matrix3::scaled(Vector2::ONE * self.camera_boomzoom);
        // We need to move the camera closer to the center based on zoom since it's technically in the top-left corner
        let camera_translated = Matrix3::translated(
            Vector2::new(
                lerpf(RENDER_WIDTH as f64 / 2.0, 0.0, self.camera_boomzoom), 
                lerpf(RENDER_HEIGHT as f64 / 2.0, 0.0, self.camera_boomzoom)
            ));

        self.camera = camera_scaled * camera_translated;
    }
    
    pub fn camera_impact_effect(&mut self) {
        self.camera_boomzoom = 1.0333;
    }

    ///// ====== CAMERA ====== /////




    ///// ====== BULLET ====== /////

    pub fn spawn_bullet(&mut self, offset: Vector2, direction: Vector2, force: f64) {
        let mut bullet = &mut self.bullets[self.uidx_bullets % self.bullets.len()];

        bullet.position = offset;
        bullet.velocity = direction * force;
        bullet.radius = 2.0;
        bullet.scale = Vector2::ONE;
        bullet.rotation = 0.0; // Really if the bullets don't end up being dots this would be useful, otherwise not really.
        bullet.active = true;    // Kinda expensive to do it this way but it's simple

        self.uidx_bullets += 1;
        
        self.remove_score(5);
    }

    pub fn update_bullets(&mut self) {

        for i in 0..self.bullets.len() {
            if self.bullets[i].active {
                self.bullets[i].position += self.bullets[i].velocity * self.dt;
                self.bullets[i].position.x = self.bullets[i].position.x.rem_euclid(RENDER_WIDTH as f64);
                self.bullets[i].position.y = self.bullets[i].position.y.rem_euclid(RENDER_HEIGHT as f64);
                self.bullets[i].lifetime += self.dt;
                if self.bullets[i].lifetime > 5.0 {
                    self.bullets[i].active = false;
                }
            }
        }
    }

    pub fn draw_bullets(&mut self) {
        for bullet in &self.bullets {
            if bullet.active {
                let mtx_position = self.camera.forward(bullet.position);
                self.rasterizer.pcircle(true, mtx_position.x as i64, mtx_position.y as i64, bullet.radius as i64, Color::white());
            }   
        }
    }

    ///// ====== BULLET ====== /////




    ///// ====== ASTEROID ====== /////

    pub fn spawn_asteroid(&mut self) {
        let mut asteroid = &mut self.asteroids[self.uidx_asteroids % self.asteroids.len()];

        asteroid.radius = self.rng.rangef64(4.0, 16.0);
        asteroid.shape = Asteroid::generate_shape(asteroid.radius, &mut self.rng);
        asteroid.position = Vector2::new(self.rng.rangef64(0.0, RENDER_WIDTH as f64), self.rng.rangef64(0.0, RENDER_HEIGHT as f64));
        asteroid.rotation = self.rng.rangef64(0.0, 6.28);
        asteroid.velocity = Vector2::new(self.rng.rangef64(-1.0, 1.0), self.rng.rangef64(-1.0, 1.0)) * self.rng.rangef64(2.0, 64.0);
        asteroid.active = true;

        self.uidx_asteroids += 1;
    }

    pub fn update_asteroids(&mut self) {

        // Update active asteroids. Also keep track of how many are active.
        let mut asteroids_active: u32 = 0;
        for i in 0..self.asteroids.len() {
            if self.asteroids[i].active { 
                asteroids_active += 1;

                self.asteroids[i].position += self.asteroids[i].velocity * self.dt;
                self.asteroids[i].position.x = self.asteroids[i].position.x.rem_euclid(RENDER_WIDTH as f64);
                self.asteroids[i].position.y = self.asteroids[i].position.y.rem_euclid(RENDER_HEIGHT as f64);

                

                for j in 0..self.bullets.len() {
                    let bullet = &mut self.bullets[j];
                    if bullet.active && circle_overlap(bullet.position, bullet.radius, self.asteroids[i].position, self.asteroids[i].radius) {
                        self.asteroids[i].health -= 1;
                        if self.asteroids[i].health <= 0 {
                            self.asteroids[i].active = false;
                            bullet.active = false;
                            self.camera_impact_effect();
                            self.spawn_explosion(self.asteroids[i].position);

                            self.add_score(25);
                        }
                    }
                }
            }        
        }

        if asteroids_active <= 0 {
            for _ in 0..8 {
                self.spawn_asteroid();
            }
        }
    }

    pub fn draw_asteroids(&mut self) {
        for asteroid in &self.asteroids {
            if asteroid.active {
                // Prepare a transformation chain to get our final transformation matrix
                let translated: Matrix3 = Matrix3::translated(asteroid.position);
                let rotated: Matrix3 = Matrix3::rotated(asteroid.rotation);
                let scaled: Matrix3 = Matrix3::scaled(asteroid.scale);

                // Transformations are done in the order of right to left
                let mtx = self.camera * translated * rotated * scaled;

                // Transform points into world space
                let mtx_line0 = mtx.forward(asteroid.shape[0]);
                let mtx_line1 = mtx.forward(asteroid.shape[1]);
                let mtx_line2 = mtx.forward(asteroid.shape[2]);
                let mtx_line3 = mtx.forward(asteroid.shape[3]);
                let mtx_line4 = mtx.forward(asteroid.shape[4]);
                let mtx_line5 = mtx.forward(asteroid.shape[5]);
                let mtx_line6 = mtx.forward(asteroid.shape[6]);
                let mtx_line7 = mtx.forward(asteroid.shape[7]);

                // Draw lines to rasterizer with wrapping
                //self.rasterizer.wrapping = true;
                self.rasterizer.pline(mtx_line0.x as i64, mtx_line0.y as i64, mtx_line1.x as i64, mtx_line1.y as i64, Color::white());
                self.rasterizer.pline(mtx_line1.x as i64, mtx_line1.y as i64, mtx_line2.x as i64, mtx_line2.y as i64, Color::white());
                self.rasterizer.pline(mtx_line2.x as i64, mtx_line2.y as i64, mtx_line3.x as i64, mtx_line3.y as i64, Color::white());
                self.rasterizer.pline(mtx_line3.x as i64, mtx_line3.y as i64, mtx_line4.x as i64, mtx_line4.y as i64, Color::white());
                self.rasterizer.pline(mtx_line4.x as i64, mtx_line4.y as i64, mtx_line5.x as i64, mtx_line5.y as i64, Color::white());
                self.rasterizer.pline(mtx_line5.x as i64, mtx_line5.y as i64, mtx_line6.x as i64, mtx_line6.y as i64, Color::white());
                self.rasterizer.pline(mtx_line6.x as i64, mtx_line6.y as i64, mtx_line7.x as i64, mtx_line7.y as i64, Color::white());
                self.rasterizer.pline(mtx_line7.x as i64, mtx_line7.y as i64, mtx_line0.x as i64, mtx_line0.y as i64, Color::white());
                //self.rasterizer.wrapping = false;
            }
            
        }
    }

    ///// ====== ASTEROID ====== /////

    


    ///// ====== ENGINE ====== /////

    pub fn add_score(&mut self, score: i64) {
        self.score += score;
    }

    pub fn remove_score(&mut self, score: i64) {
        self.score -= score;
        if self.score < 0 {
            self.score = 0;
        }
    }

    pub fn draw_score(&mut self) {
        self.rasterizer.set_draw_mode(DrawMode::Alpha);
        self.rasterizer.opacity = 200;
        self.rasterizer.tint = Color::hsv(self.realtime * 10.0, 1.0, 1.0);
        self.rasterizer.pprint(&self.font_score, format!("{:0>8}", self.score), 232, 248, 0, None);
        self.rasterizer.tint = Color::white();
        self.rasterizer.opacity = 255;
        self.rasterizer.set_draw_mode(DrawMode::Opaque);
    }

    pub fn draw_performance_text(&mut self) {
        let total_pixels = self.rasterizer.drawn_pixels_since_clear;
        self.rasterizer.pprint(&self.font_score, format!("{:.1}ms  ({} UPS) PIXELS: {}\nCONTROLS: {}\n{}", (self.dt_unscaled * 100000.0).ceil() / 100.0, self.fps_print, total_pixels, self.controls, self.rng_number), 8, 8, 7, None);
    }

    pub fn draw_debug_collision(&mut self) {
        self.rasterizer.pcircle(false, self.player.position.x as i64, self.player.position.y as i64, self.player.radius as i64, Color::green());

        for asteroid in &self.asteroids {
            if asteroid.active {
                self.rasterizer.pcircle(false, asteroid.position.x as i64, asteroid.position.y as i64, asteroid.radius as i64, Color::green());
            }
        }

        for bullet in &self.bullets {
            if bullet.active {
                self.rasterizer.pcircle(false, bullet.position.x as i64, bullet.position.y as i64, bullet.radius as i64, Color::green());
            }
        }
    }

    pub fn update_times(&mut self) {
        let now = Instant::now();
        let now_s = (now.elapsed().as_secs() as f64) + (now.elapsed().subsec_nanos() as f64 * 1.0e-9);
        let before_s = (self.dt_before.elapsed().as_secs() as f64) + (self.dt_before.elapsed().subsec_nanos() as f64 * 1.0e-9);
        self.dt_unscaled = before_s - now_s;
        
        self.dt_before = Instant::now();
        if self.dt_unscaled < 0.0 {
            self.dt_unscaled = 0.0;
        }
        self.dt = self.dt_unscaled * self.timescale;
        self.realtime += self.dt_unscaled;
    }

    pub fn restart_game(&mut self) {
        self.player.active = true;
        self.player.position = Vector2::new(RENDER_WIDTH as f64 / 2.0, RENDER_HEIGHT as f64 / 2.0);
        self.player.velocity = Vector2::ZERO;
        self.player.rotation = 0.0;

        for asteroid in &mut self.asteroids {
            asteroid.active = false;
        }

        for bullet in &mut self.bullets {
            bullet.active = false;
        }

        for particle in &mut self.explosion_particles {
            particle.radius = 0.0;
        }

        self.score = 0;
    }

    ///// ====== ENGINE ====== /////
}

pub fn main() {
    
    let mut engine = AsteroidsEngine::new();
    let mut hardware_accelerated: bool = false;

    let args: Vec<_> = std::env::args().collect();
    for arg in args {
        match arg.as_str() {
            "--exclusive" => { engine.video_mode = VideoMode::Exclusive; },
            "--fullscreen" => { engine.video_mode = VideoMode::Fullscreen; },
            "--windowed" => { engine.video_mode = VideoMode::Windowed; },
            "--hardware-canvas" => { hardware_accelerated = true; }
            _ => {}
        }
    }
    

    let _error_code = engine.run(hardware_accelerated);
}