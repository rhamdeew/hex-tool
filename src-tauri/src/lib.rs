// Hexo Blog Editor - Tauri Backend

mod commands;
mod config;
mod files;
mod hexo;
mod markdown;

use commands::*;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, test_connection])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
