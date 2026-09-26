use serde::Deserialize;
use thiserror::Error;

#[derive(Debug, Default, Clone, Deserialize, PartialEq)]
pub struct IndexRecord {
    pub file: String,
    pub title: String,
}

#[derive(Debug, Default, Clone, Deserialize)]
pub struct Index {
    pub lessons: Vec<IndexRecord>,
}

impl Index {
    pub fn parse(content: &str) -> Result<Self, Error> {
        serde_yaml::from_str(content).map_err(|e| Error::Parse(e.to_string()))
    }

    pub fn next_lesson(&self, current_lesson: &str) -> Option<&str> {
        if let Some(index) = self
            .lessons
            .iter()
            .position(|index_record| index_record.file.eq(current_lesson))
        {
            self.lessons
                .get(index + 1)
                .map(|index_record| index_record.file.as_str())
        } else {
            None
        }
    }
}

#[derive(Debug, Error, Clone)]
pub enum Error {
    #[error("{0}")]
    Parse(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    const INDEX: &str = "lessons:\n  - file: l01\n    title: One\n  - file: l02\n    title: Two\n";

    #[test]
    fn parses_lesson_records() {
        let index = Index::parse(INDEX).unwrap();

        assert_eq!(index.lessons.len(), 2);
        assert_eq!(index.lessons[1].title, "Two");
    }

    #[test]
    fn rejects_malformed_content() {
        assert!(Index::parse("lessons: [oops").is_err());
    }

    #[test]
    fn next_lesson_follows_index_order() {
        let index = Index::parse(INDEX).unwrap();

        assert_eq!(index.next_lesson("l01"), Some("l02"));
        assert_eq!(index.next_lesson("l02"), None);
        assert_eq!(index.next_lesson("unknown"), None);
    }
}
