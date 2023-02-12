extern crate sdl2;

use aftershock::buffer::*;
use aftershock::color::*;

use aftershock::timestamp;
use sdl2::event::Event;
use sdl2::pixels::{PixelFormatEnum};

use std::thread::*;

pub const RENDER_WIDTH: u32 = 960;
pub const RENDER_HEIGHT: u32 = 540;

pub struct Benchmark {
    pub screen: Buffer,
    pub stage: u8,
    pub tics: u32,
    pub dt: f64,
    pub benchmark_time_before: f64,
}

impl Benchmark {
    pub fn new() -> Benchmark {
        Benchmark { screen: Buffer::new(RENDER_WIDTH as usize, RENDER_HEIGHT as usize), stage: 0, tics: 0, dt: 0.0, benchmark_time_before: timestamp() }
    }

    pub fn update(&mut self, dt: f64) -> bool {
        match self.stage {
            0 => { self.prectangle_outline(); false},
            1 => { self.pcircle_outline(); false },
            2 => { self.ptriangle_outline(); false },
            3 => { self.prectangle_filled(); false},
            4 => { self.pcircle_filled(); false },
            5 => { self.ptriangle_filled(); false },
            _ => {true}
        }
    }

    pub fn prectangle_outline(&mut self) {
        self.screen.clear();
        self.screen.prectangle(false, 0, 0, RENDER_WIDTH as i32, RENDER_HEIGHT as i32, Color::hsv(self.tics as f32, 1.0, 1.0));
        self.tics += 1;
    
        if self.tics > 1000 {
            self.tics = 0;
            self.stage += 1;
    
            let profile_prect_time_after = timestamp();
    
            let prect_time = profile_prect_time_after - self.benchmark_time_before;
            println!("prectangle_outline time: {}s", prect_time);
            self.benchmark_time_before = profile_prect_time_after;
        }
    }

    pub fn prectangle_filled(&mut self) {
        self.screen.clear();
        self.screen.prectangle(true, 0, 0, RENDER_WIDTH as i32, RENDER_HEIGHT as i32, Color::hsv(self.tics as f32, 1.0, 1.0));
        self.tics += 1;
    
        if self.tics > 1000 {
            self.tics = 0;
            self.stage += 1;
    
            let profile_prect_time_after = timestamp();
    
            let prect_time = profile_prect_time_after - self.benchmark_time_before;
            println!("prectangle time: {}s", prect_time);
            self.benchmark_time_before = profile_prect_time_after;
        }
    }

    pub fn pcircle_outline(&mut self) {
        self.screen.clear();
        self.screen.pcircle(false, RENDER_WIDTH as i32 / 2, RENDER_HEIGHT as i32 / 2, RENDER_WIDTH as i32 * 2, Color::hsv(self.tics as f32, 1.0, 1.0));
        self.tics += 1;
    
        if self.tics > 1000 {
            self.tics = 0;
            self.stage += 1;
    
            let profile_prect_time_after = timestamp();
    
            let prect_time = profile_prect_time_after - self.benchmark_time_before;
            println!("pcircle_outline time: {}s", prect_time);
            self.benchmark_time_before = profile_prect_time_after;
        }
    }

    pub fn pcircle_filled(&mut self) {
        self.screen.clear();
        self.screen.pcircle(true, RENDER_WIDTH as i32 / 2, RENDER_HEIGHT as i32 / 2, RENDER_WIDTH as i32 * 2, Color::hsv(self.tics as f32, 1.0, 1.0));
        self.tics += 1;
    
        if self.tics > 1000 {
            self.tics = 0;
            self.stage += 1;
    
            let profile_prect_time_after = timestamp();
    
            let prect_time = profile_prect_time_after - self.benchmark_time_before;
            println!("pcircle time: {}s", prect_time);
            self.benchmark_time_before = profile_prect_time_after;
        }
    }

    pub fn ptriangle_outline(&mut self) {
        self.screen.clear();
        self.screen.ptriangle(false, 0, RENDER_HEIGHT as i32, RENDER_WIDTH as i32, 0, RENDER_WIDTH as i32, RENDER_HEIGHT as i32, Color::hsv(self.tics as f32, 1.0, 1.0));
        self.screen.ptriangle(false, 0, 0, RENDER_WIDTH as i32, 0, 0, RENDER_HEIGHT as i32, Color::hsv(self.tics as f32, 1.0, 1.0));
        self.tics += 1;
    
        if self.tics > 1000 {
            self.tics = 0;
            self.stage += 1;
    
            let profile_prect_time_after = timestamp();
    
            let prect_time = profile_prect_time_after - self.benchmark_time_before;
            println!("ptriangle_outline time: {}s", prect_time);
            self.benchmark_time_before = profile_prect_time_after;
        }
    }

    pub fn ptriangle_filled(&mut self) {
        self.screen.clear();
        self.screen.ptriangle(true, 0, RENDER_HEIGHT as i32, RENDER_WIDTH as i32, 0, RENDER_WIDTH as i32, RENDER_HEIGHT as i32, Color::hsv(self.tics as f32, 1.0, 1.0));
        self.screen.ptriangle(true, 0, 0, RENDER_WIDTH as i32, 0, 0, RENDER_HEIGHT as i32, Color::hsv(self.tics as f32, 1.0, 1.0));
        self.tics += 1;
    
        if self.tics > 1000 {
            self.tics = 0;
            self.stage += 1;
    
            let profile_prect_time_after = timestamp();
    
            let prect_time = profile_prect_time_after - self.benchmark_time_before;
            println!("ptriangle time: {}s", prect_time);
            self.benchmark_time_before = profile_prect_time_after;
        }
    }
}

pub fn main() {

    // Init SDL and surface texture
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let title = "Benchmark";
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

    let mut present_timer: f64 = 0.0;

    let mut delta_now: f64 = aftershock::timestamp();
    let mut delta_last: f64;

    let mut realtime: f64 = 0.0;

    println!("Benchmark Test");

    let mut benchmark: Benchmark = Benchmark::new();
    

    'running: loop {

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => {
                    break 'running
                },
                _ => {},
            }
        }

        delta_last = delta_now;
        delta_now = aftershock::timestamp();


        let dt: f64 = delta_now - delta_last;
        realtime += dt;
        

        present_timer -= dt;

        canvas.clear();

        if benchmark.update(dt) {
            return;
        }

        if present_timer <= 0.0 {
            let _ = screentex.update(None, &benchmark.screen.color, (RENDER_WIDTH * 4) as usize);
            let _ = canvas.copy(&screentex, None, None);
            canvas.present();

            present_timer = 1.0 / 60.0;
        }

        std::thread::sleep(std::time::Duration::from_micros(1));
    }
}