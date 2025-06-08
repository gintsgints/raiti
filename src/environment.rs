use std::{env, path::PathBuf};

pub const CONFIG_FILE_NAME: &str = "config.yaml";

pub fn config_dir() -> PathBuf {
    portable_dir().unwrap_or_else(platform_specific_config_dir)
}

pub fn data_dir() -> PathBuf {
    env::current_dir().ok().unwrap().join("data")
}

pub fn platform_specific_config_dir() -> PathBuf {
    dirs_next::config_dir()
        .expect("Cannot find valid configuration directory")
        .join("raiti")
}

/// Checks if a config file exists in the same directory as the executable.
/// If so, it'll use that directory for config dir.
/// Credit goes to - https://github.com/squidowl/halloy/blob/main/data/src/environment.rs
fn portable_dir() -> Option<PathBuf> {
    let exe = env::current_exe().ok()?;
    let dir = exe.parent()?;

    dir.join(CONFIG_FILE_NAME)
        .is_file()
        .then(|| dir.to_path_buf())
}
