use aftershock::vector2::*;
use aftershock::buffer::*;
use aftershock::color::*;

use crate::level::*;



pub fn draw_sector(level: &Level, sector_idx: usize, buffer: &mut Buffer, camera_position: Vector2, camera_rotation: f32, camera_height: f32, camera_look: f32) {
    let sector = &level.sectors[sector_idx];

    let mut pvs_lines: Vec<Line> = Vec::new();

    for line in &sector.lines {
        // Backface culling
        //let line_normal = Vector2::direction(line.start, line.end).rotated((-90.0 as f32).to_radians());
        //if Vector2::dot(view_direction, line_normal) < 0.0 {
            pvs_lines.push(*line);
        //}
    }

    let half_width = buffer.width as f32 * 0.5;
    let half_height = buffer.height as f32 * 0.5;

    let center: Vector2 = Vector2::new(half_width, half_height);


    buffer.set_draw_mode(DrawMode::NoOp);

    for line in &pvs_lines {

        let offset_line = Line {
            start: line.start - camera_position,
            end: line.end - camera_position,
            height_bottom: line.height_bottom,
            height_top: line.height_top,
            flipped: false
        };

        let mut rotated_line = Line {
            start: offset_line.start.rotated_pivot(camera_rotation, Vector2::ZERO),
            end: offset_line.end.rotated_pivot(camera_rotation, Vector2::ZERO),
            height_bottom: line.height_bottom,
            height_top: line.height_top,
            flipped: false,
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

        const HORIZONTAL_PROJECTION: f32 = 150.0;
        const VERTICAL_PROJECTION: f32 = 64.0;

        // Divide X (horizontal space) by Y (distance) to project line to screen space
        let projected_line_bottom = Line {
            start: Vector2::new(-(rotated_line.start.x * HORIZONTAL_PROJECTION) / rotated_line.start.y, (line.height_bottom - camera_height) / rotated_line.start.y * VERTICAL_PROJECTION),
            end: Vector2::new(-(rotated_line.end.x * HORIZONTAL_PROJECTION) / rotated_line.end.y, (line.height_bottom - camera_height) / rotated_line.end.y * VERTICAL_PROJECTION),
            height_bottom: line.height_bottom,
            height_top: line.height_top,
            flipped: false,
        };

        let projected_line_top = Line {
            start: Vector2::new(-(rotated_line.start.x * HORIZONTAL_PROJECTION) / rotated_line.start.y, (line.height_top - camera_height) / rotated_line.start.y * VERTICAL_PROJECTION),
            end: Vector2::new(-(rotated_line.end.x * HORIZONTAL_PROJECTION) / rotated_line.end.y, (line.height_top - camera_height) / rotated_line.end.y * VERTICAL_PROJECTION),
            height_bottom: line.height_bottom,
            height_top: line.height_top,
            flipped: false,
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

        let mut columns_top: [i32; 2048]    = [0; 2048];
        let mut columns_bottom: [i32; 2048] = [0; 2048];

        loop {
            if x0b > 0 && x0b < 2048 { columns_bottom[x0b as usize] = y0b; }

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
            if x0t > 0 && x0t < 2048 { columns_top[x0t as usize] = y0t; }

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

        let mut x_start_clamp = i32::clamp(x_start_bottom, 0, buffer.width as i32);
        let mut x_end_clamp = i32::clamp(x_end_bottom, 0, buffer.width as i32);

        // Swap iterators if one is bigger than the other
        // This usually happens if the camera is viewing the backside of a line
        if x_start_clamp > x_end_clamp {
            let temp: i32 = x_end_clamp;
            x_end_clamp = x_start_clamp;
            x_start_clamp = temp;
        }

        for column in x_start_clamp..x_end_clamp {

            let mut y_top: i32 = i32::clamp(columns_top[column as usize], 0, buffer.height as i32);
            let mut y_bottom: i32 = i32::clamp(columns_bottom[column as usize], 0, buffer.height as i32);

            if y_top > y_bottom {
                let temp: i32 = y_bottom;
                y_top = y_bottom;
                y_bottom = temp;
            }

            for py in y_top..y_bottom {
                buffer.pset_panic_oob(column, py, Color::YELLOW);
            }
            
        }

    }

    buffer.pset(center.x as i32, center.y as i32, Color::GREEN);
}

fn clip_line_to_buffer(buffer: &Buffer, line: &mut Line) {

    let width: f32 = buffer.width as f32;
    let height: f32 = buffer.height as f32;

    let clip_top = Vector2::intersection_segment(
        line.start, line.end, 
        Vector2::new(0.0, 0.0), Vector2::new(width, 0.0))
    ;

    let clip_bottom = Vector2::intersection_segment(
        line.start, line.end, 
        Vector2::new(0.0, height), Vector2::new(width, height))
    ;

    let clip_left = Vector2::intersection_segment(
        line.start, line.end, 
        Vector2::new(0.0, 0.0), Vector2::new(0.0, height))
    ;

    let clip_right = Vector2::intersection_segment(
        line.start, line.end, 
        Vector2::new(width, 0.0), Vector2::new(width, height))
    ;

    // No intersection with screen borders
    if clip_top.is_none() && clip_bottom.is_none() && clip_left.is_none() && clip_right.is_none() { return; }

    // It's impossible for a line to intersect with more than two sides
    // Get two intersections and then change the start and end to the points (order doesn't matter at this stage)

    // Five cases of intersections: TB, TL, TR, BL, BR

    if clip_top.is_some() && clip_bottom.is_some() {
        line.start = clip_top.unwrap();
        line.end = clip_bottom.unwrap();
        return;
    }

    if clip_top.is_some() && clip_left.is_some() {
        line.start = clip_top.unwrap();
        line.end = clip_left.unwrap();
        return;
    }

    if clip_top.is_some() && clip_right.is_some() {
        line.start = clip_top.unwrap();
        line.end = clip_right.unwrap();
        return;
    }

    if clip_left.is_some() && clip_right.is_some() {
        line.start = clip_left.unwrap();
        line.end = clip_right.unwrap();
        return;
    }

    if clip_bottom.is_some() && clip_right.is_some() {
        line.start = clip_bottom.unwrap();
        line.end = clip_right.unwrap();
        return;
    }

    if clip_bottom.is_some() && clip_left.is_some() {
        line.start = clip_bottom.unwrap();
        line.end = clip_left.unwrap();
        return;
    }
}