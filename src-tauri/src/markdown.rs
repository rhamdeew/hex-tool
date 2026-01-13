// Markdown and frontmatter parsing

use crate::files;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
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

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Post {
    pub id: String,
    pub title: String,
    pub date: String,
    pub content: String,
    pub frontmatter: Frontmatter,
    pub file_path: String,
    pub created_at: i64,
    pub modified_at: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Page {
    pub id: String,
    pub title: String,
    pub content: String,
    pub frontmatter: Frontmatter,
    pub file_path: String,
    pub created_at: i64,
    pub modified_at: i64,
}

impl Page {
    pub fn to_markdown(&self) -> Result<String, String> {
        let frontmatter_yaml = serde_yaml::to_string(&self.frontmatter)
            .map_err(|e| format!("Failed to serialize frontmatter: {}", e))?;

        Ok(format!("---\n{}---\n\n{}", frontmatter_yaml, self.content))
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Draft {
    pub id: String,
    pub title: String,
    pub content: String,
    pub frontmatter: Frontmatter,
    pub file_path: String,
    pub created_at: i64,
    pub modified_at: i64,
}

impl Draft {
    pub fn to_markdown(&self) -> Result<String, String> {
        let frontmatter_yaml = serde_yaml::to_string(&self.frontmatter)
            .map_err(|e| format!("Failed to serialize frontmatter: {}", e))?;

        Ok(format!("---\n{}---\n\n{}", frontmatter_yaml, self.content))
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ImageInfo {
    pub filename: String,
    pub path: String,
    pub full_path: String,
    pub url: String,
    pub size: u64,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub created_at: i64,
}

impl Post {
    pub fn from_file(file_path: &Path, project_path: &Path) -> Result<Self, String> {
        let content = files::read_file(file_path)?;

        let doc = MarkdownDocument::parse(&content)?;

        // Get file metadata
        let metadata = fs::metadata(file_path)
            .map_err(|e| format!("Failed to get file metadata: {}", e))?;

        let created_at = metadata
            .created()
            .ok()
            .or(metadata.modified().ok())
            .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
            .map(|d: std::time::Duration| d.as_secs() as i64)
            .unwrap_or(0);

        let modified_at = metadata
            .modified()
            .ok()
            .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
            .map(|d: std::time::Duration| d.as_secs() as i64)
            .unwrap_or(0);

        // Generate ID (relative path from source/_posts/)
        let id = file_path
            .strip_prefix(project_path)
            .ok()
            .and_then(|p| p.to_str())
            .unwrap_or_else(|| file_path.to_str().unwrap_or(""))
            .to_string();

        Ok(Self {
            id,
            title: doc.frontmatter.title.clone(),
            date: doc.frontmatter.date.clone(),
            content: doc.content,
            frontmatter: doc.frontmatter,
            file_path: file_path.to_string_lossy().to_string(),
            created_at,
            modified_at,
        })
    }

    pub fn to_markdown(&self) -> Result<String, String> {
        let frontmatter_yaml = serde_yaml::to_string(&self.frontmatter)
            .map_err(|e| format!("Failed to serialize frontmatter: {}", e))?;

        Ok(format!("---\n{}---\n\n{}", frontmatter_yaml, self.content))
    }
}
