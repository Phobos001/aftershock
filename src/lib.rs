//! # Aftershock
//!
//! Aftershock is a Software-Rendered Graphics API focused on simplicity, being able to just jump in and start drawing stuff to the screen. Mainly inspired by the PICO-8
//! and loosely named after Quake. No, there's no textured-triangle drawing functions yet, but it's planned.

#![crate_name = "aftershock"]
#![crate_type = "lib"]

pub mod rasterizer;
pub mod assets;
pub mod math;
pub mod drawables;
pub mod matricies;
pub mod vectors;
pub mod color;

pub mod all {
	pub use crate::rasterizer::*;
	pub use crate::assets::*;
	pub use crate::math::*;
	pub use crate::drawables::*;
	pub use crate::matricies::*;
	pub use crate::vectors::*;
	pub use crate::color::*;
}