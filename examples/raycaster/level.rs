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
    pub height_bottom: f32,
    pub height_top: f32,
    pub flipped: bool,
}

pub struct Sector {
    pub lines: Vec<Line>,
}

pub struct Level {
    pub sectors: Vec<Sector>,
    pub camera_position: Vector2,
    pub camera_rotation: f32,
    pub textures: Arc<DashMap<String, Buffer>>,
}

impl Level {
    pub fn new() -> Level {

        let textures: Arc<DashMap<String, Buffer>> = Arc::new(DashMap::new());
        textures.insert("pattern_test".to_string(), Buffer::new_from_image("shared_assets/patterntest.png").unwrap());

        Level {
            sectors: vec![
                Sector {
                    lines: vec![
                        Line {
                            start: Vector2::new(-10.0, -10.0),
                            end: Vector2::new(10.0, -10.0),
                            height_bottom: 0.0,
                            height_top: 5.0,
                            flipped: false,
                        },
                        Line {
                            start: Vector2::new(10.0, -10.0),
                            end: Vector2::new(10.0, 10.0),
                            height_bottom: 0.0,
                            height_top: 5.0,
                            flipped: false,
                        },
                        Line {
                            start: Vector2::new(10.0, 10.0),
                            end: Vector2::new(-10.0, 10.0),
                            height_bottom: 0.0,
                            height_top: 5.0,
                            flipped: false,
                        },
                        Line {
                            start: Vector2::new(-10.0, 10.0),
                            end: Vector2::new(-10.0, -10.0),
                            height_bottom: 0.0,
                            height_top: 5.0,
                            flipped: false,
                        },
                        
                    ]
                }
            ],
            camera_position: Vector2::ZERO,
            camera_rotation: 0.0,
            textures,
        }
    }
}