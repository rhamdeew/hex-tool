// Application configuration management

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppConfig {
    pub version: String,
    pub last_project_path: Option<String>,
    pub recent_projects: Vec<String>,
    pub ui_language: String,
    pub theme: String,
    pub auto_save_enabled: bool,
    pub auto_save_interval: u32,
    pub editor_font_size: u32,
    pub editor_line_height: f32,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            version: env!("CARGO_PKG_VERSION").to_string(),
            last_project_path: None,
            recent_projects: Vec::new(),
            ui_language: "en".to_string(),
            theme: "auto".to_string(),
            auto_save_enabled: true,
            auto_save_interval: 30,
            editor_font_size: 16,
            editor_line_height: 1.5,
        }
    }
}

impl AppConfig {
    pub fn load() -> Result<Self, String> {
        // TODO: Load from config file
        // For now, return default
        Ok(Self::default())
    }

    pub fn save(&self) -> Result<(), String> {
        // TODO: Save to config file
        Ok(())
    }
}
