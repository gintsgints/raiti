mod exercise;
mod index;
mod lesson;

use serde::{Deserialize, Serialize};
use std::{borrow::Cow, fs, path::PathBuf};
use thiserror::Error;

use crate::{data::DataSource, environment, keyboard_config::KeyboardConfig, Result};
pub use index::Index;
pub use index::IndexRecord;
pub use lesson::Exercise;
pub use lesson::Lesson;

#[derive(Deserialize, Serialize, Default)]
pub struct Configuration {
    #[serde(default = "default_keyboard", alias = "current_keyboard_layout")]
    current_keyboard: String,
    #[serde(default)]
    current_lesson: String,
    #[serde(default)]
    current_page: usize,
    #[serde(default)]
    current_exercise: usize,
}

fn default_keyboard() -> String {
    "querty".to_string()
}

#[derive(Debug, Clone, Default)]
pub struct Config {
    pub index: Index,
    pub data: DataSource,
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

    fn read_lesson(&self, file_name: &str) -> Result<Lesson> {
        let content = self.data.read_text(&format!("{file_name}.yaml"))?;
        let read_image = |name: &str| self.data.read(name).map(Cow::into_owned);
        Ok(Lesson::parse(&content, &read_image)?)
    }

    pub fn read_keyboard(&self) -> Result<KeyboardConfig> {
        let content = self
            .data
            .read_text(&format!("keyboards/{}.yaml", self.current_keyboard))?;
        Ok(KeyboardConfig::parse(&content)?)
    }

    pub fn current_lesson(&self) -> Option<Result<Lesson>> {
        if self.current_lesson.is_empty() {
            None
        } else {
            Some(self.read_lesson(&self.current_lesson))
        }
    }

    fn path() -> PathBuf {
        Self::config_dir().join(environment::CONFIG_FILE_NAME)
    }

    pub fn load() -> Result<Self> {
        let path = Self::path();
        let Configuration {
            mut current_keyboard,
            current_lesson,
            current_page,
            current_exercise,
        } = if path.exists() {
            let content = fs::read_to_string(path).map_err(|e| Error::Read(e.to_string()))?;
            serde_yaml::from_str(content.as_ref()).map_err(|e| Error::Parse(e.to_string()))?
        } else {
            Configuration {
                current_keyboard: default_keyboard(),
                current_lesson: String::new(),
                ..Configuration::default()
            }
        };

        // A stored empty value would build a path like `keyboards/.yaml`, so fall back instead.
        if current_keyboard.is_empty() {
            current_keyboard = default_keyboard();
        }

        let (data, index) = DataSource::resolve(&environment::data_candidates(), &current_keyboard);

        Ok(Config {
            index,
            data,
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
        let lesson = self.read_lesson(file_name)?;
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
