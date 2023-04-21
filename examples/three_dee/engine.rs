use aftershock::buffer::*;
use aftershock::font::*;
use aftershock::color::*;
use aftershock::partitioned_buffer::*;
use aftershock::shader::*;
use aftershock::three_dee::*;
use glam::*;

use crate::controls::*;

#[derive(Debug, Clone)]
pub struct ShaderDepthBuffer {
    pub depth_buffer: DepthBuffer,
}

impl Shader for ShaderDepthBuffer {
    fn shade(&mut self, buffer: &[u8], width: usize, height: usize, params: ShaderParams) -> Option<(i32, i32, Color)> {
        let pixel_depth: f32 = params.p_f32[2];

        if (self.depth_buffer.width * self.depth_buffer.height) != width * height { 
            //println!("ERROR: ShaderDepthBuffer and Buffer have different lengths!!");
            return Some((params.x, params.y, params.color));
        }

        let depth_idx: usize = ((params.y * (width as i32) + params.x)) as usize;

        // Single rule, the lower the number the closer it is.
        if pixel_depth < self.depth_buffer.buffer[depth_idx] {
            self.depth_buffer.buffer[depth_idx] = pixel_depth;
            
            Some((params.x, params.y, params.color))
        } else {
            None
        }
    }

    fn reset(&mut self) {self.depth_buffer.clear(); }
}

pub struct ThreeDeeEngine {
    pub screen: PartitionedBuffer,

    pub patterntest: Buffer,

    pub hardware_canvas: bool,
    pub integer_scaling: bool,
    pub stretch_fill: bool,
    pub fullscreen: bool,
    pub exclusive: bool,

    pub controls: Controls,

    pub quad: Mesh,
    pub pipeline: ThreeDeePipeline,
    pub shader_depth_buffer: ShaderDepthBuffer,

    pub paused: bool,

    pub main_font: Font,
    pub cursor: Buffer,

    pub tics: u64,
    pub realtime: f32,
    pub timescale: f32,

    pub dt: f32,
    pub dt_unscaled: f32,

    pub profiling_update_time: f64,
    pub profiling_draw_time: f64,
    
    pub present_time: f32,

    pub is_quitting: bool,

    

}

impl ThreeDeeEngine {
    pub const TITLE: &str = "Platformer Example";

    pub const RENDER_WIDTH: usize = 960;
    pub const RENDER_HEIGHT: usize = 540;

    pub fn new() -> ThreeDeeEngine {
        println!("== Platformer Example ==");

        // Font images will be read left-to-right, top-to-bottom. 
        // This will tell the Font what character goes to what part of the image.
        let tinyfont10_glyphidx = "ABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890!?/\\@#$%^&*()[]_-+=\"';:.";

        let main_font = match Font::new("shared_assets/tiny_font10.png", tinyfont10_glyphidx, 10, 10, 1) {
            Ok(font) => { font },
            Err(_) => { Font::default() },
        };

        ThreeDeeEngine {
            hardware_canvas: false,
            integer_scaling: true,
            stretch_fill: false,
            fullscreen: true,
            exclusive: false,

            main_font,
            cursor: Buffer::new_from_image("shared_assets/cursor.png").unwrap_or_default(),

            // Always draw in Parallel using a pixel-work threshold of 0, or else the depth buffer shader will switch between the
            // full buffer and the split partitions. This can cause 3D meshes to draw depth but not write any color to the screen.
            screen: PartitionedBuffer::new(ThreeDeeEngine::RENDER_WIDTH, ThreeDeeEngine::RENDER_HEIGHT, 0, 0),

            patterntest: Buffer::new_from_image("shared_assets/patterntest.png").unwrap_or_default(),

            controls: Controls::new(),

            pipeline: ThreeDeePipeline::new_perspective(100.0, ThreeDeeEngine::RENDER_WIDTH as f32, ThreeDeeEngine::RENDER_HEIGHT as f32, 0.001, 1024.0),
            quad: Mesh::new_cube(),
            shader_depth_buffer: ShaderDepthBuffer { depth_buffer: DepthBuffer::new(ThreeDeeEngine::RENDER_WIDTH, ThreeDeeEngine::RENDER_HEIGHT) },

            paused: false,
            
            dt: 0.0,
            dt_unscaled: 0.0,
            realtime: 0.0,
            timescale: 1.0,

            tics: 0,

            profiling_update_time: 0.0,
            profiling_draw_time: 0.0,

            present_time: 0.0,

            is_quitting: false,
		}
	}

    pub fn init(&mut self) {
        self.screen.add_shader(BufferShader::new(Box::new(self.shader_depth_buffer.clone()), true, 0));
        self.screen.add_shader(BufferShader::new(Box::new(ShaderOpaque), false, 1));
        self.pipeline.set_position(glam::Vec3::new(0.0, 0.0, 5.0));
    }

    pub fn update(&mut self) {
        let update_time_before: f64 = aftershock::timestamp();
        self.controls.update();

        let mut target_velocity: Vec3 = Vec3::ZERO;
        if self.controls.is_control_down(ControlKeys::MoveForward) {
            target_velocity += Vec3::new(0.0, 0.0, -1.0) * self.dt * 8.0;
        }

        if self.controls.is_control_down(ControlKeys::MoveBackward) {
            target_velocity += Vec3::new(0.0, 0.0, 1.0) * self.dt * 8.0;
        }

        if self.controls.is_control_down(ControlKeys::MoveLeft) {
            target_velocity += Vec3::new(1.0, 0.0, 0.0) * self.dt * 8.0;
        }

        if self.controls.is_control_down(ControlKeys::MoveRight) {
            target_velocity += Vec3::new(-1.0, 0.0, 0.0) * self.dt * 8.0;
        }

        if self.controls.is_control_down(ControlKeys::MoveUp) {
            target_velocity += Vec3::new(0.0, 1.0, 0.0) * self.dt * 8.0;
        }

        if self.controls.is_control_down(ControlKeys::MoveDown) {
            target_velocity += Vec3::new(0.0, -1.0, 0.0) * self.dt * 8.0;
        }

        target_velocity = self.pipeline.rotation * target_velocity;
        self.pipeline.set_position(self.pipeline.position + target_velocity);
        
        self.pipeline.update_view();
        let update_time_after: f64 = aftershock::timestamp();



        self.profiling_update_time = update_time_after - update_time_before;

        self.tics += 1;

        // Give the processor a break
        std::thread::sleep(std::time::Duration::from_micros(1));
    }

    pub fn draw(&mut self) {
        let draw_time_before: f64 = aftershock::timestamp();
        self.screen.clear();


        let cube1_transform =   Mat4::from_scale(glam::Vec3::ONE) * 
                                Mat4::from_translation(glam::Vec3::ZERO) *
                                Mat4::from_euler(glam::EulerRot::XYZ, 0.0, self.realtime, 0.0);

        let cube2_transform =   Mat4::from_scale(Vec3::ONE) * 
                                Mat4::from_translation(Vec3::new(-1.0, 0.5, 0.0)) *
                                Mat4::from_euler(EulerRot::XYZ, 0.0, -self.realtime, 0.0);

        // Enable depth buffer shader [0] and disable alpha clip [1]
        self.screen.buffer.shader_stack[0].active = true;
        self.screen.buffer.shader_stack[1].active = false;
        self.screen.buffer.shader_stack[0].shader.as_mut().reset();

        self.pipeline.draw_mesh_to_screen(&mut self.screen, &self.quad, &self.patterntest, cube1_transform); 
        self.pipeline.draw_mesh_to_screen(&mut self.screen, &self.quad, &self.patterntest, cube2_transform); 
                                
        self.screen.enable_drawing();
        let draw_time_after: f64 = aftershock::timestamp();
        self.profiling_draw_time = draw_time_after - draw_time_before;

        // Disable depth drawing and enable alpha clip
        self.screen.buffer.shader_stack[0].active = false;
        self.screen.buffer.shader_stack[1].active = true;


        self.screen.buffer.pprint(&self.main_font, format!("UPDATE TIME: {:.02}MS\nDRAW TIME: {:.02}MS\nTICS: {}\nRT: {:.02}s", 
        (self.profiling_update_time * 100000.0).round() / 100.0, 
        (self.profiling_draw_time * 100000.0).round() / 100.0,
        self.tics, self.realtime),
        4, 4, 10, None);

        // Cursor
        self.screen.pimg(&self.cursor, self.controls.mouse_position.0, self.controls.mouse_position.1);
    }

}