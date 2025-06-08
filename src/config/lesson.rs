use serde::Deserialize;
use std::{fs, path::PathBuf};
use thiserror::Error;

use crate::keyboard_config::PressedKeyCoord;

pub use super::exercise::Exercise;

#[derive(Debug, Default, Clone, Deserialize)]
pub struct LessonPage {
    pub title: String,
    pub content: String,
    #[serde(default)]
    pub show_keys: Vec<PressedKeyCoord>,
    #[serde(default)]
    pub keyboard: bool,
    #[serde(default)]
    pub exercises: Vec<Exercise>,
    #[serde(default)]
    pub content2: String,
}

#[derive(Debug, Default, Clone, Deserialize)]
pub struct Lesson {
    pub pages: Vec<LessonPage>,
}

impl Lesson {
    pub fn load(path: PathBuf) -> Result<Self, Error> {
        let content = fs::read_to_string(path.clone())
            .map_err(|e| Error::Read(path.display().to_string(), e.to_string()))?;
        let lesson: Lesson =
            serde_yaml::from_str(&content).map_err(|e| Error::Parse(e.to_string()))?;
        Ok(lesson)
    }

    pub fn get_page(&self, page_index: usize) -> Option<&LessonPage> {
        self.pages.get(page_index)
    }

    pub fn get_exercise(&self, current_page: usize, current_exercise: usize) -> Option<&Exercise> {
        match self.get_page(current_page) {
            Some(page) => page.exercises.get(current_exercise),
            None => None,
        }
    }
}

#[derive(Debug, Error, Clone)]
pub enum Error {
    #[error("Lessons content could not be read from file {0}. Error: {1}")]
    Read(String, String),
    #[error("{0}")]
    Parse(String),
}
