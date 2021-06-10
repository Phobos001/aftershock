/// High-Level Graphical User Interface Drawing Stuff(tm)

use crate::image::*;
use crate::drawables::*;
use crate::vector2::*;
use crate::rasterizer::*;

pub enum FitRule {
	None,
	Stretch,
}

pub struct GUI {
	pub anchored_sprites: Vec<AnchoredSprite>,
}

/// Ties a sprite to a part of the screen using two screen ratios. 
pub struct AnchoredSprite<'a> {
	pub sprite: Sprite,
	pub screen_position: Vector2,
	pub anchor: Vector2,
	pub center: Vector2,
	pub mode: FitRule,
}

impl AnchoredSprite {
	pub fn new(sprite: Sprite) -> AnchoredSprite {
		AnchoredSprite {
			sprite,
			anchor: Vector2::one() * 0.5,
			center: Vector2::one() * 0.5,
			mode: FitRule::None,
		}
	}

	pub fn draw(&mut rasterizer: Rasterizer) {
		sprite.position = screen_position * Vector2::new();
	}
}