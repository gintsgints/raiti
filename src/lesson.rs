use serde::Deserialize;

use crate::{config::PressedKeyCoord, error::LoadError};

#[derive(Debug, Clone, Deserialize)]
pub enum LessonCommands {
    ShowKey(PressedKeyCoord),
    WaitForEnter(String),
}

#[derive(Debug, Default, Clone, Deserialize)]
pub struct Lesson {
    name: String,
    lesson_commands: Vec<LessonCommands>,
}

impl Lesson {
    pub async fn load(path: &str) -> Result<Vec<Lesson>, LoadError> {
        use async_std::prelude::*;

        let mut contents = String::new();

        let mut file = async_std::fs::File::open(path)
            .await
            .map_err(|_| LoadError::File)?;

        file.read_to_string(&mut contents)
            .await
            .map_err(|_| LoadError::File)?;

        serde_yaml::from_str(&contents).map_err(|msg| LoadError::Format(format!("Lesson format {:?}", msg)))
    }
}
