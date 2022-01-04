//! # Aftershock
//!
//! Aftershock is a Software-Rendered Graphics API focused on simplicity, being able to just jump in and start drawing stuff to the screen. Mainly inspired by the PICO-8
//! and loosely named after Quake.

#![crate_name = "aftershock"]
#![crate_type = "lib"]

// Core
pub mod framebuffer;
pub mod rasterizer;

// Assets

pub mod image;
pub mod font;

// Utilities
pub mod math;
pub mod drawables;
pub mod color;
pub mod random;

// Math 2D
pub mod vector2;
pub mod matrix3;

// Math 3D
pub mod matrix4;
pub mod vector3;

// 3D Utilities
pub mod three;