// Hexo integration module
// Handles Hexo project structure, config parsing, and operations

use std::path::PathBuf;

pub struct HexoProject {
    pub path: PathBuf,
}

impl HexoProject {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }

    pub fn validate(&self) -> Result<bool, String> {
        // Check if _config.yml exists
        let config_path = self.path.join("_config.yml");
        if !config_path.exists() {
            return Err("_config.yml not found".to_string());
        }

        // Check if source/ directory exists
        let source_path = self.path.join("source");
        if !source_path.exists() || !source_path.is_dir() {
            return Err("source/ directory not found".to_string());
        }

        Ok(true)
    }

    pub fn get_posts_dir(&self) -> PathBuf {
        self.path.join("source").join("_posts")
    }

    pub fn get_pages_dir(&self) -> PathBuf {
        self.path.join("source")
    }

    pub fn get_images_dir(&self) -> PathBuf {
        self.path.join("source").join("images")
    }
}
