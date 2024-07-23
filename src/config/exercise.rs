use serde::Deserialize;

#[derive(Debug, Default, Clone, Deserialize, PartialEq)]
pub enum Exercise {
    #[default]
    None,
    OneLineNoEnter(String),
    Multiline(String),
}
