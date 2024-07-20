use serde::Deserialize;
use std::{fs, path::PathBuf};
use thiserror::Error;

use super::keyboard::PressedKeyCoord;

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub enum LessonAction {
    ShowKey(PressedKeyCoord),
    AutoCorrectLine(String),
}

#[derive(Debug, Default, Clone, Deserialize)]
pub struct LessonPage {
    pub title: String,
    pub content: String,
    // pub actions: Vec<LessonAction>,
    #[serde(default = "empty_string")]
    pub content2: String,
}

fn empty_string() -> String {
    "".to_string()
}

#[derive(Debug, Default, Clone, Deserialize)]
pub struct Lesson {
    pub _title: String,
    pub pages: Vec<LessonPage>,
}

impl Lesson {
    pub fn load(path: PathBuf) -> Result<Self, Error> {
        let content = fs::read_to_string(path).map_err(|e| Error::Read(e.to_string()))?;
        let lesson: Lesson =
            serde_yaml::from_str(&content).map_err(|e| Error::Parse(e.to_string()))?;
        Ok(lesson)
    }
}

#[derive(Debug, Error, Clone)]
pub enum Error {
    #[error("Lessons content could not be read: {0}")]
    Read(String),
    #[error("{0}")]
    Parse(String),
}
