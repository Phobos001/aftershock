extern crate device_query;
use device_query::*;

#[derive(Debug, Copy, Clone)]
pub enum ControlKeys {
    MoveLeft = 0,
    MoveRight = 1,
    Jump = 2,
    Crouch = 3,
    AimUp = 4,
    AimDown = 5,
    AimLeft = 6,
    AimRight = 7,
    Inventory = 8,

    Pause = 28,
    MouseButtonLeft = 29,
    MouseButtonRight = 30,
    MouseButtonMiddle = 31,
}

pub struct Controls {
    pub input_current: u32,
    pub input_last: u32,
    pub device_state: device_query::DeviceState,
}

impl Controls {
    pub fn new() -> Controls {
        Controls {
            input_current: 0,
            input_last: 0,
            device_state: device_query::DeviceState::new(),
        }
    }

    pub fn update(&mut self) {
        let keys: Vec<Keycode> = self.device_state.query_keymap();
        let mouse: Vec<bool> = self.device_state.query_pointer().button_pressed;

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
                Keycode::Left   => { self.input_current  |= 1 << ControlKeys::AimLeft as u32; },
                Keycode::Right  => { self.input_current  |= 1 << ControlKeys::AimRight as u32; },
                Keycode::Up     => { self.input_current  |= 1 << ControlKeys::AimUp as u32; },
                Keycode::Down   => { self.input_current  |= 1 << ControlKeys::AimDown as u32; },

                Keycode::A      => { self.input_current  |= 1 << ControlKeys::MoveLeft as u32; },
                Keycode::D      => { self.input_current  |= 1 << ControlKeys::MoveRight as u32; },
                Keycode::W      => { self.input_current  |= 1 << ControlKeys::Jump as u32; },
                Keycode::S      => { self.input_current  |= 1 << ControlKeys::Crouch as u32; },

                Keycode::I      => { self.input_current  |= 1 << ControlKeys::Inventory as u32; }
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