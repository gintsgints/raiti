mod exercise;
mod index;
mod lesson;

use index::Index;
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};
use thiserror::Error;

use crate::{environment, Result};
pub use index::IndexRecord;
pub use lesson::Exercise;
pub use lesson::Lesson;

#[derive(Deserialize, Serialize, Default)]
pub struct Configuration {
    #[serde(default)]
    current_keyboard: String,
    #[serde(default)]
    current_lesson: String,
    #[serde(default)]
    current_page: usize,
    #[serde(default)]
    current_exercise: usize,
}

#[derive(Debug, Clone, Default)]
pub struct Config {
    pub index: Index,
    pub current_keyboard: String,
    pub current_lesson: String,
    pub current_page: usize,
    pub current_exercise: usize,
}

impl Config {
    pub fn config_dir() -> PathBuf {
        let dir = environment::config_dir();

        if !dir.exists() {
            std::fs::create_dir_all(dir.as_path())
                .expect("expected permissions to create config folder");
        }

        dir
    }

    pub fn data_dir() -> PathBuf {
        environment::data_dir()
    }

    fn path() -> PathBuf {
        Self::config_dir().join(environment::CONFIG_FILE_NAME)
    }

    pub fn load() -> Result<Self> {
        let path = Self::path();
        let Configuration {
            current_keyboard,
            current_lesson,
            current_page,
            current_exercise,
        } = if path.exists() {
            let content = fs::read_to_string(path).map_err(|e| Error::Read(e.to_string()))?;
            serde_yaml::from_str(content.as_ref()).map_err(|e| Error::Parse(e.to_string()))?
        } else {
            Configuration {
                current_keyboard: "querty".to_string(),
                current_lesson: "".to_string(),
                ..Configuration::default()
            }
        };

        let index = Index::load(Self::data_dir().join("index.yaml"))?;
        Ok(Config {
            index,
            current_keyboard,
            current_lesson,
            current_page,
            current_exercise,
        })
    }

    pub async fn save(self) -> core::result::Result<(), Error> {
        let config_to_save = Configuration {
            current_keyboard: self.current_keyboard.clone(),
            current_lesson: self.current_lesson.clone(),
            current_page: self.current_page,
            current_exercise: self.current_exercise,
        };
        let config =
            serde_yaml::to_string(&config_to_save).map_err(|e| Error::Parse(e.to_string()))?;
        let path = Self::path();
        tokio::fs::write(path, &config)
            .await
            .map_err(|e| Error::Write(e.to_string()))?;
        Ok(())
    }

    // If current_page goes out of index, lesson is considered finished
    // and index page is shown.
    pub fn next_page(&mut self) {
        self.current_exercise = 0;
        self.current_page += 1;
    }

    pub fn load_lesson(&mut self, file_name: &str) -> Result<Lesson> {
        let lesson = Lesson::load(Self::data_dir().join(format!("{}.yaml", file_name)))?;
        self.current_lesson = file_name.to_string();
        self.current_exercise = 0;
        self.current_page = 0;
        Ok(lesson)
    }
}

#[derive(Debug, Clone, Error)]
pub enum Error {
    #[error("Config file could not be read: {0}")]
    Read(String),
    #[error("Config file could not be saved: {0}")]
    Write(String),
    #[error("{0}")]
    Parse(String),
}
