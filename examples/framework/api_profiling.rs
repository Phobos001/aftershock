use mlua::prelude::*;

pub fn register_profiling_api(lua: &Lua) {
    println!("Registering API: Profiling");
    
    let fn_timestamp = lua.create_function(move |_, ()| {
        Ok(aftershock::timestamp())
    }).unwrap();
    let _ = lua.globals().set("timestamp", fn_timestamp);
}