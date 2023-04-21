extern crate device_query;

use aftershock::*;
use aftershock::buffer::*;
use aftershock::color::*;
use aftershock::font::*;
use aftershock::math::*;
use aftershock::glam::{Vec2, Affine2};

use aftershock::shader::ShaderAlpha;
use aftershock::shader::ShaderNoOp;
use aftershock::shader::ShaderOpaque;
use aftershock::shader::ShaderTint;
use device_query::*;

use std::time::Instant;

pub const CONTROL_ROTATE_LEFT: u8   = 0;
pub const CONTROL_ROTATE_RIGHT: u8  = 1;
pub const CONTROL_THRUST_FORWARD: u8     = 2;
pub const CONTROL_THRUST_BACKWARD: u8   = 3;
pub const CONTROL_FIRE: u8   = 4;
pub const CONTROL_PAUSE: u8  = 5;
pub const CONTROL_DEBUG_COLLISION: u8 = 6;
pub const CONTROL_DEBUG_INFO: u8 = 7;


#[derive(Debug, Clone, Copy)]
pub struct Player {
    pub active: bool,
    pub velocity: Vec2,
    pub position: Vec2,
    pub rotation: f32,
    pub radius: f32,
    pub scale: Vec2,
}

#[derive(Debug, Clone, Copy)]
pub struct Bullet {
    pub active: bool,
    pub velocity: Vec2,
    pub position: Vec2,
    pub rotation: f32,
    pub radius: f32,
    pub scale: Vec2,
    pub lifetime: f32,
}

impl Bullet {
    pub fn new() -> Bullet {
        Bullet {
            active: false,
            velocity: Vec2::ZERO,
            position: Vec2::ZERO,
            rotation: 0.0,
            radius: 0.0,
            scale: Vec2::ONE,
            lifetime: 0.0,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Asteroid {
    pub active: bool,
    pub shape: [Vec2; 8],
    pub velocity: Vec2,
    pub position: Vec2,
    pub rotation: f32,
    pub radius: f32,
    pub scale: Vec2,
    pub health: i8,
    pub split: u32,
}

impl Asteroid {
    pub fn new() -> Asteroid {
        Asteroid {
            active: false,
            shape: [Vec2::ZERO; 8],
            velocity: Vec2::ZERO,
            position: Vec2::ZERO,
            rotation: 0.0,
            radius: 0.0,
            scale: Vec2::ONE,
            health: 3,
            split: 0,
        }
    }

    pub fn generate_shape(radius: f32) -> [Vec2; 8] {
        let mut points: [Vec2; 8] = [
            Vec2::new(1.0, 0.0), // Right
            Vec2::new(0.5, 0.5), // Bottom Right
            Vec2::new(0.0, 1.0), // Bottom
            Vec2::new(-0.5, 0.5), // Bottom Left
            Vec2::new(-1.0, 0.0), // Left
            Vec2::new(-0.5, -0.5), // Top Left
            Vec2::new(0.0, -1.0), // Top
            Vec2::new(0.5, -0.5), // Top Right
        ];

        // Some noise to make them look more natrual
        // Does not effect collisions
        for p in points.iter_mut() {
            *p *= radius;
            *p += Vec2::new(alea::f32_in_range(-4.0, 4.0), alea::f32_in_range(-4.0, 4.0));
        }

        points
    }   
}

#[derive(Debug, Clone, Copy)]
pub struct ExplosionParticle {
    pub position: Vec2,
    pub velocity: Vec2,
    pub radius: f32,
}

impl ExplosionParticle {
    pub fn new() -> ExplosionParticle {
        ExplosionParticle {
            position: Vec2::ZERO,
            velocity: Vec2::ZERO,
            radius: 0.0,
        }
    }
}

/// If the squared distance between the two points is smaller than the combined diameter's squared, then the two circles are overlapping!
pub fn circle_overlap(p1: Vec2, r1: f32, p2: Vec2, r2: f32) -> bool {
    // Double the incoming radius's so they are diameter's instead.
    let (r1, r2) = (r1 * 2.0, r2 * 2.0);
    (p2.x - p1.x) * (p2.x - p1.x) + (p2.y - p1.y) * (p2.y - p1.y) < (r1*r2) + (r1*r2)
}

pub struct AsteroidsEngine {
    pub camera: Affine2,
    pub camera_boomzoom: f32,

    pub player: Player,
    pub asteroids: [Asteroid; 128],
    pub bullets: [Bullet; 128],

    pub explosion_particles: [ExplosionParticle; 8192],

    pub score: i32,

    pub uidx_asteroids: usize,
    pub uidx_bullets: usize,

    pub screen: Buffer,

    pub controls: u8,
    pub controls_last: u8,
    pub device_state: device_query::DeviceState,

    pub paused: bool,

    pub font_score: Font,

    pub pattern_test_image: Buffer,

    pub debug_collision: bool,
    pub debug_info: bool,

    pub realtime: f32,
    pub timescale: f32,

    pub profiling_update_time: f64,
    pub profiling_draw_time: f64,

    pub dt: f32,
    pub dt_unscaled: f32,
    
    pub present_time: f32,

    dt_before: Instant,
}

impl AsteroidsEngine {

    pub const RENDER_WIDTH: usize = 512;
    pub const RENDER_HEIGHT: usize = 512;

    pub fn new() -> AsteroidsEngine {
        println!("== OH BOY ITS ANOTHER ASTEROIDS EXAMPLE ==");

        // Font images will be read left-to-right, top-to-bottom. 
        // This will tell the Font what character goes to what part of the image.
        let tinyfont10_glyphidx = "ABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890!?/\\@#$%^&*()[]_-+=\"';:.";


        let center = Vec2::new(AsteroidsEngine::RENDER_WIDTH as f32 / 2.0, AsteroidsEngine::RENDER_HEIGHT as f32 / 2.0);

        let pattern_test_image: Buffer = Buffer::new_from_image("shared_assets/patterntest.png").unwrap();

        AsteroidsEngine {
            camera: Affine2::IDENTITY,
            camera_boomzoom: 1.0,

            player: Player { active: true, velocity: Vec2::new(0.0, 0.0), position: center, rotation: 0.0, radius: 4.0, scale: Vec2::ONE},
            asteroids: [Asteroid::new(); 128],
            bullets: [Bullet::new(); 128],

            explosion_particles: [ExplosionParticle::new(); 8192],

            score: 0,
            font_score: Font::new("shared_assets/tiny_font10.png", tinyfont10_glyphidx, 10, 10, 1).unwrap(),

            uidx_asteroids: 0,
            uidx_bullets: 0,

            screen: Buffer::new(AsteroidsEngine::RENDER_WIDTH, AsteroidsEngine::RENDER_HEIGHT),

            pattern_test_image,

            controls: 0,
            controls_last: 0,
            device_state: device_query::DeviceState::new(),

            debug_collision: false,
            debug_info: false,

            paused: false,
            
            dt: 0.0,
            dt_unscaled: 0.0,
            dt_before: Instant::now(),
            realtime: 0.0,
            timescale: 1.0,

            profiling_update_time: 0.0,
            profiling_draw_time: 0.0,

            present_time: 0.0,
		}
	}

    pub fn update(&mut self) {
        let update_time_before: f64 = timestamp();

        self.update_times();

        self.update_controls();
        

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

        let update_time_after: f64 = timestamp();

        self.profiling_update_time = update_time_after - update_time_before;

        // Give the processor a break
        std::thread::sleep(std::time::Duration::from_micros(1));
    }

    pub fn draw(&mut self) {
        let draw_time_before: f64 = timestamp();

        self.screen.clear();

        // No alpha-compositing or color-multiplication here, just draw directly to framebuffer.
        self.draw_explosion_particles();

        self.draw_score();
        self.draw_bullets();
        self.draw_asteroids();
        self.draw_player();

        if self.paused {
            self.screen.add_shader(BufferShader::new(Box::new(ShaderAlpha { opacity: if self.realtime.rem_euclid(0.5) > 0.25 { 255 } else { 0 }}), true, 0));

            self.screen.prectangle(true, 232, 232, 16, 32, Color::WHITE);
            self.screen.prectangle(true, 256, 232, 16, 32, Color::WHITE);

            self.screen.clear_shaders();
        }
        
        let draw_time_after: f64 = timestamp();
        self.profiling_draw_time = draw_time_after - draw_time_before;
        

        if self.debug_info {
            self.draw_performance_text();
        };

        if self.debug_collision {
            self.draw_debug_collision();
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

    pub fn update_controls(&mut self) {
        let keys: Vec<Keycode> = self.device_state.query_keymap();

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
                self.player.rotation -= self.dt * 3.0;
            }
    
            if self.is_control_down(CONTROL_ROTATE_RIGHT) {
                self.player.rotation += self.dt * 3.0;
            }
    
            if self.is_control_down(CONTROL_THRUST_FORWARD) {
                let direction = Affine2::from_angle(self.player.rotation);
    
                // We accelerate instead of speed up, so we multiply by dt here as well as when we update the position
                // Also we assume right is the starting rotation direction because of how we draw the player
                self.player.velocity += direction.transform_point2(Vec2::new(1.0, 0.0) * 64.0) * self.dt;
            }
    
            if self.is_control_down(CONTROL_THRUST_BACKWARD) {
                let direction = Affine2::from_angle(self.player.rotation);
    
                // We accelerate instead of speed up, so we multiply by dt here as well as when we update the position
                // Also we assume right is the starting rotation direction because of how we draw the player
                self.player.velocity -= direction.transform_point2(Vec2::new(1.0, 0.0) * 64.0) * self.dt;
            }
    
            if self.is_control_pressed(CONTROL_FIRE) {
                let direction = Affine2::from_angle(self.player.rotation).transform_point2(Vec2::new(1.0, 0.0));
                let offset = Affine2::from_translation(self.player.position).transform_point2(direction * 16.0);
                
                self.spawn_bullet(offset, direction.normalize(), self.player.velocity.length() + 128.0);
            }
    
            self.player.position += self.player.velocity * self.dt;
            self.player.position.x = self.player.position.x.rem_euclid(AsteroidsEngine::RENDER_WIDTH as f32);
            self.player.position.y = self.player.position.y.rem_euclid(AsteroidsEngine::RENDER_HEIGHT as f32);

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
        let translated: Affine2 = Affine2::from_translation(self.player.position);
        let rotated: Affine2 = Affine2::from_angle(self.player.rotation);
        let scaled: Affine2 = Affine2::from_scale(self.player.scale);

        // Transformations are done in the order of right to left
        let mtx = self.camera * translated * rotated * scaled;

        // Defines an arrow looking thing to represent the player
        let player_points = [
            Vec2::new(8.0, 0.0),
            Vec2::new(-8.0, 8.0),
            Vec2::new(-5.0, 0.0),
            Vec2::new(-8.0, -8.0),
        ];

        // Transform points into world space
        let mtx_line0 = mtx.transform_point2(player_points[0]);
        let mtx_line1 = mtx.transform_point2(player_points[1]);
        let mtx_line2 = mtx.transform_point2(player_points[2]);
        let mtx_line3 = mtx.transform_point2(player_points[3]);

        // Draw lines to Buffer with wrapping

        let player_color: Color = {
            if self.player.active {
                Color::WHITE
            } else {
                Color::RED
            }
        };

        //self.Buffer.wrapping = true;
        self.screen.pline(mtx_line0.x as i32, mtx_line0.y as i32, mtx_line1.x as i32, mtx_line1.y as i32, player_color);
        self.screen.pline(mtx_line1.x as i32, mtx_line1.y as i32, mtx_line2.x as i32, mtx_line2.y as i32, player_color);
        self.screen.pline(mtx_line2.x as i32, mtx_line2.y as i32, mtx_line3.x as i32, mtx_line3.y as i32, player_color);
        self.screen.pline(mtx_line3.x as i32, mtx_line3.y as i32, mtx_line0.x as i32, mtx_line0.y as i32, player_color);
        //self.Buffer.wrapping = false;
    }

    ///// ====== PLAYER ====== /////

    ///// ====== EFFECTS ====== /////

    pub fn spawn_explosion(&mut self, position: Vec2) {
        let mut spawn_counter = 32;
        for i in 0..self.explosion_particles.len() {
            if spawn_counter > 0 {
                if self.explosion_particles[i].radius <= 0.1 {
                    self.explosion_particles[i].radius = alea::f32_in_range(4.0, 16.0);
                    self.explosion_particles[i].position = position;
                    self.explosion_particles[i].velocity = Vec2::new(
                        alea::f32_in_range(-1.0, 1.0),
                        alea::f32_in_range(-1.0, 1.0)
                    ) * alea::f32_in_range(8.0, 256.0);
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
                self.explosion_particles[i].velocity = Vec2::lerp(self.explosion_particles[i].velocity, Vec2::new(0.0, 1.0), 1.0 * self.dt);
            }
            
        }
    }

    pub fn draw_explosion_particles(&mut self) {
        for i in 0..self.explosion_particles.len() {
            if self.explosion_particles[i].radius > 0.1 {
                let mtx_position = self.camera.transform_point2(self.explosion_particles[i].position);
                self.screen.pcircle(false, 
                    mtx_position.x as i32,
                    mtx_position.y as i32,
                    (self.explosion_particles[i].radius * self.camera_boomzoom) as i32, Color::hsv(0.1, 1.0, self.explosion_particles[i].radius / 8.0)
                );
            }
        }
    }

    ///// ====== EFFECTS ====== /////

    ///// ====== CAMERA ====== /////

    pub fn update_camera(&mut self) {
        self.camera_boomzoom = lerpf(self.camera_boomzoom, 1.0, 5.0 * self.dt);
        let camera_scaled = Affine2::from_scale(Vec2::ONE * self.camera_boomzoom);
        // We need to move the camera closer to the center based on zoom since it's technically in the top-left corner
        let camera_translated = Affine2::from_translation(
            Vec2::new(
                lerpf(AsteroidsEngine::RENDER_WIDTH as f32 / 2.0, 0.0, self.camera_boomzoom), 
                lerpf(AsteroidsEngine::RENDER_HEIGHT as f32 / 2.0, 0.0, self.camera_boomzoom)
            ));

        self.camera = camera_scaled * camera_translated;
    }
    
    pub fn camera_impact_effect(&mut self) {
        self.camera_boomzoom = 1.0333;
    }

    ///// ====== CAMERA ====== /////




    ///// ====== BULLET ====== /////

    pub fn spawn_bullet(&mut self, offset: Vec2, direction: Vec2, force: f32) {
        let mut bullet = &mut self.bullets[self.uidx_bullets % self.bullets.len()];

        bullet.position = offset;
        bullet.velocity = direction * force;
        bullet.radius = 2.0;
        bullet.scale = Vec2::ONE;
        bullet.rotation = 0.0; // Really if the bullets don't end up being dots this would be useful, otherwise not really.
        bullet.active = true;    // Kinda expensive to do it this way but it's simple

        self.uidx_bullets += 1;
        
        self.remove_score(5);
    }

    pub fn update_bullets(&mut self) {

        for i in 0..self.bullets.len() {
            if self.bullets[i].active {
                self.bullets[i].position += self.bullets[i].velocity * self.dt;
                self.bullets[i].position.x = self.bullets[i].position.x.rem_euclid(AsteroidsEngine::RENDER_WIDTH as f32);
                self.bullets[i].position.y = self.bullets[i].position.y.rem_euclid(AsteroidsEngine::RENDER_HEIGHT as f32);
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
                let mtx_position = self.camera.transform_point2(bullet.position);
                self.screen.pcircle(true, mtx_position.x as i32, mtx_position.y as i32, bullet.radius as i32, Color::WHITE);
            }   
        }
    }

    ///// ====== BULLET ====== /////




    ///// ====== ASTEROID ====== /////

    pub fn spawn_asteroid(&mut self) {
        let mut asteroid = &mut self.asteroids[self.uidx_asteroids % self.asteroids.len()];

        // Make size smaller if this asteroid is from a destroyed asteroid.
        asteroid.radius = alea::f32_in_range(8.0, 32.0);
        asteroid.shape = Asteroid::generate_shape(asteroid.radius);
        asteroid.position = Vec2::new(alea::f32_in_range(0.0, AsteroidsEngine::RENDER_WIDTH as f32), alea::f32_in_range(0.0, AsteroidsEngine::RENDER_HEIGHT as f32));
        asteroid.rotation = alea::f32_in_range(0.0, 6.28);
        asteroid.velocity = Vec2::new(alea::f32_in_range(-1.0, 1.0), alea::f32_in_range(-1.0, 1.0)) * alea::f32_in_range(2.0, 64.0);
        asteroid.active = true;
        asteroid.split = 1;

        self.uidx_asteroids += 1;
    }

    pub fn spawn_asteroid_split(&mut self, original_asteroid_idx: usize, split_count: u32) {
        let original_radius = self.asteroids[original_asteroid_idx].radius;
        let original_position = self.asteroids[original_asteroid_idx].position;

        let mut asteroid = &mut self.asteroids[self.uidx_asteroids % self.asteroids.len()];
        

        // Make size smaller if this asteroid is from a destroyed asteroid.
        asteroid.radius = original_radius - (alea::f32_in_range(1.0, 4.0) * split_count as f32);

        if asteroid.radius < 2.0 { return; }

        asteroid.shape = Asteroid::generate_shape(asteroid.radius);
        asteroid.position = original_position;
        asteroid.rotation = alea::f32_in_range(0.0, 6.28);
        asteroid.velocity = Vec2::new(alea::f32_in_range(-1.0, 1.0), alea::f32_in_range(-1.0, 1.0)) * alea::f32_in_range(2.0, 64.0);
        asteroid.active = true;
        asteroid.split += 1;

        self.uidx_asteroids += 1;
    }

    pub fn update_asteroids(&mut self) {

        // Update active asteroids. Also keep track of how many are active.
        let mut asteroids_active: u32 = 0;
        for i in 0..self.asteroids.len() {
            if self.asteroids[i].active { 
                asteroids_active += 1;

                self.asteroids[i].position += self.asteroids[i].velocity * self.dt;
                self.asteroids[i].position.x = self.asteroids[i].position.x.rem_euclid(AsteroidsEngine::RENDER_WIDTH as f32);
                self.asteroids[i].position.y = self.asteroids[i].position.y.rem_euclid(AsteroidsEngine::RENDER_HEIGHT as f32);

                self.asteroids[i].rotation += (self.asteroids[i].velocity.length() / 100.0) * self.dt;

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

                            if self.asteroids[i].split < 3 {
                                let split_count: u32 = alea::u32_in_range(2, 4);
                                for _k in 0..split_count {
                                    self.spawn_asteroid_split(i, split_count);
                                }
                            }
                            
                            

                            break;
                        }
                    }
                }
            }        
        }

        if asteroids_active <= 0 {
            for _ in 0..alea::i32_in_range(8, 16) {
                self.spawn_asteroid();
            }
        }
    }

    pub fn draw_asteroids(&mut self) {
        for asteroid in &self.asteroids {
            if asteroid.active {
                // Prepare a transformation chain to get our final transformation matrix
                let translated: Affine2 = Affine2::from_translation(asteroid.position);
                let rotated: Affine2 = Affine2::from_angle(asteroid.rotation);
                let scaled: Affine2 = Affine2::from_scale(asteroid.scale);

                // Transformations are done in the order of right to left
                let mtx = self.camera * translated * rotated * scaled;

                // Transform points into world space
                let mtx_line0 = mtx.transform_point2(asteroid.shape[0]);
                let mtx_line1 = mtx.transform_point2(asteroid.shape[1]);
                let mtx_line2 = mtx.transform_point2(asteroid.shape[2]);
                let mtx_line3 = mtx.transform_point2(asteroid.shape[3]);
                let mtx_line4 = mtx.transform_point2(asteroid.shape[4]);
                let mtx_line5 = mtx.transform_point2(asteroid.shape[5]);
                let mtx_line6 = mtx.transform_point2(asteroid.shape[6]);
                let mtx_line7 = mtx.transform_point2(asteroid.shape[7]);

                // Draw lines to Buffer with wrapping
                //self.Buffer.wrapping = true;
                self.screen.pline(mtx_line0.x as i32, mtx_line0.y as i32, mtx_line1.x as i32, mtx_line1.y as i32, Color::WHITE);
                self.screen.pline(mtx_line1.x as i32, mtx_line1.y as i32, mtx_line2.x as i32, mtx_line2.y as i32, Color::WHITE);
                self.screen.pline(mtx_line2.x as i32, mtx_line2.y as i32, mtx_line3.x as i32, mtx_line3.y as i32, Color::WHITE);
                self.screen.pline(mtx_line3.x as i32, mtx_line3.y as i32, mtx_line4.x as i32, mtx_line4.y as i32, Color::WHITE);
                self.screen.pline(mtx_line4.x as i32, mtx_line4.y as i32, mtx_line5.x as i32, mtx_line5.y as i32, Color::WHITE);
                self.screen.pline(mtx_line5.x as i32, mtx_line5.y as i32, mtx_line6.x as i32, mtx_line6.y as i32, Color::WHITE);
                self.screen.pline(mtx_line6.x as i32, mtx_line6.y as i32, mtx_line7.x as i32, mtx_line7.y as i32, Color::WHITE);
                self.screen.pline(mtx_line7.x as i32, mtx_line7.y as i32, mtx_line0.x as i32, mtx_line0.y as i32, Color::WHITE);
                //self.Buffer.wrapping = false;
            }
            
        }
    }

    ///// ====== ASTEROID ====== /////

    


    ///// ====== ENGINE ====== /////

    pub fn add_score(&mut self, score: i32) {
        self.score += score;
    }

    pub fn remove_score(&mut self, score: i32) {
        self.score -= score;
        if self.score < 0 {
            self.score = 0;
        }
    }

    pub fn draw_score(&mut self) {

        let score_color: Color = Color::hsv(self.realtime * 10.0, 1.0, 1.0);
        self.screen.add_shader(BufferShader::new(Box::new(ShaderOpaque), true, 0));
        self.screen.add_shader(BufferShader::new(Box::new(ShaderTint {tint: score_color}), true, 1));

        

        self.screen.pprint(&self.font_score, format!("{:0>8}", self.score), self.player.position.x as i32 - 44, self.player.position.y as i32 + 24, 0, None);

        self.screen.clear_shaders();
    }

    pub fn draw_performance_text(&mut self) {
        self.screen.pprint(&self.font_score, format!("UPDATE TIME: {}MS\nDRAW TIME: {}MS\nCONTROLS: {}\n", 
        (self.profiling_update_time * 100000.0).ceil() / 100.0, 
        (self.profiling_draw_time * 100000.0).ceil() / 100.0, self.controls), 8, 8, 7, None);
    }

    pub fn draw_debug_collision(&mut self) {
        self.screen.pcircle(false, self.player.position.x as i32, self.player.position.y as i32, self.player.radius as i32, Color::GREEN);

        for asteroid in &self.asteroids {
            if asteroid.active {
                self.screen.pcircle(false, asteroid.position.x as i32, asteroid.position.y as i32, asteroid.radius as i32, Color::GREEN);
            }
        }

        for bullet in &self.bullets {
            if bullet.active {
                self.screen.pcircle(false, bullet.position.x as i32, bullet.position.y as i32, bullet.radius as i32, Color::GREEN);
            }
        }
    }

    pub fn update_times(&mut self) {
        let now = Instant::now();

        let now_s = (now.elapsed().as_secs() as f32) + (now.elapsed().subsec_nanos() as f32 * 1.0e-9);
        let before_s = (self.dt_before.elapsed().as_secs() as f32) + (self.dt_before.elapsed().subsec_nanos() as f32 * 1.0e-9);

        self.dt_unscaled = before_s - now_s;
        
        if self.dt_unscaled < 0.0 {
            self.dt_unscaled = 0.0;
        }

        self.dt = self.dt_unscaled * self.timescale;
        self.realtime += self.dt_unscaled;
        self.present_time -= self.dt_unscaled;

        self.dt_before = Instant::now();
    }

    pub fn restart_game(&mut self) {
        self.player.active = true;
        self.player.position = Vec2::new(AsteroidsEngine::RENDER_WIDTH as f32 / 2.0, AsteroidsEngine::RENDER_HEIGHT as f32 / 2.0);
        self.player.velocity = Vec2::ZERO;
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

}