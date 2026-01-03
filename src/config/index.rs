use serde::Deserialize;
use std::{fs, path::PathBuf};
use thiserror::Error;

#[derive(Debug, Default, Clone, Deserialize, PartialEq)]
pub struct IndexRecord {
    pub file: String,
    pub title: String,
}

#[derive(Debug, Default, Clone, Deserialize)]
pub struct Index {
    pub lessons: Vec<IndexRecord>,
}

impl Index {
    pub fn load(path: PathBuf) -> Result<Self, Error> {
        let content = fs::read_to_string(path).map_err(|e| Error::Read(e.to_string()))?;
        let lesson: Index =
            serde_yaml::from_str(&content).map_err(|e| Error::Parse(e.to_string()))?;
        Ok(lesson)
    }

    pub fn next_lesson(&self, current_lesson: &str) -> Option<&str> {
        if let Some(index) = self
            .lessons
            .iter()
            .position(|index_record| index_record.file.eq(current_lesson))
        {
            self.lessons
                .get(index + 1)
                .map(|index_record| index_record.file.as_str())
        } else {
            None
        }
    }
}

#[derive(Debug, Error, Clone)]
pub enum Error {
    #[error("Lessons content could not be read: {0}")]
    Read(String),
    #[error("{0}")]
    Parse(String),
}
