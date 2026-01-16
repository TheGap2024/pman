use std::path::{Path, PathBuf};
use std::process::Command;

use crate::error::{PmanError, Result};
use crate::integrations::TmuxClient;
use crate::models::NvimBuffer;

pub struct NvimIntegration {
    tmux: TmuxClient,
}

impl NvimIntegration {
    pub fn new(tmux: TmuxClient) -> Self {
        Self { tmux }
    }

    pub fn open_file(&self, file_path: &Path) -> Result<()> {
        let window_id = self.tmux.get_or_create_editor_window()?;

        let nvim_cmd = format!("nvim \"{}\"", file_path.display());
        self.tmux.send_keys(&window_id, &nvim_cmd)?;
        self.tmux.select_window(&window_id)?;

        Ok(())
    }

    pub fn open_buffer(&self, socket: &Path, bufnr: i64) -> Result<()> {
        // Switch to buffer in nvim
        let cmd = format!(":buffer {}", bufnr);
        Command::new("nvim")
            .args(["--server", socket.to_str().unwrap_or(""), "--remote-send", &cmd])
            .output()
            .map_err(|e| PmanError::Nvim(e.to_string()))?;

        // Switch to editor window
        let _ = self.tmux.get_or_create_editor_window().and_then(|w| self.tmux.select_window(&w));

        Ok(())
    }

    pub fn list_buffers() -> Result<Vec<(PathBuf, NvimBuffer)>> {
        let sockets = Self::find_nvim_sockets()?;
        let mut all_buffers = Vec::new();

        for socket in sockets {
            if let Ok(buffers) = Self::get_buffers_from_socket(&socket) {
                for buf in buffers {
                    all_buffers.push((socket.clone(), buf));
                }
            }
        }

        Ok(all_buffers)
    }

    fn find_nvim_sockets() -> Result<Vec<PathBuf>> {
        let mut sockets = Vec::new();

        // Check common nvim socket locations
        let user = std::env::var("USER").unwrap_or_default();

        // macOS: /tmp/nvim.<user>/
        let tmp_dir = PathBuf::from(format!("/tmp/nvim.{}", user));
        if tmp_dir.exists() {
            if let Ok(entries) = std::fs::read_dir(&tmp_dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_dir() {
                        if let Ok(inner_entries) = std::fs::read_dir(&path) {
                            for inner in inner_entries.flatten() {
                                let inner_path = inner.path();
                                if inner_path.to_string_lossy().contains("nvim.") {
                                    sockets.push(inner_path);
                                }
                            }
                        }
                    }
                }
            }
        }

        // Also check XDG_RUNTIME_DIR (Linux)
        if let Ok(runtime_dir) = std::env::var("XDG_RUNTIME_DIR") {
            let nvim_dir = PathBuf::from(runtime_dir);
            if let Ok(entries) = std::fs::read_dir(&nvim_dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.to_string_lossy().contains("nvim") {
                        sockets.push(path);
                    }
                }
            }
        }

        Ok(sockets)
    }

    fn get_buffers_from_socket(socket: &Path) -> Result<Vec<NvimBuffer>> {
        let expr = r#"json_encode(map(getbufinfo({'buflisted': 1}), {_, v -> {'bufnr': v.bufnr, 'name': v.name, 'changed': v.changed}}))"#;

        let output = Command::new("nvim")
            .args([
                "--server",
                socket.to_str().unwrap_or(""),
                "--remote-expr",
                expr,
            ])
            .output()
            .map_err(|e| PmanError::Nvim(e.to_string()))?;

        if !output.status.success() {
            return Err(PmanError::Nvim("Failed to get buffers".to_string()));
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        Self::parse_buffer_json(&stdout)
    }

    fn parse_buffer_json(json: &str) -> Result<Vec<NvimBuffer>> {
        // Simple JSON parsing without external crate
        let json = json.trim();
        if !json.starts_with('[') || !json.ends_with(']') {
            return Ok(Vec::new());
        }

        let mut buffers = Vec::new();
        let inner = &json[1..json.len() - 1];

        // Split by },{ to get individual objects
        for obj_str in inner.split("},{") {
            let obj_str = obj_str.trim_start_matches('{').trim_end_matches('}');

            let mut bufnr: i64 = 0;
            let mut name = String::new();
            let mut changed = false;

            for part in obj_str.split(',') {
                let part = part.trim();
                if let Some((key, value)) = part.split_once(':') {
                    let key = key.trim().trim_matches('"');
                    let value = value.trim();

                    match key {
                        "bufnr" => bufnr = value.parse().unwrap_or(0),
                        "name" => name = value.trim_matches('"').to_string(),
                        "changed" => changed = value == "1" || value == "true",
                        _ => {}
                    }
                }
            }

            if !name.is_empty() {
                buffers.push(NvimBuffer {
                    bufnr,
                    name: PathBuf::from(name),
                    modified: changed,
                });
            }
        }

        Ok(buffers)
    }
}
