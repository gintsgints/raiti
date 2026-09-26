use iced::widget::image;
use serde::Deserialize;
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
    #[serde(default)]
    pub image: Option<String>,
    #[serde(default)]
    pub image_width: Option<f32>,
    /// Decoded once at load time; cloning it in `view` reuses the cached texture.
    #[serde(skip)]
    pub image_handle: Option<image::Handle>,
}

#[derive(Debug, Default, Clone, Deserialize)]
pub struct Lesson {
    pub pages: Vec<LessonPage>,
}

impl Lesson {
    /// Parses a lesson and resolves its images through `read_image`, which is
    /// given the image name as written in the lesson file.
    pub fn parse(
        content: &str,
        read_image: &dyn Fn(&str) -> Option<Vec<u8>>,
    ) -> Result<Self, Error> {
        let mut lesson: Lesson =
            serde_yaml::from_str(content).map_err(|e| Error::Parse(e.to_string()))?;

        for page in &mut lesson.pages {
            let Some(name) = page.image.clone() else {
                continue;
            };
            match read_image(&name) {
                Some(bytes) => page.image_handle = Some(image::Handle::from_bytes(bytes)),
                None => eprintln!("Lesson image {name} could not be read, showing text only"),
            }
        }

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
    #[error("{0}")]
    Parse(String),
}
