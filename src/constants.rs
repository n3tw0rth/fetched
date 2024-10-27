use dirs;
use std::path::PathBuf;

pub struct Constants {
    pub app_config_path: PathBuf,
    pub file_extension: String,
}

impl Constants {
    pub fn new() -> Self {
        Self {
            app_config_path: dirs::config_dir().unwrap().join("fetched"),
            file_extension: ".ft".to_string(),
        }
    }
}
