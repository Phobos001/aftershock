//! # Aftershock
//!
//! Aftershock is a Software-Rendered Graphics API focused on simplicity, being able to just jump in and start drawing stuff to the screen. Mainly inspired by the PICO-8
//! and loosely named after Quake.

#![crate_name = "aftershock"]
#![crate_type = "lib"]

// pub mod three;
pub mod rasterizer;
pub mod assets;
pub mod math;
pub mod drawables;
pub mod matrix3;
pub mod line;
pub mod vector2;
pub mod polygon;
pub mod color;
pub mod random;

//pub mod matrix4;
//pub mod quaternion;
//pub mod vector3;