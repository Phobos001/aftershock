use std::ops::DerefMut;
use crate::buffer::*;

use dyn_clone::DynClone;
dyn_clone::clone_trait_object!(Shader);

use crate::color::*;

#[derive(Debug, Clone, Copy)]
pub struct ShaderParams {
    pub x: i32,
    pub y: i32,
    pub color: Color,
    pub p_f32: [f32; 8],
    pub p_i32: [i32; 8],
    pub p_color: [Color; 8]
}

impl ShaderParams {
    pub fn new(x: i32, y: i32, color: Color) -> ShaderParams {
        ShaderParams { x, y, color, p_f32: [0.0; 8], p_i32: [0; 8], p_color: [Color::CLEAR; 8] }
    }
}
pub trait Shader: DynClone + Send + Sync {
    fn shade(&mut self, buffer: &[u8], width: usize, height: usize, params: ShaderParams) -> Option<(i32, i32, Color)>;
    fn reset(&mut self);
}



#[derive(Debug, Clone)]
pub struct ShaderNoOp; impl Shader for ShaderNoOp {
    fn shade(&mut self, buffer: &[u8], width: usize, height: usize, params: ShaderParams) -> Option<(i32, i32, Color)> {
        Some((params.x, params.y, params.color))
    }

    fn reset(&mut self) {}
}

#[derive(Debug, Clone)]
pub struct ShaderOpaque; impl Shader for ShaderOpaque {
    fn shade(&mut self, buffer: &[u8], width: usize, height: usize, params: ShaderParams) -> Option<(i32, i32, Color)> {

        if params.color.a < 255 { return None; } else { return Some((params.x, params.y, params.color)) }
    }

    fn reset(&mut self) {}
}

#[derive(Debug, Clone)]
pub struct ShaderForceColor { pub color: Color } impl Shader for ShaderForceColor {
    fn shade(&mut self, buffer: &[u8], width: usize, height: usize, params: ShaderParams) -> Option<(i32, i32, Color)> {

        Some((params.x, params.y, self.color))
    }

    fn reset(&mut self) {self.color = Color::CLEAR; }
}

#[derive(Debug, Clone)]
pub struct ShaderMultiply; impl Shader for ShaderMultiply {
    fn shade(&mut self, buffer: &[u8], width: usize, height: usize, params: ShaderParams) -> Option<(i32, i32, Color)> {
        let idx: usize = ((params.y * (width as i32) + params.x) * 4) as usize;

        let bg = Color::new(
            buffer[idx],
            buffer[idx + 1],
            buffer[idx + 2],
            255
        );

        Some((params.x, params.y, bg * params.color))
    }

    fn reset(&mut self) {}
}

#[derive(Debug, Clone)]
pub struct ShaderTint {pub tint: Color} impl Shader for ShaderTint {
    fn shade(&mut self, buffer: &[u8], width: usize, height: usize, params: ShaderParams) -> Option<(i32, i32, Color)> {
        Some((params.x, params.y, self.tint * params.color))
    }

    fn reset(&mut self) {}
}

#[derive(Debug, Clone)]
pub struct ShaderAddition; impl Shader for ShaderAddition {
    fn shade(&mut self, buffer: &[u8], width: usize, height: usize, params: ShaderParams) -> Option<(i32, i32, Color)> {

        let idx: usize = ((params.y * (width as i32) + params.x) * 4) as usize;

        let bg = Color::new(
            buffer[idx],
            buffer[idx + 1],
            buffer[idx + 2],
            255
        );

        Some((params.x, params.y, bg + params.color))
    }

    fn reset(&mut self) {}
}

#[derive(Debug, Clone)]
pub struct ShaderAlpha { pub opacity: u8 } impl Shader for ShaderAlpha {
    fn shade(&mut self, buffer: &[u8], width: usize, height: usize, params: ShaderParams) -> Option<(i32, i32, Color)> {
        let idx: usize = ((params.y * (width as i32) + params.x) * 4) as usize;

        let bg = Color::new(
            buffer[idx],
            buffer[idx + 1],
            buffer[idx + 2],
            255
        );

        let c = Color::blend_fast(params.color, bg, self.opacity);

        Some((params.x, params.y, c))
    }

    fn reset(&mut self) {self.opacity = 255; }
}