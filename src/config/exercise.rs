use serde::Deserialize;

#[derive(Debug, Default, Clone, Deserialize)]
pub enum Exercise {
    #[default]
    None,
    OneLineNoEnter(String),
}
