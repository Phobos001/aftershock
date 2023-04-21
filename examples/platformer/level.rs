use aftershock::buffer::*;
use aftershock::color::*;

use dashmap::*;
use glam::*;

use crate::aabb::AABB;

pub struct TileSet {
    pub atlas: Buffer,
    pub is_destructable: bool,
}

pub struct Light {
    pub position: Vec2,
    pub color: Color,
    pub size: f32,
}

impl Light {
    pub fn new(position: Vec2, color: Color, size: f32) -> Light {
        Light {
            position, color, size
        }
    }
}


pub struct Level {
    pub lights: Vec<Light>,
    pub walls: DashMap<(i32, i32), u8>,
}

impl Level {
    pub fn new_prototype_1() -> Level {

        let mut walls: DashMap<(i32, i32), u8> = DashMap::new();

        for y in 0..128 {
            for x in 0..128 {
                if alea::f32() < (y as f32 / 128.0) {
                    walls.insert((x, y), alea::u32_less_than(5) as u8);
                }
            }
        }

        Level {
            lights: Vec::new(),
            walls
        }
    }
}