use crate::{matrix4::*, color::*, vector2::*, vector3::*, rasterizer::*};

use std::sync::Arc;

#[derive(Copy, Clone, Debug)]
pub struct Triangle {
	pub v1: Vector3, pub v2: Vector3, pub v3: Vector3,
}

pub struct Mesh {
	pub triangles: Vec<Triangle>,
	pub uvs: Vec<Vector2>,
}

pub struct MeshRenderer {
	pub position: Vector3,
	pub rotation: Vector3,
	pub scale: Vector3,
	pub mesh: Arc<Mesh>,
}


// Wrapper for Matrix4 for handling projections
pub struct Projection {
	pub mtx: Matrix4,
	pub field_of_view: f32,
	pub clip_near: f32,
	pub clip_far: f32,
	pub screen_width: f32,
	pub screen_height: f32,
}

impl Projection {
	pub fn perspective(clip_near: f32, clip_far: f32, field_of_view: f32, screen_width: f32, screen_height: f32) -> Projection {
		let mut projection = Projection {
			mtx: Matrix4::identity(),
			field_of_view,
			clip_near,
			clip_far,
			screen_width,
			screen_height,
		};
		projection.update_mtx();
		projection
	}

	pub fn update_mtx(&mut self) {
		let fov_rad = 1.0 / (self.field_of_view * 0.5).to_radians().tan();

		// I inverted the aspect ratio as a joke while trying to figure out how to scale it correctly and it works and I cant fucking handle it.
		let aspect_ratio = 1.0 / (self.screen_width / self.screen_height);

		self.mtx.m[0][0] = aspect_ratio * fov_rad;
		self.mtx.m[1][1] = fov_rad;
		self.mtx.m[2][2] = self.clip_far / (self.clip_far - self.clip_near);
		self.mtx.m[3][2] = (-self.clip_far * self.clip_near) / (self.clip_far - self.clip_near);
		self.mtx.m[2][3] = 1.0;
		self.mtx.m[3][3] = 0.0;
	}

	pub fn project_triangle(&self, triangle: &Triangle) -> Triangle {

		// Multiply the triangle against the projection matrix
		let mut projected_triangle = Triangle {
			v1: self.mtx.forward(triangle.v1),
			v2: self.mtx.forward(triangle.v2),
			v3: self.mtx.forward(triangle.v3),
		};

		// Scale the triangle so it fits on screen
		projected_triangle.v1.x += 1.0; projected_triangle.v1.y += 1.0;
		projected_triangle.v2.x += 1.0; projected_triangle.v2.y += 1.0;
		projected_triangle.v3.x += 1.0; projected_triangle.v3.y += 1.0;
		projected_triangle.v1.x *= 0.5 * self.screen_width;
		projected_triangle.v1.y *= 0.5 * self.screen_height;
		projected_triangle.v2.x *= 0.5 * self.screen_width;
		projected_triangle.v2.y *= 0.5 * self.screen_height;
		projected_triangle.v3.x *= 0.5 * self.screen_width;
		projected_triangle.v3.y *= 0.5 * self.screen_height;


		projected_triangle
	}
}

impl Triangle {
	pub fn new(v1: Vector3, v2: Vector3, v3: Vector3) -> Triangle {
		Triangle {
			v1, v2, v3
		}
	}

	pub fn new_f32(v1x: f32, v1y: f32, v1z: f32, v2x: f32, v2y: f32, v2z: f32, v3x: f32, v3y: f32, v3z: f32) -> Triangle {
		Triangle {
			v1: Vector3::new(v1x, v1y, v1z),
			v2: Vector3::new(v2x, v2y, v2z),
			v3: Vector3::new(v3x, v3y, v3z),
		}
	}
}

impl Mesh {
	pub fn new_empty() -> Mesh {
		Mesh {
			triangles: Vec::new(),
			uvs: Vec::new(),
		}
	}

	pub fn new_cube() -> Mesh {
		Mesh {
			triangles: vec![
				// SOUTH
				Triangle::new_f32(0.0, 0.0, 0.0,	0.0, 1.0, 0.0,		1.0, 1.0, 0.0),
				Triangle::new_f32(0.0, 0.0, 0.0,	1.0, 1.0, 0.0,		1.0, 0.0, 0.0 ),

				// EAST                                                      
				Triangle::new_f32(1.0, 0.0, 0.0,	1.0, 1.0, 0.0,		1.0, 1.0, 1.0 ),
				Triangle::new_f32(1.0, 0.0, 0.0,	1.0, 1.0, 1.0,		1.0, 0.0, 1.0 ),

				// NORTH                                                     
				Triangle::new_f32(1.0, 0.0, 1.0,	1.0, 1.0, 1.0,		0.0, 1.0, 1.0 ),
				Triangle::new_f32(1.0, 0.0, 1.0,	0.0, 1.0, 1.0,		0.0, 0.0, 1.0 ),

				// WEST                                                      
				Triangle::new_f32(0.0, 0.0, 1.0,	0.0, 1.0, 1.0,		0.0, 1.0, 0.0 ),
				Triangle::new_f32(0.0, 0.0, 1.0,	0.0, 1.0, 0.0,		0.0, 0.0, 0.0 ),

				// TOP                                                       
				Triangle::new_f32(0.0, 1.0, 0.0,	0.0, 1.0, 1.0,		1.0, 1.0, 1.0 ),
				Triangle::new_f32(0.0, 1.0, 0.0,	1.0, 1.0, 1.0,		1.0, 1.0, 0.0 ),

				// BOTTOM                                                    
				Triangle::new_f32(1.0, 0.0, 1.0,	0.0, 0.0, 1.0,		0.0, 0.0, 0.0 ),
				Triangle::new_f32(1.0, 0.0, 1.0,	0.0, 0.0, 0.0,		1.0, 0.0, 0.0 ),

			],
			uvs: Vec::new(),
		}
	}
}

impl MeshRenderer {

	pub fn new(mesh: Arc<Mesh>) -> MeshRenderer {
		MeshRenderer {
			position: Vector3::zero(),
			rotation: Vector3::zero(),
			scale: Vector3::one(),
			mesh,
		}
	}

	pub fn draw_flat(&self, rasterizer: &mut Rasterizer, projection: &Projection, color: Color, wireframe: bool, cull_backfaces: bool) {
		for triangle in &self.mesh.triangles {
			let translation = Matrix4::translated(self.position);
			let rotation = Matrix4::rotated(self.rotation.z, self.rotation.y, self.rotation.x);
			let scaled = Matrix4::scaled(self.scale);

			let transform: Matrix4 = translation /* * rotation */ * scaled;

			let transformed_triangle = transform.transform_triangle(*triangle);
			let projected_triangle = projection.project_triangle(&transformed_triangle);

			// Actual Drawing
			let line1: Vector3 = projected_triangle.v2 - projected_triangle.v1;
			let line2: Vector3 = projected_triangle.v3 - projected_triangle.v1;
			let normal: Vector3 = Vector3::cross(line1, line2).normalized();
			
			let is_backface: bool = normal.x * transformed_triangle.v1.x +
									normal.y * transformed_triangle.v1.y +
									normal.z * transformed_triangle.v1.y >= 0.0;

			if is_backface && cull_backfaces { continue; }

			

			rasterizer.ptriangle(!wireframe, 
				projected_triangle.v1.x as i32, projected_triangle.v1.y as i32,
				projected_triangle.v2.x as i32, projected_triangle.v2.y as i32,
				projected_triangle.v3.x as i32, projected_triangle.v3.y as i32,
				color
			);
		}
	}

	pub fn draw_lit_directional(&self, rasterizer: &mut Rasterizer, projection: &Projection, color: Color, wireframe: bool, cull_backfaces: bool, lightdir: Vector3) {
		for triangle in &self.mesh.triangles {
			let translation = Matrix4::translated(self.position);
			let rotation = Matrix4::rotated(self.rotation.z, self.rotation.y, self.rotation.x);
			let scaled = Matrix4::scaled(self.scale);

			let transform: Matrix4 = translation  * rotation  * scaled;

			let transformed_triangle = transform.transform_triangle(*triangle);
			let projected_triangle = projection.project_triangle(&transformed_triangle);

			// Actual Drawing
			let line1: Vector3 = transformed_triangle.v2 - transformed_triangle.v1;
			let line2: Vector3 = transformed_triangle.v3 - transformed_triangle.v1;
			let normal: Vector3 = Vector3::cross(line1, line2).normalized();
			
			let is_backface: bool = normal.x * transformed_triangle.v1.x +
									normal.y * transformed_triangle.v1.y +
									normal.z * transformed_triangle.v1.y >= 0.0;

			if is_backface && cull_backfaces { continue; }

			let nlightdir = lightdir.normalized();

			let dot_normal_lightdir = Vector3::dot(normal, nlightdir);
			

			let lum: u8 = (((dot_normal_lightdir + 1.0) * 0.5) * 255.0) as u8;

			let lit_color = color * Color::new(lum, lum, lum, 255) ;

			rasterizer.ptriangle(!wireframe, 
				projected_triangle.v1.x as i32, projected_triangle.v1.y as i32,
				projected_triangle.v2.x as i32, projected_triangle.v2.y as i32,
				projected_triangle.v3.x as i32, projected_triangle.v3.y as i32,
				lit_color
			);
		}
	}
}