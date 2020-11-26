// Built off of Rusty-Audio as a template

use rodio::{
    source::{Buffered, Source},
    Decoder, Sink,
};

use std::collections::HashMap;
use std::fs::File;
use std::io::{Cursor, Read};


pub struct Audio {
	_stream: rodio::OutputStream,
	_stream_handle: rodio::OutputStreamHandle,
    clips: HashMap<&'static str, Buffered<Decoder<Cursor<Vec<u8>>>>>,
    channels: Vec<Sink>,
    current_channel: usize,
}

impl Audio {
    pub fn new(max_audio_channels: u8) -> Self {
        let (_stream, _stream_handle) = rodio::OutputStream::try_default().unwrap();
        let clips = HashMap::new();
        let mut channels: Vec<Sink> = Vec::new();
        for _ in 0..max_audio_channels {
            channels.push(Sink::try_new(&_stream_handle).unwrap());
        }
        Self {
			_stream,
			_stream_handle,
            clips,
            channels,
            current_channel: 0,
        }
	}
	
    pub fn add(&mut self, name: &'static str, path: &str) {
        let mut file_vec: Vec<u8> = Vec::new();
        File::open(path)
            .expect("ERROR - AUDIO: Couldn't find audio file to add.")
            .read_to_end(&mut file_vec)
            .expect("ERROR - AUDIO: Failed reading in opened audio file.");
        let cursor = Cursor::new(file_vec);
        let decoder = Decoder::new(cursor).unwrap();
        let buffered = decoder.buffered();
		
		// Pre-warms audio to stop lazy decoding, which can produce static or strange sound. This is not required so maybe make optional down the road (--lazy-audio)
        let warm = buffered.clone();
        for i in warm {
            #[allow(clippy::drop_copy)]
            drop(i);
        }
        self.clips.insert(name, buffered);
    }
    /// Play an audio clip that has already been loaded.  `name` is the name you chose when you
    /// added the clip to the `Audio` system. If you forgot to load the clip first, this will crash.
    pub fn play_oneshot(&mut self, name: &str) {
        let buffer = self.clips.get(name).expect("WARNING - AUDIO: No clip by that name.").clone();
        self.channels[self.current_channel].append(buffer);
        self.current_channel += 1;
        if self.current_channel >= self.channels.len() {
            self.current_channel = 0;
        }
    }
}