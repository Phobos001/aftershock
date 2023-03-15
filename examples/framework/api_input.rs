use crate::{api_shareables::*, controls::ControlData};
use mlua::prelude::*;

pub fn register_input_api(control_data: SharedControlData, lua: &Lua) {
    println!("Registering API: Input KB/M");

    let input = control_data.clone();
    let fn_mouse_x = lua.create_function( move |_, ()| {
        Ok(input.borrow().mouse.x)
    }).unwrap();
    let _ = lua.globals().set("mouse_x", fn_mouse_x);

    let input = control_data.clone();
    let fn_mouse_y = lua.create_function( move |_, ()| {
        Ok(input.borrow().mouse.y)
    }).unwrap();
    let _ = lua.globals().set("mouse_y", fn_mouse_y);

    // Keys

    let input = control_data.clone();
    let fn_set_key_bind = lua.create_function( move |_, (control_number, key_name): (u8, String)| {
        input.borrow_mut().set_key_bind_from_string(control_number, &key_name);
        Ok(())
    }).unwrap();
    let _ = lua.globals().set("set_key_bind", fn_set_key_bind);

    let input = control_data.clone();
    let fn_is_control_down = lua.create_function( move |_, control: u8| {
        Ok(input.borrow().is_control_down(control))
    }).unwrap();
    let _ = lua.globals().set("is_control_down", fn_is_control_down);

    let input = control_data.clone();
    let fn_is_control_pressed = lua.create_function( move |_, control: u8| {
        Ok(input.borrow().is_control_pressed(control))
    }).unwrap();
    let _ = lua.globals().set("is_control_pressed", fn_is_control_pressed);

    let input = control_data.clone();
    let fn_is_control_released = lua.create_function( move |_, control: u8| {
        Ok(input.borrow().is_control_released(control))
    }).unwrap();
    let _ = lua.globals().set("is_control_released", fn_is_control_released);

    let input = control_data.clone();
    let fn_is_mouse_button_down = lua.create_function( move |_, button: u8| {
        let mouse_button = match button {
            0 => { input.borrow().is_control_down(ControlData::MOUSE_LEFT) },
            1 => { input.borrow().is_control_down(ControlData::MOUSE_RIGHT) },
            2 => { input.borrow().is_control_down(ControlData::MOUSE_MIDDLE) },
            3 => { input.borrow().is_control_down(ControlData::MOUSE_X1) },
            4 => { input.borrow().is_control_down(ControlData::MOUSE_X2) },
            _ => { false }
        };
        Ok(mouse_button)
    }).unwrap();
    let _ = lua.globals().set("is_mouse_button_down", fn_is_mouse_button_down);

    let input = control_data.clone();
    let fn_is_mouse_button_pressed = lua.create_function( move |_, button: u8| {
        let mouse_button = match button {
            0 => { input.borrow().is_control_pressed(ControlData::MOUSE_LEFT) },
            1 => { input.borrow().is_control_pressed(ControlData::MOUSE_RIGHT) },
            2 => { input.borrow().is_control_pressed(ControlData::MOUSE_MIDDLE) },
            3 => { input.borrow().is_control_pressed(ControlData::MOUSE_X1) },
            4 => { input.borrow().is_control_pressed(ControlData::MOUSE_X2) },
            _ => { false }
        };
        Ok(mouse_button)
    }).unwrap();
    let _ = lua.globals().set("is_mouse_button_pressed", fn_is_mouse_button_pressed);

    let input = control_data.clone();
    let fn_is_mouse_button_released = lua.create_function( move |_, button: u8| {
        let mouse_button = match button {
            0 => { input.borrow().is_control_released(ControlData::MOUSE_LEFT) },
            1 => { input.borrow().is_control_released(ControlData::MOUSE_RIGHT) },
            2 => { input.borrow().is_control_released(ControlData::MOUSE_MIDDLE) },
            3 => { input.borrow().is_control_released(ControlData::MOUSE_X1) },
            4 => { input.borrow().is_control_released(ControlData::MOUSE_X2) },
            _ => { false }
        };
        Ok(mouse_button)
    }).unwrap();
    let _ = lua.globals().set("is_mouse_button_released", fn_is_mouse_button_released);
}