use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
pub enum Key {
    /// A key with an established name.
    Named(String),

    /// A key string that corresponds to the character typed by the user, taking into account the
    /// user’s current locale setting, and any system-level keyboard mapping overrides that are in
    /// effect.
    Character(String),

    /// An unidentified key.
    #[default]
    Unidentified,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct KeySpec {
    key: Key,
    label1: String,
    #[serde(default)]
    label2: String,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Row {
    keys: Vec<KeySpec>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Config {
    pub name: String,
    pub cols_for_keys: f32,
    pub space_between_keys: f32,
    pub keyboard_corner_curve: f32,
    pub keyboard_side_padding: f32,
    pub key_corner_curve: f32,
    pub key_text_top_pad: f32,
    pub key_text_left_pad: f32,
    pub rows: Vec<Row>,
}

#[derive(Debug, Clone)]
pub enum LoadError {
    File,
    Format(String),
}

impl Config {
    pub async fn load(path: &str) -> Result<Config, LoadError> {
        use async_std::prelude::*;

        let mut contents = String::new();

        let mut file = async_std::fs::File::open(path)
            .await
            .map_err(|_| LoadError::File)?;

        file.read_to_string(&mut contents).await.map_err(|_| LoadError::File)?;
        
        serde_yaml::from_str(&contents).map_err(|msg| {
            LoadError::Format(format!("{:?}", msg))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[async_std::test]
    async fn loads_file_corect() {
        let result = Config::load("./test_data/test_keyboard.yaml").await;
        assert!(result.is_ok(), "Result should be ok parsed");

        if let Ok(parsed) = result {
            assert_eq!(parsed.cols_for_keys, 23.0)
        }
    }
}
