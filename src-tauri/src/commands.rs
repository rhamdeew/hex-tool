// Tauri commands for frontend-backend communication

use tauri::command;

#[command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[command]
pub fn test_connection() -> Result<String, String> {
    Ok("Connection successful!".to_string())
}

// Placeholder for future commands
// pub fn select_project_folder() -> Result<String, String>
// pub fn list_posts(project_path: String) -> Result<Vec<Post>, String>
// pub fn get_post(project_path: String, post_id: String) -> Result<Post, String>
// pub fn save_post(project_path: String, post: Post) -> Result<(), String>
// etc.
