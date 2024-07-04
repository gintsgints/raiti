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
pub enum Location {
    /// The standard group of keys on the keyboard.
    #[default]
    Standard,
    /// The left side of the keyboard.
    Left,
    /// The right side of the keyboard.
    Right,
    /// The numpad of the keyboard.
    Numpad,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct KeySpec {
    pub key: Key,
    #[serde(default = "default_location")]
    pub location: Location,
    // Key width ratio against calculated key width
    // Should be specified if key is larger than usual keys
    #[serde(default = "default_width_ratio")]
    pub width_ratio: f32,
    pub label1: String,
    #[serde(default)]
    pub label2: String,
}

fn default_width_ratio() -> f32 {
    1.0
}

fn default_location() -> Location {
    Location::Standard
}

impl KeySpec {
    pub fn eq(&self, iced_key: iced::keyboard::Key, location: iced::keyboard::Location) -> bool {
        match iced_key {
            iced::keyboard::Key::Named(name) => {
                if let Key::Named(my_name) = &self.key {
                    let name_str = format!("{:?}", name);
                    if name_str.eq(my_name) {
                        match self.location {
                            Location::Left => {
                                if location == iced::keyboard::Location::Left {
                                    return true;
                                }
                            }
                            Location::Right => {
                                if location == iced::keyboard::Location::Right {
                                    return true;
                                }
                            }
                            _ => {}
                        }
                    }
                };
                false
            }
            iced::keyboard::Key::Character(character) => {
                if let Key::Character(my_name) = &self.key {
                    let name_with_quotes = format!(r#""{}""#, my_name);
                    let name_str = format!("{:?}", character);
                    if name_str.eq(&name_with_quotes) {
                        return true;
                    }
                }
                false
            }
            iced::keyboard::Key::Unidentified => false,
        }
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Row {
    pub keys: Vec<KeySpec>,
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

        file.read_to_string(&mut contents)
            .await
            .map_err(|_| LoadError::File)?;

        serde_yaml::from_str(&contents).map_err(|msg| LoadError::Format(format!("{:?}", msg)))
    }

    pub fn find_key(&self, key: iced::keyboard::Key, location: iced::keyboard::Location) -> Option<(usize, usize)> {
        for (key_y_index, row) in self.rows.iter().enumerate() {
            for (key_x_index, keyspec) in row.keys.iter().enumerate() {
                if keyspec.eq(key.clone(), location) {
                    return Some((key_x_index, key_y_index));
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use smol_str::SmolStr;

    use super::*;

    #[async_std::test]
    async fn loads_file_corect() {
        let result = Config::load("./test_data/test_keyboard.yaml").await;
        assert!(result.is_ok(), "Result should be ok parsed");

        if let Ok(parsed) = result {
            assert_eq!(
                parsed.cols_for_keys, 23.0,
                "Key number specified should be 23.0"
            );

            let tab_key = parsed.rows.get(1).unwrap().keys.first().unwrap();
            assert_eq!(
                tab_key.width_ratio, 1.5,
                "Width if specified should be specified"
            );

            let esc_key = parsed.rows.first().unwrap().keys.first().unwrap();
            assert_eq!(
                esc_key.width_ratio, 1.0,
                "If not provided key width should be 0.0"
            )
        }
    }

    #[test]
    fn key_compared_ok() {
        let iced_key = iced::keyboard::Key::Character(SmolStr::new("c"));
        let iced_location = iced::keyboard::Location::Standard;
        let keyspec = KeySpec {
            key: Key::Character("c".to_string()),
            location: Location::Standard,
            width_ratio: 1.0,
            label1: "label1".to_string(),
            label2: "label2".to_string(),
        };
        assert!(keyspec.eq(iced_key, iced_location), "C key should be found equal")
    }
}
