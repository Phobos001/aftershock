use aftershock::buffer::Buffer;
use aftershock::vector2::*;
use aftershock::math::*;

use dashmap::*;

use crate::aabb::*;
use crate::controls::*;
use crate::engine::PlatformerEngine;

use aftershock::color::*;

#[repr(u8)]
pub enum Wall {
    PrototypeRed        = 0,
    PrototypeGreen      = 1,
    PrototypeBlue       = 2,
    PrototypeYellow     = 3,
    PrototypeCyan       = 4,
    PrototypeMagenta    = 5,
    PrototypeGrey       = 6,
    Count               = 7,
}

pub struct GameState {
    pub camera_position: Vector2,

    pub player: AABB,
    pub is_grabbing_ceiling: bool,
    pub is_grounded: bool,

    pub walls: DashMap<(i32, i32), u8>,
}

impl GameState {
    pub const TILE_SIZE: i32 = 8;

    pub fn new() -> GameState {
        let mut gs = GameState {
            camera_position: Vector2::ZERO,
            player: AABB::new(Vector2::new(480.0, -256.0), Vector2::new(14.0, 14.0)),
            is_grabbing_ceiling: false,
            is_grounded: false,
            walls: DashMap::new(),
        };

        gs.init();
        gs
    }

    pub fn camera_offset(&self, input: Vector2) -> Vector2 {
        Vector2::new(
            input.x - (self.camera_position.x - (PlatformerEngine::RENDER_WIDTH as f32 / 2.0)),
            input.y - (self.camera_position.y - (PlatformerEngine::RENDER_HEIGHT as f32 / 2.0)),
        )
    }

    pub fn init(&mut self) {
        for y in 0..128 {
            for x in 0..128 {
                if alea::f32() < (y as f32 / 128.0) {
                    self.walls.insert((x, y), alea::u32_less_than(Wall::Count as u32) as u8);
                }
            }
        }
    }

    pub fn update(&mut self, controls: &Controls, dt: f32) {
        let dt = f32::min(dt, 0.01667);
        let mut target_velocity: Vector2 = Vector2::ZERO;
        target_velocity.x -= if controls.is_control_down(ControlKeys::MoveLeft)  { 1.0 } else { 0.0 };
        target_velocity.x += if controls.is_control_down(ControlKeys::MoveRight) { 1.0 } else { 0.0 };
        
        

        if controls.is_control_down(ControlKeys::MoveLeft) || controls.is_control_down(ControlKeys::MoveRight) {
            self.player.velocity.x = lerpf(self.player.velocity.x, target_velocity.x * 256.0, 1.0 * dt);
        } else {
            self.player.velocity.x = lerpf(self.player.velocity.x, 0.0, 10.0 * dt);
        }

        if controls.is_control_down(ControlKeys::Jump) && self.is_grounded {
            self.player.velocity.y = -128.0;
            self.is_grounded = false;
        }

        self.player.velocity.y += (if self.is_grabbing_ceiling { -512.0 } else { 512.0 }) * dt;


        self.player.velocity.y = f32::clamp(self.player.velocity.y, -512.0, 512.0);
        self.player.position += self.player.velocity * dt;

        self.camera_position = Vector2::lerp(self.camera_position, self.player.position, 10.0 * dt);

        let grid_idx: (i32, i32) = ((self.player.position.x / (GameState::TILE_SIZE as f32)).ceil() as i32, (self.player.position.y.ceil() / (GameState::TILE_SIZE as f32)) as i32);
        

        // Get list of nearby walls and sort them from distance to the player                              
        let mut testable_aabbs: Vec<AABB> = Vec::new();


        for iy in (grid_idx.1 - 3)..(grid_idx.1 + 3) {
            for ix in (grid_idx.0 - 3)..(grid_idx.0 + 3) {
                let wall_aabb_check = self.walls.get(&(ix, iy));


                
                if wall_aabb_check.is_some() {

                    //let wall_aabb_result = wall_aabb_check.unwrap();
                    testable_aabbs.push(AABB::new(Vector2::new(ix as f32 * (GameState::TILE_SIZE as f32), iy as f32 * (GameState::TILE_SIZE as f32)), Vector2::new((GameState::TILE_SIZE as f32), (GameState::TILE_SIZE as f32))));

                    
                }
            }
        }

        // Sort closest to player
        testable_aabbs.sort_by(
            |a, b| 
            Vector2::distance(self.player.position, a.position).partial_cmp( 
            &Vector2::distance(self.player.position, b.position)).unwrap())
        ;

        for aabb in &testable_aabbs {
            let position_normal = AABB::resolve_aabb(&mut self.player, aabb, dt);

            if position_normal.is_some() {
                let new_position: Vector2 = position_normal.unwrap().0;
                let normal: Vector2 = position_normal.unwrap().1;
    
                // Pushes 1 pixel away from AABB surface
                self.player.position = new_position + (normal);
                self.player.velocity = Vector2::slide(self.player.velocity, normal);
    
                
                self.is_grounded = true;
                
            }
    
            // Ceiling Check
            let ceiling_check_opt = AABB::raycast_aabb(
                &aabb, 
                self.player.position, 
                self.player.position + (Vector2::UP * self.player.extents.y)
            );
    
            self.is_grabbing_ceiling = ceiling_check_opt.is_some() && controls.is_control_down(ControlKeys::Jump);
        }
        
    }

    pub fn draw(&self, screen: &mut Buffer) {
        screen.clear();

        let player_screen_position: Vector2 = self.camera_offset(self.player.position - (self.player.extents / 2.0)).rounded();

        screen.prectangle(true,
            player_screen_position.x as i32, 
            player_screen_position.y as i32,
            self.player.extents.x as i32, self.player.extents.y as i32,
            Color::GREEN
        );

        


        let grid_idx: (i32, i32) = ((self.camera_position.x / 8.0).ceil() as i32, (self.camera_position.y.ceil() / 8.0) as i32);
        

        // Nearby world render
        for iy in (grid_idx.1 - 26)..(grid_idx.1 + 26) {
            for ix in (grid_idx.0 - 42)..(grid_idx.0 + 42) {
                let wall_aabb_check = self.walls.get(&(ix, iy));
                if wall_aabb_check.is_some() {
                    let result = wall_aabb_check.unwrap();
                    let wall = result.value();

                    let wall_position = Vector2::new(ix as f32 * 8.0, iy as f32 * 8.0);
                
                    let wall_screen_position = self.camera_offset(wall_position - (Vector2::ONE * ((GameState::TILE_SIZE as f32) / 2.0)));
                    screen.prectangle(true,
                        wall_screen_position.x as i32,
                        wall_screen_position.y as i32,
                        GameState::TILE_SIZE, GameState::TILE_SIZE,
                        Color::new(128, 128, 128, 255)
                    );
                }
            }
        }

        let grid_idx: (i32, i32) = ((self.player.position.x / (GameState::TILE_SIZE as f32)).ceil() as i32, (self.player.position.y.ceil() / (GameState::TILE_SIZE as f32)) as i32);

        // Static Collision Check Vis
        for iy in (grid_idx.1 - 3)..(grid_idx.1 + 3) {
            for ix in (grid_idx.0 - 3)..(grid_idx.0 + 3) {
                let wall_aabb_check = self.walls.get(&(ix, iy));
                if wall_aabb_check.is_some() {
                    let result = wall_aabb_check.unwrap();
                    let wall = result.value();

                    let wall_position = Vector2::new(ix as f32 * (GameState::TILE_SIZE as f32), iy as f32 * (GameState::TILE_SIZE as f32));

                    let wall_screen_position = self.camera_offset(wall_position - (Vector2::ONE * ((GameState::TILE_SIZE as f32) / 2.0)));
                    screen.prectangle(false,
                        wall_screen_position.x as i32, 
                        wall_screen_position.y as i32,
                        GameState::TILE_SIZE, GameState::TILE_SIZE,
                        Color::YELLOW
                    );
                }
            }
        }
    }
}