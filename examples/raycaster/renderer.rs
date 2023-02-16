use aftershock::vector2::*;
use aftershock::buffer::*;

use crate::engine::*;
use crate::level::*;

use std::thread::scope;

pub struct Renderer {
    pub screen: Buffer,
    pub threaded_buffers: Vec<Buffer>,
}

impl Renderer {
    pub const HORIZONTAL_PROJECTION_RATIO: f32 = 150.0 / 384.0; // 384 reference width
    pub const VERTICAL_PROJECTION_RATIO: f32 = 64.0 / 216.0; // 216 reference height

    pub fn new(width: usize, height: usize, threads: usize) -> Renderer {
        let mut threaded_buffers: Vec<Buffer> = Vec::new();
        let width_div: usize = width / threads;
        let width_div_remainder: usize = width - (width_div * threads);

        for i in 0..threads {
            let mut buffer = Buffer::new(width_div, height);
            buffer.offset_x = width_div * i;
            threaded_buffers.push(buffer);
        }

        if width_div_remainder != 0 {
            let idx = threaded_buffers.len()-1;
            threaded_buffers[idx] = Buffer::new(width_div + width_div_remainder, height);
        }

        Renderer { screen: Buffer::new(width, height), threaded_buffers, }
    }

    pub fn draw_sector(&mut self, level: &Level, sector_idx: usize, camera_position: Vector2, camera_rotation: f32, camera_height: f32, camera_look: f32) {
        let sector = &level.sectors[sector_idx];
    
        let mut pvs_lines: Vec<Line> = Vec::new();
    
        for line in &sector.lines {
            pvs_lines.push(level.lines[*line]);
        }

        // Draw the world in all the threads avalible to the CPU
        scope(|s| {
            let mut join_handles: Vec<std::thread::ScopedJoinHandle<()>> = Vec::new();

            for thbuf in &mut self.threaded_buffers {

                let pvs_lines_ref = pvs_lines.clone();
                let buffer_ref: &Buffer = &self.screen;
                let handle = s.spawn(move || {
                    thbuf.clear();

                    let half_width = buffer_ref.width as f32 * 0.5;
                    let half_height = buffer_ref.height as f32 * 0.5;

                    let thbuf_half_width = buffer_ref.width as f32 * 0.5;
                    let thbuf_half_height = buffer_ref.height as f32 * 0.5;
                
                    //let center: Vector2 = Vector2::new(half_width, half_height);
                
                
                    thbuf.set_draw_mode(DrawMode::NoOp);
                
                    for line in &pvs_lines_ref {
                
                        // Offset line by camera position in 2D space
                        let offset_line = Line {
                            start: line.start - camera_position,
                            end: line.end - camera_position,
                            flipped: false,
                            tint: line.tint,
                        };
                
                        // Spin line around 0, 0 (Now the relative camera location)
                        let mut rotated_line = Line {
                            start: offset_line.start.rotated_pivot(camera_rotation, Vector2::ZERO),
                            end: offset_line.end.rotated_pivot(camera_rotation, Vector2::ZERO),
                            flipped: false,
                            tint: line.tint,
                        };
                
                        // Cannot be seen by player as both points are behind the camera.
                        if rotated_line.start.y > 0.0 && rotated_line.end.y > 0.0 { continue; }
                
                        if rotated_line.start.y > 0.0 {
                
                            // This should always work after the first test but we'll play it safe
                            let clip_point_opt = Vector2::intersection_infinite(
                                Vector2::ZERO, Vector2::new(-1.0, 0.0),
                                 rotated_line.start, rotated_line.end
                            );
                
                            if clip_point_opt.is_some() {
                                rotated_line.start = clip_point_opt.unwrap();
                                rotated_line.start.y -= 0.05;
                            }
                        } else if rotated_line.end.y > 0.0 {
                
                            // This should always work after the first test but we'll play it safe
                            let clip_point_opt = Vector2::intersection_infinite(
                                Vector2::ZERO, Vector2::new(-1.0, 0.0),
                                 rotated_line.start, rotated_line.end
                            );
                
                            if clip_point_opt.is_some() {
                                rotated_line.end = clip_point_opt.unwrap();
                                rotated_line.end.y -= 0.05;
                            }
                        }
                
                        let projection_horizontal: f32 = Renderer::HORIZONTAL_PROJECTION_RATIO * RebuiltEngine::RENDER_WIDTH as f32;
                        let projection_vertical: f32 = Renderer::VERTICAL_PROJECTION_RATIO * RebuiltEngine::RENDER_HEIGHT as f32;
                
                        // Divide X (horizontal space) by Y (distance) to project line to screen space
                        let projected_line_bottom = Line {
                            start: Vector2::new(-(rotated_line.start.x * projection_horizontal) / rotated_line.start.y, 
                            (sector.height_floor - camera_height) / rotated_line.start.y * projection_vertical)
                             - Vector2::new(thbuf.offset_x as f32, 0.0),

                            end: Vector2::new(-(rotated_line.end.x * projection_horizontal) / rotated_line.end.y, 
                            (sector.height_floor - camera_height) / rotated_line.end.y * projection_vertical)
                            - Vector2::new(thbuf.offset_x as f32, 0.0),

                            flipped: false,
                            tint: line.tint,
                        };
                
                        let projected_line_top = Line {
                            start: Vector2::new(-(rotated_line.start.x * projection_horizontal) / rotated_line.start.y, 
                            (sector.height_ceiling - camera_height) / rotated_line.start.y * projection_vertical)
                            - Vector2::new(thbuf.offset_x as f32, 0.0),

                            end: Vector2::new(-(rotated_line.end.x * projection_horizontal) / rotated_line.end.y, 
                            (sector.height_ceiling - camera_height) / rotated_line.end.y * projection_vertical)
                            - Vector2::new(thbuf.offset_x as f32, 0.0),

                            flipped: false,
                            tint: line.tint,
                        };
                        
                        
                
                        let x_start_bottom = (projected_line_bottom.start.x + half_width) as i32;
                        let x_end_bottom = (projected_line_bottom.end.x + half_width) as i32;
                
                        let y_start_bottom = (projected_line_bottom.start.y + half_height) as i32;
                        let y_end_bottom = (projected_line_bottom.end.y + half_height) as i32;
                
                        let x_start_top = (projected_line_top.start.x + half_width) as i32;
                        let x_end_top = (projected_line_top.end.x + half_width) as i32;
                
                        let y_start_top = (projected_line_top.start.y + half_height) as i32;
                        let y_end_top = (projected_line_top.end.y + half_height) as i32;
                
                        // 'Draw' two lines to find the y ranges for the columns we have to draw
                        // Uses a double brensenham line algorithm
                        let (mut x0b, mut y0b, x1b, y1b) = (x_start_bottom, y_start_bottom, x_end_bottom, y_end_bottom);
                        let (mut x0t, mut y0t, x1t, y1t) = (x_start_top, y_start_top, x_end_top, y_end_top);
                
                        let dxb = i32::abs(x1b - x0b);
                        let sxb = if x0b < x1b {1} else {-1};
                        let dyb = -i32::abs(y1b - y0b);
                        let syb = if y0b < y1b {1} else {-1};
                
                        let dxt = i32::abs(x1t - x0t);
                        let sxt = if x0t < x1t {1} else {-1};
                        let dyt = -i32::abs(y1t- y0t);
                        let syt = if y0t < y1t {1} else {-1};
                
                        let mut error_b = dxb + dyb;
                        let mut error_t = dxt + dyt;
                
                        const MAX_RESOLUTION: usize = 4096;
                
                        let mut columns_top: [i32; MAX_RESOLUTION]    = [0; MAX_RESOLUTION];
                        let mut columns_bottom: [i32; MAX_RESOLUTION] = [0; MAX_RESOLUTION];
                
                        loop {
                            if x0b > 0 && x0b < thbuf.width as i32 { columns_bottom[x0b as usize] = y0b; }
                
                            if x0b == x1b && y0b == y1b { break; }
                            let e2 = 2 * error_b;
                            if e2 >= dyb {
                                if x0b == x1b { break; }
                                error_b += dyb;
                                x0b += sxb;
                            }
                            if e2 <= dxb {
                                if y0b == y1b { break; }
                                error_b += dxb;
                                y0b += syb;
                            }
                
                            
                        }
                
                        loop {
                            if x0t > 0 && x0t < thbuf.width as i32 { columns_top[x0t as usize] = y0t; }
                
                            if x0t == x1t && y0t == y1t { break; }
                            let e2 = 2 * error_t;
                            if e2 >= dyt {
                                if x0t == x1t { break; }
                                error_t += dyt;
                                x0t += sxt;
                            }
                            if e2 <= dxt {
                                if y0t == y1t { break; }
                                error_t += dxt;
                                y0t += syt;
                            }
                        }
                
                        let mut x_start_clamp = i32::clamp(x_start_bottom, 0, thbuf.width as i32);
                        let mut x_end_clamp = i32::clamp(x_end_bottom, 0, thbuf.width as i32);
                
                        // Swap iterators if one is bigger than the other
                        // This usually happens if the camera is viewing the backside of a line
                        if x_start_clamp > x_end_clamp {
                            let temp: i32 = x_end_clamp;
                            x_end_clamp = x_start_clamp;
                            x_start_clamp = temp;
                        }
                
                        for column in x_start_clamp..x_end_clamp {
                
                            let mut y_top: i32 = i32::clamp(columns_top[column as usize], 0, thbuf.height as i32);
                            let mut y_bottom: i32 = i32::clamp(columns_bottom[column as usize], 0, thbuf.height as i32);
                
                            if y_top > y_bottom {
                                let temp: i32 = y_bottom;
                                y_top = y_bottom;
                                y_bottom = temp;
                            }
                
                            for py in y_top..y_bottom {
                                thbuf.pset_panic_oob(column, py, line.tint);
                            }
                            
                        }
                    }

                    ()
                });

                join_handles.push(handle);
            }

            for j in join_handles {
                let _ = j.join();
            }

        });

        for thbuf in &self.threaded_buffers {
            self.screen.blit(&thbuf, thbuf.offset_x as i32, 0);
        }
    
        
    }
}