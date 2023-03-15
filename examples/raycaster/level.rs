use aftershock::math::lerpi;
use aftershock::math::unlerpf;
use aftershock::vector2::*;
use aftershock::buffer::*;
use aftershock::color::*;
use dashmap::DashMap;

use std::sync::Arc;

use crate::engine::*;

#[derive(Clone, Copy)]
pub struct Line {
    pub start: Vector2,
    pub end: Vector2,
}

pub struct Sector {
    pub lines: Vec<usize>,
    pub height_floor: f32,
    pub height_ceiling: f32,
}

pub struct Level {
    pub sectors: Vec<Sector>,
    pub lines: Vec<Line>,
    pub camera_position: Vector2,
    pub camera_rotation: f32,
    pub textures: Arc<DashMap<String, Buffer>>,
}

impl Line {
    pub fn new(start: Vector2, end: Vector2) -> Line {
        Line { start, end }
    }
}

impl Level {
    pub fn new() -> Level {

        let textures: Arc<DashMap<String, Buffer>> = Arc::new(DashMap::new());
        textures.insert("pattern_test".to_string(), Buffer::new_from_image("examples/raycaster/textures/prototype/prototype_grid.png").unwrap());

        Level {
            sectors: vec![
                Sector {
                    lines: vec![
                        0, 1, 2, 3
                    ],
                    height_floor: 0.0,
                    height_ceiling: 5.0,
                }
            ],
            lines: vec![
                Line {
                    start: Vector2::new(-10.0, -10.0),
                    end: Vector2::new(10.0, -10.0),
                },
                Line {
                    start: Vector2::new(10.0, -10.0),
                    end: Vector2::new(10.0, 10.0),
                },
                Line {
                    start: Vector2::new(10.0, 10.0),
                    end: Vector2::new(-10.0, 10.0),
                },
                Line {
                    start: Vector2::new(-10.0, 10.0),
                    end: Vector2::new(-10.0, -10.0),
                },
            ],
            camera_position: Vector2::ZERO,
            camera_rotation: 0.0,
            textures,
        }
    }
}