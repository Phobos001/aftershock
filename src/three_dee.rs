use glam::*;
use crate::math::*;

use crate::{color::*, buffer::Buffer, partitioned_buffer::PartitionedBuffer};

#[derive(Debug, Copy, Clone)]
pub struct Vertex(pub Vec3);

#[derive(Debug, Copy, Clone)]
pub struct Triangle(pub Vec3, pub Vec3, pub Vec3);

#[derive(Debug, Copy, Clone)]
pub struct Plane { pub position: Vec3, pub normal: Vec3 }

pub enum PlaneSide {
    Front,
    Back,
    On,
}

impl Plane {
    
    pub fn new(position: Vec3, normal: Vec3) -> Plane {
        Plane { position, normal }
    }

    pub fn point_signed_distance(&self, point: Vec3) -> f32 {
        (point - self.position).dot(self.normal)
    }

    pub fn point_on_side(&self, point: Vec3) -> PlaneSide {
        let distance = self.point_signed_distance(point);
        if distance > 0.0 { 
            PlaneSide::Front
        } else if distance < 0.0 {
            PlaneSide::Back
        } else {
            PlaneSide::On
        }
    }


}

#[derive(Debug, Clone)]
pub struct DepthBuffer {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<f32>,
}

impl DepthBuffer {
    pub fn new(width: usize, height: usize) -> DepthBuffer {
        DepthBuffer { width, height, buffer: vec![f32::INFINITY; width * height] }
    }

    pub fn clear(&mut self) {
        self.buffer = vec![f32::INFINITY; self.width * self.height];
    }
}

#[derive(Debug, Clone)]
pub struct Mesh {
    pub verticies: Vec<Vertex>,
    pub indicies: Vec<usize>,
    pub colors: Vec<Color>,
    pub uvs: Vec<Vec2>,
}

#[derive(Debug, Clone)]
pub struct ThreeDeePipeline {
    pub fov: f32,
    pub width: f32,
    pub height: f32,
    pub aspect_ratio: f32,
    pub z_near: f32,
    pub z_far: f32,
    pub projection: Mat4,
    pub view: Mat4,

    pub position: Vec3,
    pub rotation: Quat,
}

impl Mesh {
    pub fn new_quad() -> Mesh {
        let verticies: Vec<Vertex> = [
            Vertex((-0.5, 0.5, 0.0).into()),
            Vertex((0.5, 0.5, 0.0).into()),
            Vertex((0.5, -0.5, 0.0).into()),
            Vertex((-0.5, -0.5, 0.0).into())
        ].to_vec();

        let indicies: Vec<usize> = [0, 1, 2, 0, 3, 2].to_vec();

        let colors: Vec<Color> = [Color::WHITE; 4].to_vec();

        let uvs: Vec<Vec2> = [
            (1.0, 1.0).into(),
            (0.0, 1.0).into(),
            (0.0, 0.0).into(),
            (1.0, 0.0).into()
        ].to_vec();

        Mesh {
            verticies,
            indicies,
            colors,
            uvs,
        }
    }

    pub fn new_cube() -> Mesh {
        let verticies: Vec<Vertex> = [
            Vertex(( -0.5, -0.5, -0.5).into()), // 0 Upper Top Left
            Vertex((  0.5, -0.5, -0.5).into()), // 1 Upper Top Right
            Vertex(( -0.5, -0.5,  0.5).into()), // 2 Upper Bottom Left
            Vertex((  0.5, -0.5,  0.5).into()), // 3 Upper Bottom Right
            Vertex(( -0.5,  0.5, -0.5).into()), // 4 Lower Top Left
            Vertex((  0.5,  0.5, -0.5).into()), // 5 Lower Top Right
            Vertex(( -0.5,  0.5,  0.5).into()), // 6 Lower Bottom Left
            Vertex((  0.5,  0.5,  0.5).into())  // 7 Lower Bottom Right

        ].to_vec();

        let indicies: Vec<usize> = [
            0, 1, 2, 2, 3, 1, // TOP
            4, 5, 6, 6, 7, 5, // BOTTOM
            1, 0, 4, 3, 2, 6,
            ].to_vec();

        let colors: Vec<Color> = [Color::WHITE; 4].to_vec();

        let uvs: Vec<Vec2> = [
            (1.0, 1.0).into(),
            (0.0, 1.0).into(),
            (0.0, 0.0).into(),

            (1.0, 1.0).into(),
            (0.0, 1.0).into(),
            (0.0, 0.0).into(),

            (1.0, 1.0).into(),
            (0.0, 1.0).into(),
            (0.0, 0.0).into(),
            
            (1.0, 1.0).into(),
            (0.0, 1.0).into(),
            (0.0, 0.0).into(),

            (1.0, 1.0).into(),
            (0.0, 1.0).into(),
            (0.0, 0.0).into(),

            (1.0, 1.0).into(),
            (0.0, 1.0).into(),
            (0.0, 0.0).into(),
        ].to_vec();

        Mesh {
            verticies,
            indicies,
            colors,
            uvs,
        }
    }
}

impl ThreeDeePipeline {
    pub fn new_perspective(fov: f32, width: f32, height: f32, z_near: f32, z_far: f32) -> ThreeDeePipeline {
        let aspect_ratio: f32 = width / height;
        let projection: Mat4 = Mat4::perspective_lh(fov.to_radians(), aspect_ratio, z_near, z_far);
        
        
        ThreeDeePipeline { fov, width, height, aspect_ratio, z_near, z_far, projection, view: Mat4::IDENTITY, position: Vec3::ZERO, rotation: Quat::IDENTITY }
    }

    pub fn set_position(&mut self, position: Vec3) {
        self.position = position;
        self.update_view();
    }

    pub fn set_rotation(&mut self, rotation: Quat) {
        self.rotation = rotation;
        self.update_view();
    }

    pub fn update_view(&mut self) {
        self.projection = Mat4::perspective_lh(self.fov.to_radians(), self.aspect_ratio, self.z_near, self.z_far);
        self.view = Mat4::from_quat(self.rotation) * Mat4::from_translation(self.position);
    }

    pub fn project_point_to_screen(&self, point: Vec3) -> Vec3 {
        let cmtx = (self.projection * self.view);
        let mut proj_point: Vec3 = cmtx.transform_point3(point);

        proj_point.x /= proj_point.z;
        proj_point.y /= proj_point.z;

        proj_point.x += 1.0;
        proj_point.y += 1.0;

        proj_point.x *= 0.5 * self.width;
        proj_point.y *= 0.5 * self.height;

        proj_point
    }

    pub fn project_triangle_to_screen(&self, tri: Triangle, transform: Mat4) -> Triangle {
        let cmtx = (self.projection * self.view * transform);
        let mut proj_tri: Triangle = Triangle (cmtx.transform_point3(tri.0), cmtx.transform_point3(tri.1), cmtx.transform_point3(tri.2));

        proj_tri.0.x /= proj_tri.0.z;
        proj_tri.0.y /= proj_tri.0.z;

        proj_tri.1.x /= proj_tri.1.z;
        proj_tri.1.y /= proj_tri.1.z;

        proj_tri.2.x /= proj_tri.2.z;
        proj_tri.2.y /= proj_tri.2.z;

        proj_tri.0.x += 1.0; proj_tri.1.x += 1.0; proj_tri.2.x += 1.0;
        proj_tri.0.y += 1.0; proj_tri.1.y += 1.0; proj_tri.2.y += 1.0;

        proj_tri.0.x *= 0.5 * self.width;  proj_tri.1.x *= 0.5 * self.width;  proj_tri.2.x *= 0.5 * self.width;
        proj_tri.0.y *= 0.5 * self.height; proj_tri.1.y *= 0.5 * self.height; proj_tri.2.y *= 0.5 * self.height;

        proj_tri
    }

    pub fn draw_mesh_to_screen(&self, buffer: &mut PartitionedBuffer, mesh: &Mesh, texture: &Buffer, transform: Mat4) {
        for verts in mesh.indicies.chunks_exact(3).enumerate() {
            let (index, v0, v1, v2) = (verts.0, mesh.verticies[verts.1[0]], mesh.verticies[verts.1[1]], mesh.verticies[verts.1[2]]);
            let (uv0, uv1, uv2) = (mesh.uvs[index + 0], mesh.uvs[index + 1], mesh.uvs[index + 2]);

            let triangle: Triangle = Triangle(v0.0, v1.0, v2.0);
            let projtri = self.project_triangle_to_screen(triangle, transform);

            buffer.ptritex_uvw(
                projtri.0.x as i32, projtri.0.y as i32, 
                projtri.1.x as i32, projtri.1.y as i32, 
                projtri.2.x as i32, projtri.2.y as i32, 
                uv0.x, uv0.y, projtri.0.z,
                uv1.x, uv1.y, projtri.1.z,
                uv2.x, uv2.y, projtri.2.z,
                texture
            );
        }
    }
}

