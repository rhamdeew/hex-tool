// Hexo Blog Editor - Tauri Backend

mod commands;
mod config;
mod files;
mod frontmatter_config;
mod hexo;
mod markdown;

use commands::*;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            select_project_folder,
            get_project_config,
            get_frontmatter_config,
            generate_frontmatter_config_command,
            get_frontmatter_config_raw,
            save_frontmatter_config_raw,
            list_posts,
            get_post,
            save_post,
            create_post,
            delete_post,
            list_pages,
            create_page,
            get_page,
            save_page,
            delete_page,
            list_drafts,
            create_draft,
            get_draft,
            save_draft,
            delete_draft,
            list_images,
            copy_image_to_project,
            delete_image,
            list_image_folders,
            create_image_folder,
            rename_image_folder,
            delete_image_folder,
            get_app_config,
            save_app_config,
            run_hexo_command,
            start_hexo_server,
            stop_hexo_server,
            is_hexo_server_running,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
