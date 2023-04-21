//! # Aftershock
//!
//! Aftershock is a Software-Rendered Graphics API focused on simplicity, being able to just jump in and start drawing stuff to the screen. Mainly inspired by the PICO-8
//! and loosely named after Quake.

#![crate_name = "aftershock"]
#![crate_type = "lib"]

pub extern crate glam;

// Core
pub mod buffer;
pub mod partitioned_buffer;
pub mod shader;

// Assets
pub mod font;

// Utilities
pub mod math;
pub mod color;

// Math 3D;
pub mod three_dee;



// Profiling shorthand
pub fn timestamp() -> f64 {
    std::time::SystemTime::now().duration_since(std::time::SystemTime::UNIX_EPOCH).unwrap().as_secs_f64()
}