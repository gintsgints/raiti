use std::{env, path::PathBuf};

pub const CONFIG_FILE_NAME: &str = "config.yaml";
pub const DATA_DIR_NAME: &str = "data";

pub fn config_dir() -> PathBuf {
    portable_dir().unwrap_or_else(platform_specific_config_dir)
}

/// Data directories to try, in order, before falling back to the embedded set.
/// The executable's own directory is kept for the layout older releases shipped,
/// where `data` sat next to the binary.
pub fn data_candidates() -> Vec<PathBuf> {
    let mut candidates = vec![config_dir().join(DATA_DIR_NAME)];

    if let Some(dir) = exe_dir() {
        let legacy = dir.join(DATA_DIR_NAME);
        if !candidates.contains(&legacy) {
            candidates.push(legacy);
        }
    }

    candidates
}

pub fn platform_specific_config_dir() -> PathBuf {
    dirs_next::config_dir()
        .expect("Cannot find valid configuration directory")
        .join("raiti")
}

/// Checks if a config file exists in the same directory as the executable.
/// If so, it'll use that directory for config dir.
/// Credit goes to - <https://github.com/squidowl/halloy/blob/main/data/src/environment.rs>
fn portable_dir() -> Option<PathBuf> {
    let dir = exe_dir()?;

    dir.join(CONFIG_FILE_NAME).is_file().then_some(dir)
}

fn exe_dir() -> Option<PathBuf> {
    Some(env::current_exe().ok()?.parent()?.to_path_buf())
}
