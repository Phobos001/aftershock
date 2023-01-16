extern crate minifb;
extern crate sdl2;

mod asteroids_engine;

use asteroids_engine::*;

pub enum Sdl2VideoMode {
    Windowed,
    Fullscreen,
    Exclusive,
}

pub fn main() {
    let mut engine = AsteroidsEngine::new();

    let args: Vec<_> = std::env::args().collect();
    for arg in args {
        match arg.as_str() {
            "--sdl2" => { println!("Starting SDL2..."); start_sdl2(&mut engine); },
            "--minifb" => { println!("Starting minifb..."); start_minifb(&mut engine); },
            _ => { println!("Use flags \"--minifb\" or \"--sdl2\" to pick a frontend to use.")}
        }
    }

}

pub fn start_minifb(engine: &mut AsteroidsEngine) -> u8 {
    use minifb::*;
    
    // Init MINIFB stuff
    let mut window = match Window::new(
        "Asteroids!",
        AsteroidsEngine::RENDER_WIDTH,
        AsteroidsEngine::RENDER_HEIGHT,
        WindowOptions {
            resize: false,
            scale: Scale::FitScreen,
            ..WindowOptions::default()
        },
    ) {
        Ok(win) => win,
        Err(err) => {
            println!("Unable to create window {}", err);
            return 1;
        }
    };

    while window.is_open() && !window.is_key_down(Key::Escape) {
        engine.update();

        if engine.present_time <= 0.0 {
            engine.draw();

            let colors_u32: Vec<u32> = engine.rasterizer.color.chunks_exact(4)
            .map(|c| (c[0] as u32) << 16 | (c[1] as u32) << 8 | (c[2] as u32) << 0)
            .collect();
    
            // Present
            window
            .update_with_buffer(colors_u32.as_slice(), AsteroidsEngine::RENDER_WIDTH, AsteroidsEngine::RENDER_HEIGHT)
            .unwrap();

            engine.present_time = 1.0 / 120.0;
        }
    }

    return 0;
}

pub fn start_sdl2(engine: &mut AsteroidsEngine) {
    use sdl2::event::Event;
    use sdl2::pixels::{PixelFormatEnum};
    
    let mut hardware_accelerated: bool = false;
    let mut video_mode: Sdl2VideoMode = Sdl2VideoMode::Windowed;

    let args: Vec<_> = std::env::args().collect();
    for arg in args {
        match arg.as_str() {
            "--exclusive" => { video_mode = Sdl2VideoMode::Exclusive; },
            "--fullscreen" => { video_mode = Sdl2VideoMode::Fullscreen; },
            "--windowed" => { video_mode = Sdl2VideoMode::Windowed; },
            "--hardware-canvas" => { hardware_accelerated = true; }
            _ => {}
        }
    }

    // Init SDL and surface texture
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let title = "Asteroids!";
    let window = {
        match video_mode {
            Sdl2VideoMode::Exclusive => {
                video_subsystem
                .window(title, AsteroidsEngine::RENDER_WIDTH as u32, AsteroidsEngine::RENDER_WIDTH as u32)
                .fullscreen()
                .position_centered()
                .build()
                .unwrap()
            },
            Sdl2VideoMode::Fullscreen => {
                video_subsystem
                .window(title, AsteroidsEngine::RENDER_WIDTH as u32, AsteroidsEngine::RENDER_WIDTH as u32)
                .fullscreen_desktop()
                .position_centered()
                .build()
                .unwrap()
            },
            Sdl2VideoMode::Windowed => {
                video_subsystem
                .window(title, AsteroidsEngine::RENDER_WIDTH as u32, AsteroidsEngine::RENDER_HEIGHT as u32)
                .resizable()
                .position_centered()
                .build()
                .unwrap()
            },
        }
    };

    let mut canvas = {
        if hardware_accelerated {
            window.into_canvas().build().map_err(|e| e.to_string()).unwrap()
        } else {
            window.into_canvas().software().build().map_err(|e| e.to_string()).unwrap()
        }
    };

    let _ = canvas.set_logical_size(AsteroidsEngine::RENDER_WIDTH as u32, AsteroidsEngine::RENDER_HEIGHT as u32);
    let _ = canvas.set_integer_scale(true);
    let texture_creator = canvas.texture_creator();

    // This is what we update our buffers to
    let mut screentex = texture_creator.create_texture_streaming(PixelFormatEnum::RGBA32,
        AsteroidsEngine::RENDER_WIDTH as u32,
        AsteroidsEngine::RENDER_HEIGHT as u32
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
                    break 'running
                },
                _ => {},
            }
        }

        engine.update();

        if engine.present_time <= 0.0 {
            engine.draw();

            let _ = screentex.update(None, &engine.rasterizer.color, (AsteroidsEngine::RENDER_WIDTH * 4) as usize);
            let _ = canvas.copy(&screentex, None, None);
            canvas.present();

            engine.present_time = 1.0 / 120.0;
        }
    }
    

    let _error_code = engine.update();
}