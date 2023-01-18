//! # Aftershock
//!
//! Aftershock is a Software-Rendered Graphics API focused on simplicity, being able to just jump in and start drawing stuff to the screen. Mainly inspired by the PICO-8
//! and loosely named after Quake.

#![crate_name = "aftershock"]
#![crate_type = "lib"]

// Core
pub mod rasterizer;
pub mod partitioned_rasterizer;

// Assets
pub mod font;

// Utilities
pub mod math;
pub mod color;

// Math 2D
pub mod vector2;
pub mod matrix3;

// Profiling shorthand
pub fn timestamp() -> f64 {
    use std::time::SystemTime;
    SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs_f64()
}