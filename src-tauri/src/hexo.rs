// Hexo integration module
// Handles Hexo project structure, config parsing, and operations

use std::path::PathBuf;
use std::process::{Command, Child, Stdio};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

// Global state to track running Hexo servers
lazy_static::lazy_static! {
    static ref HEXO_SERVERS: Arc<Mutex<HashMap<String, Child>>> = Arc::new(Mutex::new(HashMap::new()));
}

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

    /// Run a hexo command (generate, clean, deploy, etc.)
    pub fn run_command(&self, command: &str) -> Result<CommandOutput, String> {
        let output = Command::new("npx")
            .arg("hexo")
            .arg(command)
            .current_dir(&self.path)
            .output()
            .map_err(|e| format!("Failed to execute hexo command: {}", e))?;

        Ok(CommandOutput {
            success: output.status.success(),
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            exit_code: output.status.code().unwrap_or(-1),
        })
    }

    /// Start hexo server in background
    pub fn start_server(&self) -> Result<String, String> {
        let server_id = self.path.to_string_lossy().to_string();

        // Check if server is already running
        {
            let servers = HEXO_SERVERS.lock().unwrap();
            if servers.contains_key(&server_id) {
                return Err("Server is already running".to_string());
            }
        }

        // Start hexo server
        let child = Command::new("npx")
            .arg("hexo")
            .arg("server")
            .current_dir(&self.path)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| format!("Failed to start hexo server: {}", e))?;

        // Store the child process
        {
            let mut servers = HEXO_SERVERS.lock().unwrap();
            servers.insert(server_id.clone(), child);
        }

        Ok(server_id)
    }

    /// Stop running hexo server
    pub fn stop_server(server_id: &str) -> Result<(), String> {
        let mut servers = HEXO_SERVERS.lock().unwrap();

        if let Some(mut child) = servers.remove(server_id) {
            child.kill()
                .map_err(|e| format!("Failed to kill server process: {}", e))?;
            Ok(())
        } else {
            Err("Server not found".to_string())
        }
    }

    /// Check if server is running
    pub fn is_server_running(&self) -> bool {
        let server_id = self.path.to_string_lossy().to_string();
        let servers = HEXO_SERVERS.lock().unwrap();
        servers.contains_key(&server_id)
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct CommandOutput {
    pub success: bool,
    pub stdout: String,
    pub stderr: String,
    pub exit_code: i32,
}
