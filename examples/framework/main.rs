extern crate sdl2;

mod api_audio;
mod api_color;
mod api_display;
mod api_drawing;
mod api_font;
mod api_image;
mod api_input;
mod api_profiling;
mod api_shareables;

mod controls;
mod engine;
mod lua;

use engine::*;

pub fn main() {
    let mut engine = FrameworkEngine::new();
    let mut script: String = String::from("");

    let args: Vec<_> = std::env::args().collect();
    for i in 0..args.len()-1 {
        match args[i].as_str() {
            "--game" => {
                let lua_main_result = std::fs::read_to_string(args[i+1].as_str());
                if lua_main_result.is_ok() {
                    script = lua_main_result.unwrap();
                } else {
                    println!("{}: {}", args[i+1].as_str(), lua_main_result.err().unwrap());
                }
            },
            "--draw-hz" => {
                let parsed = args[i+1].parse::<f64>();
                if parsed.is_ok() {
                    let hz = parsed.unwrap();
                    //max_draw_hz = 1.0 / hz;
                }
            },
            "--update-hz" => {
                let parsed = args[i+1].parse::<f64>();
                if parsed.is_ok() {
                    let hz = parsed.unwrap();
                    //max_update_hz = 1.0 / hz;
                }
            }
            "--fullscreen" => { engine.fullscreen = true; },
            "--exclusive" => { engine.exclusive = true; },
            "--windowed" => { engine.fullscreen = false; },
            "--hardware-canvas" => { engine.hardware_canvas = true; },
            "--no-integer-scale" => { engine.integer_scaling = false; },
            "--stretch-fill" => { engine.stretch_fill = true; },
            _ => {}
        }
    }


    let mut lua_result = engine.setup_lua(&script);
    if script.is_empty() {
        lua_result.0 = true;
        lua_result.1 = "ERROR: Game not found! Use \"--game <game_path>.lua\" to load your game!\nFor example, \"--game src/main.lua\" or \"--game tools/level_editor.lua\"".to_string();
    }
    println!("{}", lua_result.1);

    // Lua error
    if lua_result.0 {
        start_sdl2_error();
    } else {
        while !engine.is_quitting && engine.error.is_none() {
            // If window properties change we can restart SDL2 without ending the game.
            start_sdl2(&mut engine);
        }

        // If an error comes later
        if engine.error.is_some() {
            start_sdl2_error();
        } 
    }

    
    
}

pub fn start_sdl2(engine: &mut FrameworkEngine) {
    use sdl2::event::Event;
    use sdl2::pixels::{PixelFormatEnum};

    // Init SDL and surface texture
    let sdl_context = sdl2::init().unwrap();

    let video_subsystem = sdl_context.video().unwrap();


    sdl_context.mouse().show_cursor(false);

    let title = FrameworkEngine::TITLE;
    let window = {
        match engine.fullscreen {
            true => {
                if engine.exclusive {
                    video_subsystem
                    .window(title, FrameworkEngine::RENDER_WIDTH as u32, FrameworkEngine::RENDER_WIDTH as u32)
                    .fullscreen()
                    .position_centered()
                    .build()
                    .unwrap()
                } else {
                    video_subsystem
                    .window(title, FrameworkEngine::RENDER_WIDTH as u32, FrameworkEngine::RENDER_WIDTH as u32)
                    .fullscreen_desktop()
                    .position_centered()
                    .build()
                    .unwrap()
                }
            },
            false => {
                video_subsystem
                .window(title, FrameworkEngine::RENDER_WIDTH as u32, FrameworkEngine::RENDER_HEIGHT as u32)
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
        let _ = canvas.set_logical_size(FrameworkEngine::RENDER_WIDTH as u32, FrameworkEngine::RENDER_HEIGHT as u32);
    }
    
    let _ = canvas.set_integer_scale(engine.integer_scaling);
    let texture_creator = canvas.texture_creator();

    // This is what we update our buffers to
    let mut screentex = texture_creator.create_texture_streaming(PixelFormatEnum::RGBA32,
        FrameworkEngine::RENDER_WIDTH as u32,
        FrameworkEngine::RENDER_HEIGHT as u32
    )
        .map_err(|e| e.to_string()).unwrap();

    canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();

    // Timings handled with aftershock timestamp, uses std
    let mut draw_timer: f64 = 0.0;

    let mut delta_now: f64 = aftershock::timestamp();
    let mut delta_last: f64;

    let draw_rate: f64 = 1.0 / if engine.hardware_canvas { 300.0 } else { 120.0 };

    'running: loop {
        if engine.error.is_some() { break 'running }

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

        draw_timer -= dt;

        let update_result = engine.update(dt);
        if update_result.is_err() {
            engine.error = Some(update_result.err().unwrap());
        }
        
        

        if draw_timer <= 0.0 {
            canvas.clear();
            let draw_result = engine.draw();
            if draw_result.is_err() {
                engine.error = Some(draw_result.err().unwrap());
            }

            let lua = engine.lua_main.as_ref().unwrap();

            let _ = screentex.update(None, &lua.buffer.as_ref().borrow().color, (lua.buffer.borrow().width * 4) as usize);
            let _ = canvas.copy(&screentex, None, None);
            canvas.present();

            draw_timer = draw_rate;
        }
    }
}

pub fn start_sdl2_error() {
    use sdl2::event::Event;
    use sdl2::pixels::{PixelFormatEnum};

    // Init SDL and surface texture
    let sdl_context = sdl2::init().unwrap();

    let video_subsystem = sdl_context.video().unwrap();


    let title = "Runtime Lua Error";
    let window = video_subsystem
                            .window(title, 512, 512)
                            .resizable()
                            .position_centered()
                            .build()
                            .unwrap();

    let mut canvas = window.into_canvas().software().build().map_err(|e| e.to_string()).unwrap();

    let _ = canvas.set_logical_size(512, 512);
    let texture_creator = canvas.texture_creator();

    // This is what we update our buffers to
    let mut screentex = texture_creator.create_texture_streaming(PixelFormatEnum::RGBA32,
        512,
        512
    )
        .map_err(|e| e.to_string()).unwrap();

    canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();

    use aftershock::buffer::Buffer;
    use aftershock::color::Color;

    let mut screen = Buffer::new(512, 512);



    screen.clear_color(Color::new(128, 0, 0, 255));

    

    'running: loop {
        canvas.clear();
        let _ = screentex.update(None, &screen.color, (screen.width * 4) as usize);
        let _ = canvas.copy(&screentex, None, None);
        canvas.present();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => {
                    break 'running
                },
                _ => {},
            }
        }

    }
}