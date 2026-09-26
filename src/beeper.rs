use std::io::Cursor;

use rodio::{Decoder, DeviceSinkBuilder, MixerDeviceSink, Player};

pub struct Beeper {
    stream: MixerDeviceSink,

    // Store two sounds now
    beep: Vec<u8>,
}

impl Beeper {
    pub fn new() -> Self {
        let mut stream = DeviceSinkBuilder::open_default_sink().unwrap();
        stream.log_on_drop(false);

        let beep = include_bytes!("../sounds/clack.mp3").to_vec();

        Self { stream, beep }
    }

    /// Helper to play raw data
    fn play(&self, data: &[u8]) {
        let player = Player::connect_new(self.stream.mixer());
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
