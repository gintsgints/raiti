#[derive(Debug, Clone)]
pub enum LoadError {
    File,
    Format(String),
}
