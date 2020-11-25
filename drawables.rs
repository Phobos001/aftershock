use crate::all::*;

pub struct Sprite<'a> {
	pub mtx: Mat3,
	pub offset: Vec2,
	pub image: &'a Image,
	pub position: Vec2,
	pub rotation: f32,
	pub scale: Vec2,
	pub depth: f64,
}

impl<'a> Sprite<'a> {
	pub fn new(image: &'a Image, x: f32, y: f32, a: f32, sx: f32, sy: f32, depth: f64) -> Sprite {
		Sprite {
			image,
			offset: Vec2::new(-(image.width as f32) / 2.0, -(image.height as f32) / 2.0),
			position: Vec2::new(x, y),
			rotation: a,
			scale: Vec2::new(sx, sy),
			mtx: Mat3::identity(),
			depth,
		}
	}

	pub fn draw(&self, rasterizer: &mut Rasterizer) {
		let mtx_o = Mat3::translated(self.offset);
        let mtx_r = Mat3::rotated(self.rotation);
        let mtx_p = Mat3::translated(self.position);
        let mtx_s = Mat3::scaled(self.scale);

        let cmtx = rasterizer.mtx * (mtx_p * mtx_r * mtx_s * mtx_o);

        // We have to get the rotated bounding box of the rotated sprite in order to draw it correctly without blank pixels
        let start_center: Vec2 = cmtx.forward(Vec2::zero());
        let (mut sx, mut sy, mut ex, mut ey) = (start_center.x, start_center.y, start_center.x, start_center.y);

        // Top-Left Corner
        let p1: Vec2 = cmtx.forward(Vec2::zero());
        sx = f32::min(sx, p1.x); sy = f32::min(sy, p1.y);
        ex = f32::max(ex, p1.x); ey = f32::max(ey, p1.y);

        // Bottom-Right Corner
        let p2: Vec2 = cmtx.forward(Vec2::new(self.image.width as f32, self.image.height as f32));
        sx = f32::min(sx, p2.x); sy = f32::min(sy, p2.y);
        ex = f32::max(ex, p2.x); ey = f32::max(ey, p2.y);

        // Bottom-Left Corner
        let p3: Vec2 = cmtx.forward(Vec2::new(0.0, self.image.height as f32));
        sx = f32::min(sx, p3.x); sy = f32::min(sy, p3.y);
        ex = f32::max(ex, p3.x); ey = f32::max(ey, p3.y);

        // Top-Right Corner
        let p4: Vec2 = cmtx.forward(Vec2::new(self.image.width as f32, 0.0));
        sx = f32::min(sx, p4.x); sy = f32::min(sy, p4.y);
        ex = f32::max(ex, p4.x); ey = f32::max(ey, p4.y);

        let mut rsx = sx as i32;
        let mut rsy = sy as i32;
        let mut rex = ex as i32;
        let mut rey = ey as i32;

        // Sprite isn't even in frame, don't draw anything
        if (rex < 0 || rsx > rasterizer.framebuffer.width as i32) && (rey < 0 || rsy > rasterizer.framebuffer.height as i32) { return; }

        // Okay but clamp the ranges in frame so we're not wasting time on stuff offscreen
        if rsx < 0 { rsx = 0;}
        if rsy < 0 { rsy = 0;}
        if rex > rasterizer.framebuffer.width as i32 { rex = rasterizer.framebuffer.width as i32; }
        if rey > rasterizer.framebuffer.height as i32 { rey = rasterizer.framebuffer.height as i32; }

        let cmtx_inv = cmtx.clone().inv();

		// We can finally draw!
		// Noticed some weird clipping on the right side of sprites, like the BB isn't big enough? Just gonna add some more pixels down and right just in case
        for ly in rsy..rey+8 {
            for lx in rsx..rex+8 {
                // We have to use the inverted compound matrix (cmtx_inv) in order to get the correct pixel data from the image.
                let ip: Vec2 = cmtx_inv.forward(Vec2::new(lx as f32, ly as f32));
                let color: Color = self.image.pget(ip.x as i32, ip.y as i32);
                rasterizer.pset(lx as i32, ly as i32, color);
            }
        }
	}
}

pub struct Line {
	p1: Vec2,
	p2: Vec2,
	color: Color,
}

impl Line {
	pub fn new() -> Line {
		Line {
			p1: Vec2::zero(),
			p2: Vec2::one(),
			color: Color::new(255, 255, 255, 255),
		}
	}

	pub fn draw(&self, rasterizer: &mut Rasterizer) {
		let p1 = rasterizer.mtx.forward(self.p1);
		let p2 = rasterizer.mtx.forward(self.p2);
		rasterizer.pline(p1.x as i32, p1.y as i32, p2.x as i32, p2.y as i32, self.color);
	}
}