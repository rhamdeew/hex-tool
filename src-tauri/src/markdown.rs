// Markdown and frontmatter parsing

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Frontmatter {
    pub title: String,
    pub date: String,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub categories: Vec<String>,
    pub permalink: Option<String>,
    pub list_image: Option<String>,
    pub list_image_alt: Option<String>,
    pub main_image: Option<String>,
    pub main_image_alt: Option<String>,
    #[serde(flatten)]
    pub custom_fields: HashMap<String, serde_yaml::Value>,
}

#[derive(Debug)]
pub struct MarkdownDocument {
    pub frontmatter: Frontmatter,
    pub content: String,
}

impl MarkdownDocument {
    pub fn parse(raw: &str) -> Result<Self, String> {
        // Check if the document starts with frontmatter delimiter
        if !raw.starts_with("---") {
            return Err("No frontmatter found".to_string());
        }

        // Find the end of frontmatter
        let parts: Vec<&str> = raw.splitn(3, "---").collect();
        if parts.len() < 3 {
            return Err("Invalid frontmatter format".to_string());
        }

        // Parse YAML frontmatter
        let frontmatter_str = parts[1].trim();
        let frontmatter: Frontmatter = serde_yaml::from_str(frontmatter_str)
            .map_err(|e| format!("Failed to parse frontmatter: {}", e))?;

        // Content is everything after the second ---
        let content = parts[2].trim().to_string();

        Ok(Self {
            frontmatter,
            content,
        })
    }

    pub fn to_string(&self) -> Result<String, String> {
        let frontmatter_yaml = serde_yaml::to_string(&self.frontmatter)
            .map_err(|e| format!("Failed to serialize frontmatter: {}", e))?;

        Ok(format!("---\n{}---\n\n{}", frontmatter_yaml, self.content))
    }
}
