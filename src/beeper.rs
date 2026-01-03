use std::io::Cursor;

use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};

pub struct Beeper {
    _stream: OutputStream,
    stream_handle: OutputStreamHandle,

    // Store two sounds now
    beep: Vec<u8>,
}

impl Beeper {
    pub fn new() -> Self {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();

        let beep = include_bytes!("../sounds/clack.mp3").to_vec();

        Self {
            _stream,
            stream_handle,
            beep,
        }
    }

    /// Helper to play raw data
    fn play(&self, data: &[u8]) {
        if let Ok(sink) = Sink::try_new(&self.stream_handle) {
            let cursor = Cursor::new(data.to_vec()); // Clone the data for playback
            if let Ok(source) = Decoder::new(cursor) {
                sink.append(source);
                sink.detach();
            }
        }
    }

    pub fn play_beep(&self) {
        self.play(&self.beep);
    }
}
