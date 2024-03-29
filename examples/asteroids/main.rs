extern crate sdl2;

mod asteroids;

use asteroids::*;

pub enum Sdl2VideoMode {
    Windowed,
    Fullscreen,
    Exclusive,
}

pub fn main() {
    let mut engine = AsteroidsEngine::new();

    start_sdl2(&mut engine);

}

pub fn start_sdl2(engine: &mut AsteroidsEngine) {
    use sdl2::event::Event;
    use sdl2::pixels::{PixelFormatEnum};
    
    let mut hardware_accelerated: bool = false;
    let mut integer_scaling: bool = true;
    let mut video_mode: Sdl2VideoMode = Sdl2VideoMode::Windowed;

    let args: Vec<_> = std::env::args().collect();
    for arg in args {
        match arg.as_str() {
            "--exclusive" => { video_mode = Sdl2VideoMode::Exclusive; },
            "--fullscreen" => { video_mode = Sdl2VideoMode::Fullscreen; },
            "--windowed" => { video_mode = Sdl2VideoMode::Windowed; },
            "--hardware-canvas" => { hardware_accelerated = true; }
            "--no-integer-scale" => { integer_scaling = false; }
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
    let _ = canvas.set_integer_scale(integer_scaling);
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
            canvas.clear();
            engine.draw();

            let _ = screentex.update(None, &engine.screen.color, (AsteroidsEngine::RENDER_WIDTH * 4) as usize);
            let _ = canvas.copy(&screentex, None, None);
            canvas.present();

            engine.present_time = 1.0 / if hardware_accelerated { 240.0 } else { 120.0 };
        }
    }
    

    let _error_code = engine.update();
}