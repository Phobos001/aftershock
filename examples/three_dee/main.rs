extern crate sdl2;

mod controls;
mod engine;


use engine::*;

pub fn main() {
    let mut engine = ThreeDeeEngine::new();


    let args: Vec<_> = std::env::args().collect();
    for arg in args {
        match arg.as_str() {
            "--fullscreen" => { engine.fullscreen = true; },
            "--exclusive" => { engine.exclusive = true; },
            "--windowed" => { engine.fullscreen = false; },
            "--hardware-canvas" => { engine.hardware_canvas = true; },
            "--no-integer-scale" => { engine.integer_scaling = false; },
            "--stretch-fill" => { engine.stretch_fill = true; },
            _ => {}
        }
    }

    // If window properties change we can restart SDL2 without ending the game.
    while !engine.is_quitting {
        start_sdl2(&mut engine);
    }
}

pub fn start_sdl2(engine: &mut ThreeDeeEngine) {
    use sdl2::event::Event;
    use sdl2::pixels::{PixelFormatEnum};

    // Init SDL and surface texture
    let sdl_context = sdl2::init().unwrap();

    let video_subsystem = sdl_context.video().unwrap();


    sdl_context.mouse().show_cursor(false);

    let title = ThreeDeeEngine::TITLE;
    let window = {
        match engine.fullscreen {
            true => {
                if engine.exclusive {
                    video_subsystem
                    .window(title, ThreeDeeEngine::RENDER_WIDTH as u32, ThreeDeeEngine::RENDER_WIDTH as u32)
                    .fullscreen()
                    .position_centered()
                    .build()
                    .unwrap()
                } else {
                    video_subsystem
                    .window(title, ThreeDeeEngine::RENDER_WIDTH as u32, ThreeDeeEngine::RENDER_WIDTH as u32)
                    .fullscreen_desktop()
                    .position_centered()
                    .build()
                    .unwrap()
                }
            },
            false => {
                video_subsystem
                .window(title, ThreeDeeEngine::RENDER_WIDTH as u32, ThreeDeeEngine::RENDER_HEIGHT as u32)
                .resizable()
                .position_centered()
                .build()
                .unwrap()
            },
        }
    };

    let mut canvas = {
        if engine.hardware_canvas {
            window.into_canvas().build().map_err(|e| e.to_string()).unwrap()
        } else {
            window.into_canvas().software().build().map_err(|e| e.to_string()).unwrap()
        }
    };

    if !engine.stretch_fill {
        let _ = canvas.set_logical_size(ThreeDeeEngine::RENDER_WIDTH as u32, ThreeDeeEngine::RENDER_HEIGHT as u32);
    }
    
    let _ = canvas.set_integer_scale(engine.integer_scaling);
    let texture_creator = canvas.texture_creator();

    // This is what we update our buffers to
    let mut screentex = texture_creator.create_texture_streaming(PixelFormatEnum::RGBA32,
        ThreeDeeEngine::RENDER_WIDTH as u32,
        ThreeDeeEngine::RENDER_HEIGHT as u32
    )
        .map_err(|e| e.to_string()).unwrap();

    canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();

    // Timings handled with aftershock timestamp, uses std
    let mut update_timer: f64 = 0.0;
    let mut draw_timer: f64 = 0.0;

    let mut delta_now: f64 = aftershock::timestamp();
    let mut delta_last: f64;

    let update_rate: f64 = 1.0 / 300.0;
    let draw_rate: f64 = 1.0 / if engine.hardware_canvas { 300.0 } else { 120.0 };

    engine.init();

    'running: loop {

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => {
                    engine.is_quitting = true;
                    break 'running
                },
                _ => {},
            }
        }

        // Calculate delta times for update
        delta_last = delta_now;
        delta_now = aftershock::timestamp();


        let dt: f64 = delta_now - delta_last;

        engine.dt = dt as f32;
        engine.dt_unscaled = dt as f32;
        engine.realtime += dt as f32;

        update_timer -= dt;
        draw_timer -= dt;

        engine.update();

        let mouse_x = engine.controls.mouse_position.0;
        let mouse_y = engine.controls.mouse_position.1;

        if mouse_x > ThreeDeeEngine::RENDER_WIDTH as i32 {
            sdl_context.mouse().warp_mouse_in_window(&canvas.window(), ThreeDeeEngine::RENDER_WIDTH as i32, mouse_y);
        }

        if mouse_y > ThreeDeeEngine::RENDER_HEIGHT as i32 {
            sdl_context.mouse().warp_mouse_in_window(&canvas.window(), mouse_x, ThreeDeeEngine::RENDER_HEIGHT as i32);
        }
        
        

        if draw_timer <= 0.0 {
            canvas.clear();
            engine.draw();

            let _ = screentex.update(None, &engine.screen.buffer.color, (ThreeDeeEngine::RENDER_WIDTH * 4) as usize);
            let _ = canvas.copy(&screentex, None, None);
            canvas.present();

            draw_timer = draw_rate;
        }
    }
    

    let _error_code = engine.update();
}