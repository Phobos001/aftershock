extern crate device_query;
use device_query::*;

use crate::engine::RaycastEngine;

#[derive(Debug, Copy, Clone)]
pub enum ControlKeys {
    TurnLeft = 0,
    TurnRight = 1,
    MoveForward = 2,
    MoveBackward = 3,
    StrafeLeft = 4,
    StrafeRight = 5,

    Pause = 28,
    MouseButtonLeft = 29,
    MouseButtonRight = 30,
    MouseButtonMiddle = 31,
}

pub struct Controls {
    pub input_current: u32,
    pub input_last: u32,
    pub device_state: device_query::DeviceState,

    pub mouse_position: (i32, i32),
}

impl Controls {
    pub fn new() -> Controls {
        Controls {
            input_current: 0,
            input_last: 0,
            device_state: device_query::DeviceState::new(),

            mouse_position: (RaycastEngine::RENDER_WIDTH as i32 / 2, RaycastEngine::RENDER_HEIGHT as i32 / 2),
        }
    }

    pub fn update(&mut self) {
        let keys: Vec<Keycode> = self.device_state.query_keymap();
        let mouse: Vec<bool> = self.device_state.query_pointer().button_pressed;

        let new_mouse_position: (i32, i32) = self.device_state.query_pointer().coords;

        self.mouse_position.0 += new_mouse_position.0 - self.mouse_position.0;
        self.mouse_position.1 += new_mouse_position.1 - self.mouse_position.1;


        self.input_last = self.input_current;
        self.input_current = 0;

        if mouse[0] {
            self.input_current |= 1 << ControlKeys::MouseButtonLeft as u32;
        }

        if mouse[1] {
            self.input_current |= 1 << ControlKeys::MouseButtonRight as u32;
        }

        if mouse[2] {
            self.input_current |= 1 << ControlKeys::MouseButtonMiddle as u32;
        }
        
        for key in keys.iter() {
            match key {

                Keycode::A      => { self.input_current  |= 1 << ControlKeys::StrafeLeft as u32; },
                Keycode::D      => { self.input_current  |= 1 << ControlKeys::StrafeRight as u32; },
                Keycode::W      => { self.input_current  |= 1 << ControlKeys::MoveForward as u32; },
                Keycode::S      => { self.input_current  |= 1 << ControlKeys::MoveBackward as u32; },

                Keycode::Left   => { self.input_current  |= 1 << ControlKeys::TurnLeft as u32; },
                Keycode::Right  => { self.input_current  |= 1 << ControlKeys::TurnRight as u32; },

                Keycode::Escape => { self.input_current  |= 1 << ControlKeys::Pause as u32; }
                _ => {},
            }
        }
    }

    pub fn is_control_down(&self, control: ControlKeys) -> bool {
        return self.input_current & (1 << control as u32) != 0;
    }

    pub fn is_control_pressed(&self, control: ControlKeys) -> bool {
        !(self.input_current & (1 << control as u32) != 0) && (self.input_current & (1 << control as u32) != 0)
    }

    pub fn is_control_released(&self, control: ControlKeys) -> bool {
        (self.input_current & (1 << control as u32) != 0) && !(self.input_current & (1 << control as u32) != 0)
    }
}