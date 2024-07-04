use serde::{Deserialize, Serialize};

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
}

#[derive(Debug, Clone)]
pub enum LoadError {
    File,
    Format,
}

impl Config {
    pub async fn load() -> Result<Config, LoadError> {
        use async_std::prelude::*;

        let mut contents = String::new();

        let mut file = async_std::fs::File::open("./data/keyboards/querty.yaml")
            .await
            .map_err(|_| LoadError::File)?;

        file.read_to_string(&mut contents).await.map_err(|_| LoadError::File)?;
        
        serde_yaml::from_str(&contents).map_err(|msg| {
            println!("Parse error {:?}", msg);
            LoadError::Format
        })
    }
}
