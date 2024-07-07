use serde::Deserialize;

use crate::{config::PressedKeyCoord, error::LoadError};

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub enum LessonAction {
    ShowKey(PressedKeyCoord),
    ShowCursor,
    HideCursor,
}

#[derive(Debug, Default, Clone, Deserialize)]
pub struct LessonPage {
    pub title: String,
    pub content: String,
    pub actions: Vec<LessonAction>,
    #[serde(default = "empty_string")]
    pub content2: String,
}

fn empty_string() -> String {
    "".to_string()
}

#[derive(Debug, Default, Clone, Deserialize)]
pub struct Lesson {
    pub _title: String,
    pub pages: Vec<LessonPage>
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
