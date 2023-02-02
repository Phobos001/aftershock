extern crate minifb;
extern crate sdl2;

mod aabb;
mod controls;
mod engine;
mod gamestate;

use engine::*;

pub enum Sdl2VideoMode {
    Windowed,
    Fullscreen,
    Exclusive,
}

pub fn main() {
    let mut engine = PlatformerEngine::new();

    let mut use_minifb: bool = false;

    let args: Vec<_> = std::env::args().collect();
    for arg in args {
        match arg.as_str() {
            "--fullscreen" => { engine.fullscreen = true; },
            "--windowed" => { engine.fullscreen = false; },
            "--hardware-canvas" => { engine.hardware_canvas = true; },
            "--no-integer-scale" => { engine.integer_scaling = false; },
            "--stretch-fill" => { engine.stretch_fill = true; },
            "--minifb" => {  use_minifb = true; },
            _ => {}
        }
    }

    // If window properties change we can restart SDL2 without ending the game.
    while !engine.is_quitting {
        if use_minifb {
            println!("Starting minifb...");
            start_minifb(&mut engine);
        } else {
            println!("Starting sdl2...");
            start_sdl2(&mut engine);
        }
    }
}

pub fn start_sdl2(engine: &mut PlatformerEngine) {
    use sdl2::event::Event;
    use sdl2::pixels::{PixelFormatEnum};
    
    let mut video_mode: Sdl2VideoMode = Sdl2VideoMode::Windowed;

    // Init SDL and surface texture
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let title = PlatformerEngine::TITLE;
    let window = {
        match engine.fullscreen {
            true => {
                video_subsystem
                .window(title, PlatformerEngine::RENDER_WIDTH as u32, PlatformerEngine::RENDER_WIDTH as u32)
                .fullscreen()
                .position_centered()
                .build()
                .unwrap()
            },
            false => {
                video_subsystem
                .window(title, PlatformerEngine::RENDER_WIDTH as u32, PlatformerEngine::RENDER_HEIGHT as u32)
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
        let _ = canvas.set_logical_size(PlatformerEngine::RENDER_WIDTH as u32, PlatformerEngine::RENDER_HEIGHT as u32);
    }
    
    let _ = canvas.set_integer_scale(engine.integer_scaling);
    let texture_creator = canvas.texture_creator();

    // This is what we update our buffers to
    let mut screentex = texture_creator.create_texture_streaming(PixelFormatEnum::RGBA32,
        PlatformerEngine::RENDER_WIDTH as u32,
        PlatformerEngine::RENDER_HEIGHT as u32
    )
        .map_err(|e| e.to_string()).unwrap();

    canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();

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

        engine.update_times();

        engine.update();
        

        if engine.present_time <= 0.0 {
            canvas.clear();
            engine.draw();

            let _ = screentex.update(None, &engine.screen.color, (PlatformerEngine::RENDER_WIDTH * 4) as usize);
            let _ = canvas.copy(&screentex, None, None);
            canvas.present();

            engine.present_time = 1.0 / if engine.hardware_canvas { 240.0 } else { 60.0 };
        }
    }
    

    let _error_code = engine.update();
}

pub fn start_minifb(engine: &mut PlatformerEngine) -> u8 {
    use minifb::*;
    
    // Init MINIFB stuff
    let mut window = match Window::new(
        "Asteroids!",
        PlatformerEngine::RENDER_WIDTH,
        PlatformerEngine::RENDER_HEIGHT,
        WindowOptions {
            resize: true,
            scale: Scale::FitScreen,
            scale_mode: ScaleMode::AspectRatioStretch,
            ..WindowOptions::default()
        },
    ) {
        Ok(win) => win,
        Err(err) => {
            println!("Unable to create window {}", err);
            return 1;
        }
    };

    while window.is_open() {
        engine.update_times();
        engine.update();

        if engine.present_time <= 0.0 {
            engine.draw();

            let colors_u32: Vec<u32> = engine.screen.color.chunks_exact(4)
            .map(|c| (c[0] as u32) << 16 | (c[1] as u32) << 8 | (c[2] as u32) << 0)
            .collect();
    
            // Present
            window
            .update_with_buffer(colors_u32.as_slice(), PlatformerEngine::RENDER_WIDTH, PlatformerEngine::RENDER_HEIGHT)
            .unwrap();

            engine.present_time = 1.0 / 60.0;
        }
    }

    engine.is_quitting = true;

    return 0;
}