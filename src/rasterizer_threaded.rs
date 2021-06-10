use crate::rasterizer::*;
use crate::framebuffer::*;
use crate::image::*;
use crate::font::*;
use crate::color::*;
use crate::vector2::*;

// Queueable commands to store
struct InstCls 			{ }
struct InstClsColor 	{ color: Vec<Color> }
struct InstPSet 		{ x: Vec<i32>, y: Vec<i32>, color: Vec<Color> }
struct InstPLine 		{ x0: Vec<i32>, y0: Vec<i32>, x1: Vec<i32>, y1: Vec<i32>, color: Vec<Color> }
struct InstPLine2 		{ x0: Vec<i32>, y0: Vec<i32>, x1: Vec<i32>, y1: Vec<i32>, thickness: Vec<i32>, color: Vec<Color> }
struct InstPRectangle 	{ filled: Vec<bool>, x: Vec<i32>, y: Vec<i32>, w: Vec<i32>, h: Vec<i32>, color: Vec<Color> }
struct InstPCircle 		{ filled: Vec<bool>, xc: Vec<i32>, yc: Vec<i32>, r: Vec<i32>, color: Vec<Color> }
struct InstPImg 		{ image: Vec<&Image>, x: Vec<i32>, y: Vec<i32> }
struct InstPImgRect 	{ image: Vec<&Image>, x: Vec<i32>, y: Vec<i32>, rx: Vec<usize>, ry: Vec<usize>, rw: Vec<usize>, rh: Vec<usize> }
struct InstPImgMtx 		{ image: Vec<&Image>, position: Vec<Vector2>, rotation: Vec<f32>, scale: Vec<Vector2>, offset: Vec<Vector2>}

pub struct RasterizerThreaded {
	pub rasterizer: Rasterizer,
	pub images: HashMap<&str, Image>,
	pub fonts: HashMap<&str, Font>,
}

impl RasterizerThreaded {
	pub fn qset(x: i32, y: i32, color: Color) {
		
	}
}