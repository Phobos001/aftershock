use std::rc::Rc;
use std::cell::RefCell;

use dashmap::DashMap;
use soloud::{Soloud, Wav, WavStream};

//use rapier2d_f64::prelude::*;

use aftershock::buffer::Buffer;
use crate::controls::ControlData;
use crate::engine::VideoData;
//use crate::rapier2d_wrap::RapierWorld2D;

pub type SharedVideoData = Rc<RefCell<VideoData>>;
pub type SharedBuffer = Rc<RefCell<Buffer>>;
pub type SharedControlData = Rc<RefCell<ControlData>>;

pub type SharedAudio = Rc<Soloud>;
pub type SharedAudioHandle = Rc<DashMap<String, soloud::Handle>>;
pub type SharedAudioWav = Rc<DashMap<String, Wav>>;
pub type SharedAudioWavStream = Rc<DashMap<String, WavStream>>;

pub type SharedImages = Rc<DashMap<String, Buffer>>;

