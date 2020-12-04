use crate::all::*;

use std::rc::Rc;

pub struct Resolution {
	pub time: f32,
	pub point: Vec2,
	pub normal: Vec2,
	pub delta: Vec2,
	pub normal_push: f32
}

impl Resolution {
	pub fn new(point: Vec2, normal: Vec2, delta: Vec2, time: f32, normal_push: f32) -> Resolution {
		Resolution {
			point,
			normal,
			delta,
			time,
			normal_push,
		}
	}
}

pub struct Line {
	pub p1: Vec2,
	pub p2: Vec2,
}

// This shit is tight yo: https://noonat.github.io/intersect/
pub struct AABB {
	pub position: Vec2,
	pub extents: Vec2,
}

impl AABB {
	pub fn new(position: Vec2, extents: Vec2) -> AABB {
		AABB {
			position,
			extents,
		}
	}

	pub fn overlaps_point(aabb: &AABB, point: Vec2) -> bool {
		return point.x > aabb.position.x - aabb.extents.x && point.x < aabb.position.x + aabb.extents.x &&
				point.y > aabb.position.y - aabb.extents.y && point.y < aabb.position.y + aabb.extents.y;
	}

	pub fn resolution_point(aabb: &AABB, point: Vec2) -> Option<Resolution> {

		let (dx, dy) = (aabb.position.x - point.x, aabb.position.y - point.y);
		let (px, py) = (aabb.extents.x - dx.abs(), aabb.extents.y - dy.abs());

		if px > 0.0 && py > 0.0 {
			let mut resolution: Resolution = Resolution::new(Vec2::zero(), Vec2::zero(), Vec2::zero(), 0.0, 0.0);

			if px < py {
				let signx = -signf(dx);
				resolution.delta.x = px * (signx as f32);
				resolution.normal.x = signx;
				resolution.point.x = aabb.position.x + (aabb.extents.x * signx);
				resolution.point.y = point.y;
			} else {
				let signy = -signf(dy);
				resolution.delta.y = py * (signy as f32);
				resolution.normal.y = signy;
				resolution.point.y = aabb.position.y + (aabb.extents.y * signy);
				resolution.point.x = point.x;
			}

			Some(resolution)
		} else {
			None
		}
	}

	pub fn overlap_line(aabb: &AABB, line: Line, padding: Vec2) -> bool {
		let scale = Vec2::new(1.0 / line.p2.x, 1.0 / line.p2.y);
		let sign = Vec2::new(signf(scale.x), signf(scale.y));

		// 'Time' here is just a name for a lerp scale t; How far along the line basically.
		let near_time = (aabb.position - sign * (aabb.extents + padding) - line.p1) * scale;
		let far_time = (aabb.position + sign * (aabb.extents + padding) - line.p1) * scale;

		// No intersection can be happening since it's out of bounds of the AABB
		if near_time.x > far_time.y || near_time.y > far_time.x {
			return false;
		}

		let f_near_time = f32::max(near_time.x, near_time.y);
		let f_far_time = f32::min(far_time.x, far_time.y);

		// If t is outside the line then we can't use it; out of bounds for the line.
		if f_near_time >= 1.0 || f_far_time <= 0.0 {
			return false;
		}

		// Intersection is occuring!
		return true;
	}

	pub fn resolution_line(aabb: &AABB, line: Line, padding: Vec2) -> Option<Resolution> {
		let scale = Vec2::new(1.0 / line.p2.x, 1.0 / line.p2.y);
		let sign = Vec2::new(signf(scale.x), signf(scale.y));

		// 'Time' here is just a name for a lerp scale t; How far along the line basically.
		let near_time = (aabb.position - sign * (aabb.extents + padding) - line.p1) * scale;
		let far_time = (aabb.position + sign * (aabb.extents + padding) - line.p1) * scale;

		// No intersection can be happening since it's out of bounds of the AABB
		if near_time.x > far_time.y || near_time.y > far_time.x {
			return None;
		}

		let f_near_time = f32::max(near_time.x, near_time.y);
		let f_far_time = f32::min(far_time.x, far_time.y);

		// If t is outside the line then we can't use it; out of bounds for the line.
		if f_near_time >= 1.0 || f_far_time <= 0.0 {
			return None;
		}

		// Intersection is occuring. Lets make some useful data!

		let time: f32 = clampf(f_near_time, 0.0, 1.0);
		let normal: Vec2 = {
			if near_time.x > near_time.y {
				Vec2::new(-sign.x, 0.0)
			} else {
				Vec2::new(0.0, -sign.y)
			}
		};

		let delta: Vec2 = line.p2 * (1.0 / time);
		let point: Vec2 = Vec2::lerp(line.p1, line.p2, time);

		Some(Resolution {
			point,
			delta,
			normal,
			time,
			normal_push: 0.0,
		})
	}

	pub fn overlaps_aabb(aabb1: &AABB, aabb2: &AABB) -> bool {
		return aabb1.position.x - aabb1.extents.x < aabb2.position.x + aabb2.extents.x &&
			aabb2.position.x - aabb2.extents.x < aabb1.position.x + aabb1.extents.x &&
			aabb1.position.y - aabb1.extents.y < aabb2.position.y + aabb2.extents.y &&
			aabb2.position.y - aabb2.extents.y < aabb1.position.y + aabb1.extents.y;
	}

	pub fn resolution_aabb(aabb1: &AABB, aabb2: &AABB) -> Option<Resolution> {
		let (dx, dy) = (aabb2.position.x - aabb1.position.x, aabb2.position.y - aabb1.position.y);
		let (px, py) = ((aabb1.extents.x + aabb2.extents.x) - dx.abs(), (aabb1.extents.y + aabb2.extents.y) - dy.abs());

		if px > 0.0 && py > 0.0 {
			let mut resolution: Resolution = Resolution::new(Vec2::zero(), Vec2::zero(), Vec2::zero(), 0.0, 0.0);

			if px < py {
				let signx = signf(dx);
				resolution.delta.x = px * (signx as f32);
				resolution.normal.x = signx;
				resolution.normal_push = aabb2.extents.x;
				resolution.point.x = aabb1.position.x + (aabb1.extents.x * signx);
				resolution.point.y = aabb2.position.y;
			} else {
				let signy = signf(dy);
				resolution.delta.y = py * (signy as f32);
				resolution.normal.y = signy;
				resolution.normal_push = aabb2.extents.y;
				resolution.point.y = aabb1.position.y + (aabb1.extents.y * signy);
				resolution.point.x = aabb2.position.x;
			}

			Some(resolution)
		} else {
			None
		}
	}
}

pub struct Circle {
	pub position: Vec2,
	pub radius: f32,
}

impl Circle {

}

pub enum BlockmapColliderType {
	AABB,
	Line,
	Circle,
}

pub struct BlockmapColliderEntry {
	pub id: u32,
	pub collider_type: BlockmapColliderType,
}

pub struct BlockmapCell {
	pub aabbs: Vec<usize>,
	pub lines: Vec<usize>,
	pub circles: Vec<usize>,
}

impl BlockmapCell {
	pub fn new() -> BlockmapCell {

	}
}

/// Spatial partioning for broad-phase decection.
pub struct Blockmap {
	pub uid_count: u32,
	pub colliders: Vec<BlockmapColliderEntry>,
}

impl Blockmap {

}