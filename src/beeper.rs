use std::io::Cursor;

use rodio::{Decoder, DeviceSinkBuilder, MixerDeviceSink, Player};

pub struct Beeper {
    /// Missing when no audio device could be opened, which leaves the exercises
    /// silent instead of failing. Machines without a sound device, a headless
    /// test run among them, are such a case.
    stream: Option<MixerDeviceSink>,

    // Store two sounds now
    beep: Vec<u8>,
}

impl Beeper {
    pub fn new() -> Self {
        let stream = match DeviceSinkBuilder::open_default_sink() {
            Ok(mut stream) => {
                stream.log_on_drop(false);
                Some(stream)
            }
            Err(e) => {
                eprintln!("No audio device, exercises stay silent: {e}");
                None
            }
        };

        let beep = include_bytes!("../sounds/clack.mp3").to_vec();

        Self { stream, beep }
    }

    /// Helper to play raw data
    fn play(&self, data: &[u8]) {
        let Some(stream) = &self.stream else {
            return;
        };
        let player = Player::connect_new(stream.mixer());
        let cursor = Cursor::new(data.to_vec()); // Clone the data for playback
        if let Ok(source) = Decoder::new(cursor) {
            player.append(source);
            player.detach();
        }
    }

    pub fn play_beep(&self) {
        self.play(&self.beep);
    }
}
