use serde::Deserialize;
use std::{fs, path::PathBuf};
use thiserror::Error;

#[derive(Debug, Default, Clone, Deserialize, PartialEq)]
pub struct IndexRecord {
    pub file: String,
    pub title: String,
}

impl ToString for IndexRecord {
    fn to_string(&self) -> String {
        self.title.clone()
    }
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
}

#[derive(Debug, Error, Clone)]
pub enum Error {
    #[error("Lessons content could not be read: {0}")]
    Read(String),
    #[error("{0}")]
    Parse(String),
}
