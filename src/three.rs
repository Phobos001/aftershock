use crate::{matrix4::*, color::*, vector3::*, rasterizer::*};

#[derive(Copy, Clone, Debug)]
pub struct Triangle {
	pub v1: Vector3, pub v2: Vector3, pub v3: Vector3,
}

pub struct Mesh {
	pub triangles: Vec<Triangle>,
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

	pub fn project_triangle(&self, itriangle: &Triangle) -> Triangle {

		let mut triangle = itriangle.clone();
		triangle.v1.z += 3.0;
		triangle.v2.z += 3.0;
		triangle.v3.z += 3.0;

		triangle.v1 += Vector3::one();
		triangle.v2 += Vector3::one();
		triangle.v3 += Vector3::one();

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
	pub fn draw_flat(&self, rasterizer: &mut Rasterizer, projection: &Projection, color: Color, wireframe: bool, cull_backfaces: bool) {
		let mut projected_triangles: Vec<Triangle> = Vec::with_capacity(self.triangles.len());

		for triangle in &self.triangles {
			projected_triangles.push(projection.project_triangle(triangle));
		}

		for triangle in &projected_triangles {
			let line1: Vector3 = triangle.v2 - triangle.v1;
			let line2: Vector3 = triangle.v3 - triangle.v1;
			let normal: Vector3 = -Vector3::cross(line1, line2).normalized();
				
			if normal.z < 0.0 && cull_backfaces { continue; }

			rasterizer.ptriangle(!wireframe, 
				triangle.v1.x as i32, triangle.v1.y as i32,
				triangle.v2.x as i32, triangle.v2.y as i32,
				triangle.v3.x as i32, triangle.v3.y as i32,
				color
			);
		}
	}

	pub fn draw_normals(&self, rasterizer: &mut Rasterizer, projection: &Projection, wireframe: bool, cull_backfaces: bool) {
		let mut projected_triangles: Vec<Triangle> = Vec::with_capacity(self.triangles.len());

		for triangle in &self.triangles {
			projected_triangles.push(projection.project_triangle(triangle));
		}

		for triangle in &projected_triangles {
			let line1: Vector3 = triangle.v2 - triangle.v1;
			let line2: Vector3 = triangle.v3 - triangle.v1;
			let normal: Vector3 = Vector3::cross(line1, line2).normalized();
				
			if normal.z > 0.0 && cull_backfaces { continue; }

			let color = Color::new(
				(((normal.x + 1.0) * 0.5) * 255.0) as u8,
				(((normal.y + 1.0) * 0.5) * 255.0) as u8,
				(((normal.z + 1.0) * 0.5) * 255.0) as u8,
				255
			);

			rasterizer.ptriangle(!wireframe, 
				triangle.v1.x as i32, triangle.v1.y as i32,
				triangle.v2.x as i32, triangle.v2.y as i32,
				triangle.v3.x as i32, triangle.v3.y as i32,
				color
			);
		}
	}

	pub fn new_empty() -> Mesh {
		Mesh {
			triangles: Vec::new(),
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
		}
	}
}