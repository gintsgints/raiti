mod exercise;
mod keyboard;
mod lesson;

use serde::Deserialize;
use std::{fs, path::PathBuf};
use thiserror::Error;

use crate::{environment, Result};
pub use keyboard::{Keyboard, PressedKeyCoord};
pub use lesson::Exercise;
use lesson::{Lesson, LessonPage};

#[derive(Debug, Clone, Default)]
pub struct Config {
    pub keyboard: Keyboard,
    pub lesson: Lesson,
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
        #[derive(Deserialize, Default)]
        pub struct Configuration {
            #[serde(default)]
            current_keyboard: String,
            #[serde(default)]
            current_lesson: String,
            #[serde(default)]
            next_page: usize,
            #[serde(default)]
            next_exercise: usize,
        }

        let path = Self::path();
        let Configuration {
            current_keyboard,
            current_lesson,
            next_page,
            next_exercise,
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
        Ok(Config {
            keyboard,
            lesson,
            current_page: next_page,
            current_exercise: next_exercise,
        })
    }

    pub fn get_page(&self) -> Option<&LessonPage> {
        self.lesson.pages.get(self.current_page)
    }

    pub fn next_page(&mut self) {
        if self.lesson.pages.get(self.current_page + 1).is_some() {
            self.current_page += 1;
        }
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

#[derive(Debug, Error, Clone)]
pub enum Error {
    #[error("Config file could not be read: {0}")]
    Read(String),
    #[error("{0}")]
    Parse(String),
}
