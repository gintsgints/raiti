mod exercise;
mod keyboard;
mod lesson;
mod index;

use index::Index;
use serde::{Deserialize, Serialize};
use std::{fs, io, path::PathBuf};
use thiserror::Error;

use crate::{environment, Result};
pub use keyboard::{Keyboard, PressedKeyCoord};
pub use lesson::Exercise;
pub use index::IndexRecord;
use lesson::{Lesson, LessonPage};

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
    pub keyboard: Keyboard,
    pub lesson: Lesson,
    pub index: Index,
    current_keyboard: String,
    current_lesson: String,
    current_page: usize,
    current_exercise: usize,
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
                current_lesson: "intro".to_string(),
                ..Configuration::default()
            }
        };

        let keyboard = Keyboard::load(
            Self::data_dir()
                .join("keyboards")
                .join(format!("{}.yaml", current_keyboard)),
        )?;
        let lesson = Lesson::load(Self::data_dir().join(format!("{}.yaml", current_lesson)))?;
        let index = Index::load(Self::data_dir().join("index.yaml"))?;
        Ok(Config {
            keyboard,
            lesson,
            index,
            current_keyboard,
            current_lesson,
            current_page,
            current_exercise,
        })
    }

    pub async fn save(self) -> core::result::Result<(), Error> {
        let config_to_save =    Configuration {
            current_keyboard: self.current_keyboard.clone(),
            current_lesson: self.current_lesson.clone(),
            current_page: self.current_page,
            current_exercise: self.current_exercise,
        };
        let config = serde_yaml::to_string(&config_to_save).map_err(|e| Error::Parse(e.to_string()))?;
        let path = Self::path();
        tokio::fs::write(path, &config).await.map_err(|e| Error::Write(e.to_string()))?;
        Ok(())
    }

    pub fn get_page(&self) -> Option<&LessonPage> {
        self.lesson.pages.get(self.current_page)
    }

    // If current_page goes out of index, lesson is considered finished
    // and index page is shown.
    pub fn next_page(&mut self) {
        self.current_page += 1;
    }

    pub fn load_lesson(&mut self, file_name: &str) -> Result<()> {
        self.lesson = Lesson::load(Self::data_dir().join(format!("{}.yaml", file_name)))?;
        self.current_lesson = file_name.to_string();
        self.current_exercise = 0;
        self.current_page = 0;
        Ok(())
    }

    pub fn get_exercise(&self) -> Option<&Exercise> {
        match self.get_page() {
            Some(page) => page.exercises.get(self.current_exercise),
            None => None,
        }
    }

    pub fn next_exercise(&mut self) {
        if let Some(page) = self.get_page() {
            if page.exercises.get(self.current_exercise + 1).is_some() {
                self.current_exercise += 1;
            }
        }
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
