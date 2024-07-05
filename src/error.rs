#[derive(Debug, Clone)]
pub enum LoadError {
    File,
    Format(String),
}

#[derive(Debug, Clone)]
pub enum LessonError {
    Err(String),
}
