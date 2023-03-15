use mlua::prelude::*;
use soloud::prelude::*;

use crate::api_shareables::*;

pub fn register_audio_api(audio: SharedAudio, audio_handles: SharedAudioHandle, assets_sfx: SharedAudioWav, assets_mus: SharedAudioWavStream, lua: &Lua) {
    println!("Registering API: Audio");

    // SFX //
    let sfxa = assets_sfx.clone();
    let fn_load_sound = lua.create_function(move |_, (path_to, name): (String, String)| {
        // Overwrite anything already in the key
        let mut wav = soloud::audio::Wav::default();

        let wav_result = wav.load(&path_to);
        if wav_result.is_err() {
            println!("ERROR - AUDIO: Failed to load Wav at path '{}'! Soloud: {}", path_to, wav_result.err().unwrap());
        }
        sfxa.insert(name, wav);
        Ok(())
    }).unwrap();
    let _ = lua.globals().set("load_sound", fn_load_sound);

    let sfxa = assets_sfx.clone();
    let fn_unload_sound = lua.create_function(move |_, name: String| {
        
        sfxa.remove(&name);
        Ok(())
    }).unwrap();
    let _ = lua.globals().set("unload_sound", fn_unload_sound);

    let soloud = audio.clone();
    let sfxa = assets_sfx.clone();
    let fn_sfx = lua.create_function(move |_, name: String| {
        // Play sound, don't save handle
        let find_result = sfxa.get(&name);
        if find_result.is_some() {
            soloud.play(&*find_result.unwrap());
        }
        Ok(())
    }).unwrap();
    let _ = lua.globals().set("play_sound", fn_sfx);

    let soloud = audio.clone();
    let sfxa = assets_sfx.clone();
    let handles = audio_handles.clone();
    let fn_play_sound_handle = lua.create_function(move |_, (name, handle_name): (String, String)| {
        // Play sound with handle
        let find_result = sfxa.get(&name);
        if find_result.is_some() {
            let handle = soloud.play(&*find_result.unwrap());
            if soloud.is_valid_voice_handle(handle) {
                handles.insert(handle_name, handle);
            }
        }
        Ok(())
    }).unwrap();
    let _ = lua.globals().set("play_sound_handle", fn_play_sound_handle);

    let soloud = audio.clone();
    let handles = audio_handles.clone();
    let fn_stop_sound = lua.create_function(move |_, handle_name: String| {
        let find_result = handles.get(&handle_name);
        if find_result.is_some() {
            let handle_ref = find_result.unwrap();
            if soloud.is_valid_voice_handle(*handle_ref.value()) {
                soloud.stop(*handle_ref);
            }
            
        }
        Ok(())
    }).unwrap();
    let _ = lua.globals().set("stop_sound", fn_stop_sound);

    let soloud = audio.clone();
    let fn_stop_sound_all = lua.create_function(move |_, ()| {
        soloud.stop_all();
        Ok(())
    }).unwrap();
    let _ = lua.globals().set("stop_sound_all", fn_stop_sound_all);

    // MUSIC //
    let musa = assets_mus.clone();
    let fn_load_mus = lua.create_function(move |_, (path_to, name): (String, String)| {
        // Overwrite anything already in the key
        let mut wav = soloud::audio::WavStream::default();

        let wav_result = wav.load(&path_to);
        if wav_result.is_err() {
            println!("ERROR - AUDIO: Failed to load Wav at path '{}'! Soloud: {}", path_to, wav_result.err().unwrap());
        }
        musa.insert(name, wav);
        Ok(())
    }).unwrap();
    let _ = lua.globals().set("load_music", fn_load_mus);

    let musa = assets_mus.clone();
    let fn_unload_sound = lua.create_function(move |_, name: String| {
        musa.remove(&name);
        Ok(())
    }).unwrap();
    let _ = lua.globals().set("unload_music", fn_unload_sound);

    let soloud = audio.clone();
    let musa = assets_mus.clone();
    let fn_mus = lua.create_function(move |_, name: String| {
        // Play sound, don't save handle
        let find_result = musa.get(&name);
        if find_result.is_some() {
            soloud.play(&*find_result.unwrap());
        }
        Ok(())
    }).unwrap();
    let _ = lua.globals().set("play_music", fn_mus);
}