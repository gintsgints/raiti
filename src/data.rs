use rust_embed::RustEmbed;
use std::{
    borrow::Cow,
    fmt,
    path::{Path, PathBuf},
};
use thiserror::Error;

use crate::{config::Index, keyboard_config::KeyboardConfig};

/// Lessons, keyboard layouts and images shipped inside the binary.
///
/// In debug builds `rust-embed` reads these from `data/` on disk instead, so
/// lesson edits show up without a rebuild.
#[derive(RustEmbed)]
#[folder = "data/"]
#[exclude = "*.DS_Store"]
struct Embedded;

/// Where lessons are read from. One source serves every file: a user data
/// directory either replaces the embedded set completely or is not used at all.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum DataSource {
    #[default]
    Embedded,
    Dir(PathBuf),
}

impl fmt::Display for DataSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Embedded => write!(f, "embedded data"),
            Self::Dir(dir) => write!(f, "{}", dir.display()),
        }
    }
}

impl DataSource {
    /// Picks the first candidate directory holding usable data, falling back to
    /// the embedded set. Returns the index it validated, so it is parsed once.
    pub fn resolve(candidates: &[PathBuf], keyboard: &str) -> (Self, Index) {
        for dir in candidates {
            if !dir.is_dir() {
                continue;
            }

            let source = Self::Dir(dir.clone());
            match source.validate(keyboard) {
                Ok(index) => {
                    eprintln!("Using data directory {}", dir.display());
                    return (source, index);
                }
                Err(e) => eprintln!("Ignoring data directory {}: {e}", dir.display()),
            }
        }

        // The embedded index ships with the binary, so a failure here is a build
        // error rather than anything the user can cause or fix.
        let index = Index::parse(
            &Self::Embedded
                .read_text("index.yaml")
                .expect("embedded index.yaml"),
        )
        .expect("embedded index.yaml should parse");

        (Self::Embedded, index)
    }

    /// Accepts a data directory only if the files needed to start are present
    /// and parse. Lesson contents are not parsed here; a lesson that is present
    /// but broken is reported when it is opened.
    fn validate(&self, keyboard: &str) -> Result<Index, Error> {
        let index = Index::parse(&self.read_text("index.yaml")?)
            .map_err(|e| Error::Invalid("index.yaml".to_string(), e.to_string()))?;

        let keyboard_file = format!("keyboards/{keyboard}.yaml");
        KeyboardConfig::parse(&self.read_text(&keyboard_file)?)
            .map_err(|e| Error::Invalid(keyboard_file, e.to_string()))?;

        for record in &index.lessons {
            let lesson_file = format!("{}.yaml", record.file);
            if self.read(&lesson_file).is_none() {
                return Err(Error::Read(lesson_file, self.to_string()));
            }
        }

        Ok(index)
    }

    /// Reads a data file named relative to the data directory, with `/` as the
    /// separator. Returns `None` when it is missing or unreadable.
    pub fn read(&self, rel_path: &str) -> Option<Cow<'static, [u8]>> {
        match self {
            Self::Embedded => Embedded::get(rel_path).map(|file| file.data),
            Self::Dir(dir) => std::fs::read(Self::join(dir, rel_path))
                .ok()
                .map(Cow::Owned),
        }
    }

    pub fn read_text(&self, rel_path: &str) -> Result<String, Error> {
        let bytes = self
            .read(rel_path)
            .ok_or_else(|| Error::Read(rel_path.to_string(), self.to_string()))?;

        String::from_utf8(bytes.into_owned())
            .map_err(|e| Error::Invalid(rel_path.to_string(), e.to_string()))
    }

    fn join(dir: &Path, rel_path: &str) -> PathBuf {
        rel_path
            .split('/')
            .fold(dir.to_path_buf(), |path, part| path.join(part))
    }
}

#[derive(Debug, Clone, Error)]
pub enum Error {
    #[error("{0} could not be read from {1}")]
    Read(String, String),
    #[error("{0} is not valid: {1}")]
    Invalid(String, String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    const KEYBOARD: &str = "test_layout";

    fn write(dir: &Path, rel_path: &str, content: &str) {
        let path = DataSource::join(dir, rel_path);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, content).unwrap();
    }

    /// A data directory that passes validation, with a single lesson.
    fn valid_dir() -> TempDir {
        let dir = TempDir::new().unwrap();
        write(
            dir.path(),
            "index.yaml",
            "lessons:\n  - file: l01\n    title: One\n",
        );
        write(dir.path(), "l01.yaml", "pages: []\n");
        write(
            dir.path(),
            &format!("keyboards/{KEYBOARD}.yaml"),
            "cols_for_keys: 1.0\nspace_between_keys: 1.0\nkeyboard_corner_curve: 1.0\nkeyboard_side_padding: 1.0\nkey_text_top_pad: 1.0\nkey_text_left_pad: 1.0\nrows: []\n",
        );
        dir
    }

    fn resolve(dirs: &[&TempDir]) -> DataSource {
        let candidates: Vec<PathBuf> = dirs.iter().map(|d| d.path().to_path_buf()).collect();
        DataSource::resolve(&candidates, KEYBOARD).0
    }

    #[test]
    fn first_valid_candidate_wins() {
        let first = valid_dir();
        let second = valid_dir();

        assert_eq!(
            resolve(&[&first, &second]),
            DataSource::Dir(first.path().to_path_buf())
        );
    }

    #[test]
    fn candidate_index_is_parsed_once_and_returned() {
        let dir = valid_dir();
        let (_, index) = DataSource::resolve(&[dir.path().to_path_buf()], KEYBOARD);

        assert_eq!(index.lessons.len(), 1);
        assert_eq!(index.lessons[0].file, "l01");
    }

    #[test]
    fn unparsable_index_falls_through_to_next_candidate() {
        let broken = valid_dir();
        write(broken.path(), "index.yaml", "lessons: [oops\n");
        let good = valid_dir();

        assert_eq!(
            resolve(&[&broken, &good]),
            DataSource::Dir(good.path().to_path_buf())
        );
    }

    #[test]
    fn missing_keyboard_rejects_candidate() {
        let dir = valid_dir();
        fs::remove_file(DataSource::join(
            dir.path(),
            &format!("keyboards/{KEYBOARD}.yaml"),
        ))
        .unwrap();

        assert_eq!(resolve(&[&dir]), DataSource::Embedded);
    }

    #[test]
    fn lesson_listed_in_index_but_missing_rejects_candidate() {
        let dir = valid_dir();
        fs::remove_file(DataSource::join(dir.path(), "l01.yaml")).unwrap();

        assert_eq!(resolve(&[&dir]), DataSource::Embedded);
    }

    #[test]
    fn broken_lesson_content_is_left_for_the_lesson_to_report() {
        let dir = valid_dir();
        write(dir.path(), "l01.yaml", "pages: [oops\n");

        assert_eq!(resolve(&[&dir]), DataSource::Dir(dir.path().to_path_buf()));
    }

    #[test]
    fn missing_directories_are_skipped() {
        let good = valid_dir();
        let candidates = vec![
            PathBuf::from("/raiti-does-not-exist"),
            good.path().to_path_buf(),
        ];

        assert_eq!(
            DataSource::resolve(&candidates, KEYBOARD).0,
            DataSource::Dir(good.path().to_path_buf())
        );
    }

    #[test]
    fn no_candidates_falls_back_to_embedded() {
        assert_eq!(DataSource::resolve(&[], KEYBOARD).0, DataSource::Embedded);
    }

    #[test]
    fn reads_nested_files_from_a_directory() {
        let dir = valid_dir();
        let source = DataSource::Dir(dir.path().to_path_buf());

        assert!(source
            .read_text(&format!("keyboards/{KEYBOARD}.yaml"))
            .unwrap()
            .contains("rows"));
        assert!(source.read("keyboards/missing.yaml").is_none());
        assert!(source.read_text("missing.yaml").is_err());
    }

    #[test]
    fn reads_shipped_files_from_the_embedded_set() {
        let source = DataSource::Embedded;

        assert!(!source.read_text("index.yaml").unwrap().is_empty());
        assert!(source.read("img/sitting1.jpg").is_some());
        assert!(source.read("missing.yaml").is_none());
    }

    #[test]
    fn embedded_data_is_valid() {
        assert!(DataSource::Embedded.validate("querty").is_ok());
    }

    #[test]
    fn every_shipped_lesson_parses() {
        let source = DataSource::Embedded;
        let index = source.validate("querty").unwrap();
        let read_image = |name: &str| source.read(name).map(Cow::into_owned);

        for record in &index.lessons {
            let file = format!("{}.yaml", record.file);
            let content = source.read_text(&file).unwrap();
            if let Err(e) = crate::config::Lesson::parse(&content, &read_image) {
                panic!("{file} does not parse: {e}");
            }
        }
    }
}
