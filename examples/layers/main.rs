extern crate sdl2;

use aftershock::buffer::*;
use aftershock::color::*;

use aftershock::timestamp;
use sdl2::event::Event;
use sdl2::pixels::{PixelFormatEnum};

use std::thread::*;

pub const RENDER_WIDTH: u32 = 512;
pub const RENDER_HEIGHT: u32 = 512;

pub fn main() {

    // Init SDL and surface texture
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let title = "Layers";
    let window = video_subsystem
        .window(title, RENDER_WIDTH, RENDER_HEIGHT)
        .resizable()
        .position_centered()
        .build()
        .unwrap()
    ;

    let mut canvas = window.into_canvas().software().present_vsync().build().map_err(|e| e.to_string()).unwrap();

    let _ = canvas.set_logical_size(RENDER_WIDTH, RENDER_HEIGHT);
    let _ = canvas.set_integer_scale(true);
    let texture_creator = canvas.texture_creator();

    // This is what we update our buffers to
    let mut screentex = texture_creator.create_texture_streaming(PixelFormatEnum::RGBA32, RENDER_WIDTH, RENDER_HEIGHT)
        .map_err(|e| e.to_string()).unwrap();

    canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().map_err(|e| e.to_string()).unwrap();

    // Faded light sprite for buffer4
    let mut light_sprite: Buffer = Buffer::new(256, 256);
    {
        light_sprite.set_draw_mode(DrawMode::Alpha);
        
        for step in 0..128 {
            light_sprite.opacity = 3;
            light_sprite.pcircle(true, 128, 128, step as i32, Color::WHITE)
        }

        light_sprite.set_draw_mode(DrawMode::Opaque);
    }

    let bake_time_before: f64 = timestamp();

    // Screen buffer
    let mut buffer0: Buffer = Buffer::new(RENDER_WIDTH as usize, RENDER_HEIGHT as usize);

    // Checkerboard Pattern
    let mut buffer1: Buffer = Buffer::new(RENDER_WIDTH as usize, RENDER_HEIGHT as usize);

    // Small checkerboard pattern
    let mut buffer2: Buffer = Buffer::new(RENDER_WIDTH as usize, RENDER_HEIGHT as usize);

    // Circles
    let mut buffer3: Buffer = Buffer::new(RENDER_WIDTH as usize, RENDER_HEIGHT as usize);

    // Lights
    let mut buffer4: Buffer = Buffer::new(RENDER_WIDTH as usize, RENDER_HEIGHT as usize);

    

    let mut opacity: u8 = 0;

    // Prebake buffers using multiple threads
    // Focus on composition performance
    scope(|s| {
        // Draw a checkerboard on thread 1

        let buffer1_ref: &mut Buffer = &mut buffer1;
        let _ = s.spawn(move || {
            
            for iy in 0..16 {
                for ix in 0..16 {
                    let color: Color = if iy % 2 == 0 { 
                        if ix % 2 == 1 { 
                            Color::WHITE
                        } else {
                            Color::BLACK
                        }
                    } else { 
                        if ix % 2 == 0 { 
                            Color::WHITE
                        } else {
                            Color::BLACK
                        }
                    };
                    

                    buffer1_ref.prectangle(true, ix * 32, iy * 32, 32, 32, color);
                }
            }
            
        });

        // Draw smaller checkerboard on thread 2
        let buffer2_ref: &mut Buffer = &mut buffer2;
        let _ = s.spawn(move || {
            for iy in 0..32 {
                for ix in 0..32 {
                    let color: Color = if iy % 2 == 0 { 
                        if ix % 2 == 1 { 
                            Color::RED
                        } else {
                            Color::BLUE
                        }
                    } else { 
                        if ix % 2 == 0 { 
                            Color::RED
                        } else {
                            Color::BLUE
                        }
                    };
                    

                    buffer2_ref.prectangle(true, ix * 16, iy * 16, 16, 16, color);
                }
            }
            
        });

        // Draw opaque circles everywhere on thread 3
        let buffer3_ref: &mut Buffer = &mut buffer3;
        let _ = s.spawn(move || {
            for _ in 0..64 {
                buffer3_ref.pcircle(true, 
                    (alea::f32() * RENDER_WIDTH as f32) as i32, 
                    (alea::f32() * RENDER_HEIGHT as f32) as i32,
                        alea::i32_in_range(0, 16),
                    Color::hsv(alea::f32() * 360.0, 1.0, 1.0))
            }
            
        });

        // Pre-bake lights on buffer4
        let buffer4_ref: &mut Buffer = &mut buffer4;
        let _ = s.spawn(move || {
            
            buffer4_ref.set_draw_mode(DrawMode::Addition);

            buffer4_ref.opacity = 255;
            buffer4_ref.pimgmtx(&light_sprite, 256.0, 256.0, 0.0, 1.5, 1.5, 0.5, 0.5);
            buffer4_ref.opacity = 255;

            buffer4_ref.tint = Color::RED;
            buffer4_ref.pimgmtx(&light_sprite, 128.0, 128.0, 0.0, 0.75, 0.75, 0.5, 0.5);
            buffer4_ref.tint = Color::GREEN;
            buffer4_ref.pimgmtx(&light_sprite, 384.0, 128.0, 0.0, 0.75, 0.75, 0.5, 0.5);
            buffer4_ref.tint = Color::BLUE;
            buffer4_ref.pimgmtx(&light_sprite, 384.0, 384.0, 0.0, 0.75, 0.75, 0.5, 0.5);

            buffer4_ref.set_draw_mode(DrawMode::Opaque);

        });
    });

    let bake_time_after: f64 = timestamp();

    println!("Buffer Bake Time: {} seconds!", bake_time_after - bake_time_before);

    'running: loop {

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => {
                    break 'running
                },
                _ => {},
            }
        }

        let time_before = timestamp();

        buffer0.clear();
        
        // Copy buffer1 (Background) to the screen
        buffer0.blit(&buffer1, 0, 0);

        // Composite buffer 2 over buffer 0 with transparency
        buffer0.pcomposite_alpha(&buffer2, opacity);

        // Composite buffer 3 over buffer 0 with opaque cutting (Sprites Etc)
        buffer0.pcomposite_opaque(&buffer3);

        // Composite buffer 4 over buffer 0 with multiply (Lighting effects)
        buffer0.pcomposite_multiply(&buffer4);

        let time_after = timestamp();
        println!("{}s", time_after - time_before);

        canvas.clear();
        let _ = screentex.update(None, &buffer0.color, (RENDER_WIDTH * 4) as usize);
        let _ = canvas.copy(&screentex, None, None);
        canvas.present();

        opacity = (opacity + 1) % 255;

        std::thread::sleep(std::time::Duration::from_secs_f32(1.0 / 120.0));
    }
}
