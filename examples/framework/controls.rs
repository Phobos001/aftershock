use sdl2::keyboard::Keycode;
use aftershock::vector2::*;

#[derive(Debug, Clone, Copy)]
pub enum MouseButton {
    None,
    Left,
    Right,
    Middle,
    X1,
    X2,
}

#[derive(Debug, Clone, Copy)]
pub struct KeyBind {
    pub keybit: u8,
    pub keycode: Keycode,
    pub mouse_button: MouseButton,
}

pub struct ControlData {
    pub binds: Vec<KeyBind>,
    pub controls: u128,
    pub controls_last: u128,

    pub mouse: Vector2,
    pub mouse_delta: Vector2,
    pub mouse_boundries: Vector2,
}

impl ControlData {
    pub const MOUSE_LEFT: u8    = 123;
    pub const MOUSE_RIGHT: u8   = 124;
    pub const MOUSE_MIDDLE: u8  = 125;
    pub const MOUSE_X1: u8      = 126;
    pub const MOUSE_X2: u8      = 127;

    pub fn new() -> ControlData {
        ControlData {
            binds: Vec::new(),
            controls: 0,
            controls_last: 0,

            mouse: Vector2::ZERO,
            mouse_delta: Vector2::ZERO,
            mouse_boundries: Vector2::new(512.0, 512.0),
        }
    }

    pub fn is_control_down(&self, control: u8) -> bool {
        return self.controls & (1 << control) != 0;
    }

    pub fn is_control_pressed(&self, control: u8) -> bool {
        !(self.controls_last & (1 << control) != 0) && (self.controls & (1 << control) != 0)
    }

    pub fn is_control_released(&self, control: u8) -> bool {
        (self.controls_last & (1 << control) != 0) && !(self.controls & (1 << control) != 0)
    }
    
    pub fn set_key_bind(&mut self, keybit: u8, keycode: Keycode) {
        self.binds.push(KeyBind{ keybit, keycode, mouse_button: MouseButton::None });
    }

    pub fn set_key_bind_from_string(&mut self, keybit: u8, keyname: &str) {
        let keycode_opt = Keycode::from_name(keyname);
        if keycode_opt.is_some() {
            self.binds.push(KeyBind{ keybit, keycode: keycode_opt.unwrap(), mouse_button: MouseButton::None });
        } else {
            println!("ERROR - INPUT: Keycode '{}' not found in SDL enum!", keyname);
        }
        
    }

    pub fn update_mouse_delta(&mut self, xrel: f32, yrel: f32) {
        self.mouse_delta = Vector2::new(xrel, yrel);
    }

    pub fn update_mouse_boundries(&mut self, width: f32, height: f32) {
        self.mouse_boundries.x = width;
        self.mouse_boundries.y = height;
    }

    pub fn update_controls(&mut self, mouse_state: sdl2::mouse::MouseState, keyboard_state: sdl2::keyboard::KeyboardState) {
        let new_mouse_position_dx = mouse_state.x() as f32 - self.mouse.x;
        let new_mouse_position_dy = mouse_state.y() as f32 - self.mouse.y;
        
        self.mouse += Vector2::new(new_mouse_position_dx, new_mouse_position_dy) * 0.5;
        self.mouse.x = self.mouse.x.clamp(0.0, self.mouse_boundries.x);
        self.mouse.y = self.mouse.y.clamp(0.0, self.mouse_boundries.y);

        self.mouse.x = mouse_state.x() as f32;
        self.mouse.y = mouse_state.y() as f32;
        


        let keys: Vec<Keycode> = keyboard_state.pressed_scancodes().filter_map(Keycode::from_scancode).collect();

        self.controls_last = self.controls;
        self.controls = 0;

        if mouse_state.left() {
            self.controls |= 1 << ControlData::MOUSE_LEFT;
        }

        if mouse_state.right() {
            self.controls |= 1 << ControlData::MOUSE_RIGHT;
        }

        if mouse_state.middle() {
            self.controls |= 1 << ControlData::MOUSE_MIDDLE;
        }

        if mouse_state.x1() {
            self.controls |= 1 << ControlData::MOUSE_X1;
        }

        if mouse_state.x2() {
            self.controls |= 1 << ControlData::MOUSE_X2;
        }
        
        for key in keys.iter() {
            for bind in &self.binds {
                if key == &bind.keycode {
                    self.controls |= 1 << bind.keybit;
                }
            }
        }
    }
}