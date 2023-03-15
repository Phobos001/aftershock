use aftershock::{color::Color, buffer::{Buffer, DrawMode}};

use mlua::prelude::*;

use crate::api_color::*;
use crate::api_shareables::*;

pub fn register_image(assets_images: SharedImages, lua: &Lua) {
    println!("Registering API: Images");

    let imgs = assets_images.clone();
    let fn_image_new = lua.create_function(move |_, (name, width, height): (String, f32, f32)| {
        imgs.insert(name, Buffer::new(width as usize, height as usize));
        Ok(())
        
    }).unwrap();
    let _ = lua.globals().set("image", fn_image_new);

    let imgs = assets_images.clone();
    let fn_image_load = lua.create_function(move |_, (name, path_to): (String, String)| {
        let image_result = Buffer::new_from_image(&path_to);
        if image_result.is_ok() {
            imgs.insert(name, image_result.unwrap());
            Ok(())
        } else { /* Handled by Image */ Ok(()) }
        
    }).unwrap();
    let _ = lua.globals().set("load_image", fn_image_load);

    // Draw Mode: No Operation //
    let imgs = assets_images.clone();
    let fn_set_image_draw_mode_noop = lua.create_function(move |_, name: String| {
        let img_result = imgs.get_mut(&name);
        if img_result.is_some() {
            img_result.unwrap().set_draw_mode(DrawMode::NoOp);
        }
        Ok(())
    } ).unwrap();
    let _ = lua.globals().set("set_image_draw_mode_noop", fn_set_image_draw_mode_noop);

    // Draw Mode: Opaque //
    let imgs = assets_images.clone();
    let fn_set_image_draw_mode_opaque = lua.create_function(move |_, name: String| {
        let img_result = imgs.get_mut(&name);
        if img_result.is_some() {
            img_result.unwrap().set_draw_mode(DrawMode::Opaque);
        }
        Ok(())
    } ).unwrap();
    let _ = lua.globals().set("set_image_draw_mode_opaque", fn_set_image_draw_mode_opaque);

    // Draw Mode: Alpha //
    let imgs = assets_images.clone();
    let fn_set_image_draw_mode_alpha = lua.create_function(move |_, name: String| {
        let img_result = imgs.get_mut(&name);
        if img_result.is_some() {
            img_result.unwrap().set_draw_mode(DrawMode::Alpha);
        }
        Ok(())
    } ).unwrap();
    let _ = lua.globals().set("set_image_draw_mode_alpha", fn_set_image_draw_mode_alpha);

    // Draw Mode: Addition //
    let imgs = assets_images.clone();
    let fn_set_image_draw_mode_addition = lua.create_function(move |_, name: String| {
        let img_result = imgs.get_mut(&name);
        if img_result.is_some() {
            img_result.unwrap().set_draw_mode(DrawMode::Addition);
        }
        Ok(())
    } ).unwrap();
    let _ = lua.globals().set("set_image_draw_mode_addition", fn_set_image_draw_mode_addition);

    // Draw Mode: Subtract //
    let imgs = assets_images.clone();
    let fn_set_image_draw_mode_subtraction = lua.create_function(move |_, name: String| {
        let img_result = imgs.get_mut(&name);
        if img_result.is_some() {
            img_result.unwrap().set_draw_mode(DrawMode::Subtraction);
        }
        Ok(())
    } ).unwrap();
    let _ = lua.globals().set("set_image_draw_mode_subtraction", fn_set_image_draw_mode_subtraction);

    // Draw Mode: Multiply //
    let imgs = assets_images.clone();
    let fn_set_image_draw_mode_multiply = lua.create_function(move |_, name: String| {
        let img_result = imgs.get_mut(&name);
        if img_result.is_some() {
            img_result.unwrap().set_draw_mode(DrawMode::Multiply);
        }
        Ok(())
    } ).unwrap();
    let _ = lua.globals().set("set_image_draw_mode_multiply", fn_set_image_draw_mode_multiply);

    // pset image //
    let imgs = assets_images.clone();
    let fn_iset = lua.create_function(move |_, (name, x, y, color): (String, i32, i32, LuaColor)| {
        let img_result = imgs.get_mut(&name);
        if img_result.is_some() {
            img_result.unwrap().pset(x, y, color.color);
        }
        Ok(())
    } ).unwrap();
    let _ = lua.globals().set("iset", fn_iset);

    // prectangle image //
    let imgs = assets_images.clone();
    let fn_irectangle = lua.create_function(move |_, (name, filled, x, y, width, height, color): (String, bool, i32, i32, i32, i32, LuaColor)| {
        let img_result = imgs.get_mut(&name);
        if img_result.is_some() {
            img_result.unwrap().prectangle(filled, x, y, width, height, color.color);
        }
        Ok(())
    } ).unwrap();
    let _ = lua.globals().set("irectangle", fn_irectangle);

    // pcircle image //
    let imgs = assets_images.clone();
    let fn_icircle = lua.create_function(move |_, (name, filled, xc, yc, radius, color): (String, bool, i32, i32, i32, LuaColor)| {
        let img_result = imgs.get_mut(&name);
        if img_result.is_some() {
            img_result.unwrap().pcircle(filled, xc, yc, radius, color.color);
        }
        Ok(())
    } ).unwrap();
    let _ = lua.globals().set("icircle", fn_icircle);
}