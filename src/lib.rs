//! # Aftershock
//!
//! Aftershock is a Software-Rendered Graphics API focused on simplicity, being able to just jump in and start drawing stuff to the screen. Mainly inspired by the PICO-8
//! and loosely named after Quake. No, there's no textured-triangle drawing functions yet, but it's planned.

#![crate_name = "aftershock"]
#![crate_type = "lib"]

pub mod three;
pub mod rasterizer;
pub mod assets;
pub mod math;
pub mod drawables;
pub mod matrix3;
pub mod matrix4;
pub mod vector2;
pub mod vector3;
pub mod color;
pub mod random;
pub mod quaternion;