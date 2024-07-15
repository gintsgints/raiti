mod keyboard;
mod lesson;

use keyboard::Keyboard;
use lesson::Lesson;
use crate::Result;


#[derive(Debug, Clone, Default)]
pub struct Config {
    keyboard: Keyboard,
    lesson: Lesson,
}

impl Config {
    pub fn load() -> Result<Self> {
        let keyboard = Keyboard::load()?;
        let lesson = Lesson::load()?;

        Ok(Config { keyboard, lesson })
    }
}
