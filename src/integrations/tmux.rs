use std::path::PathBuf;
use std::process::Command;

use crate::error::{PmanError, Result};
use crate::models::TmuxSession;

pub struct TmuxClient;

impl TmuxClient {
    pub fn new() -> Self {
        Self
    }

    pub fn list_sessions(&self) -> Result<Vec<TmuxSession>> {
        let output = Command::new("tmux")
            .args([
                "list-sessions",
                "-F",
                "#{session_name}\t#{session_attached}\t#{session_path}\t#{session_windows}\t#{session_created}",
            ])
            .output()
            .map_err(|e| PmanError::Tmux(e.to_string()))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            if stderr.contains("no server running") || stderr.contains("no sessions") {
                return Ok(Vec::new());
            }
            return Err(PmanError::Tmux(stderr.to_string()));
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let sessions = stdout
            .lines()
            .filter(|line| !line.is_empty())
            .filter_map(|line| {
                let parts: Vec<&str> = line.split('\t').collect();
                if parts.len() >= 4 {
                    Some(TmuxSession {
                        name: parts[0].to_string(),
                        attached: parts[1] == "1",
                        path: if parts[2].is_empty() {
                            None
                        } else {
                            Some(PathBuf::from(parts[2]))
                        },
                        windows: parts[3].parse().unwrap_or(1),
                        created: parts.get(4).and_then(|s| s.parse().ok()),
                    })
                } else {
                    None
                }
            })
            .collect();

        Ok(sessions)
    }

    pub fn switch_session(&self, session_name: &str) -> Result<()> {
        let output = Command::new("tmux")
            .args(["switch-client", "-t", session_name])
            .output()
            .map_err(|e| PmanError::Tmux(e.to_string()))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(PmanError::Tmux(stderr.to_string()));
        }

        Ok(())
    }

    pub fn create_session(&self, name: &str, path: Option<&PathBuf>) -> Result<()> {
        let mut args = vec!["new-session", "-d", "-s", name];

        if let Some(p) = path {
            args.push("-c");
            args.push(p.to_str().unwrap_or("."));
        }

        let output = Command::new("tmux")
            .args(&args)
            .output()
            .map_err(|e| PmanError::Tmux(e.to_string()))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(PmanError::Tmux(stderr.to_string()));
        }

        Ok(())
    }

    pub fn kill_session(&self, session_name: &str) -> Result<()> {
        let output = Command::new("tmux")
            .args(["kill-session", "-t", session_name])
            .output()
            .map_err(|e| PmanError::Tmux(e.to_string()))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(PmanError::Tmux(stderr.to_string()));
        }

        Ok(())
    }

    pub fn current_session(&self) -> Result<String> {
        let output = Command::new("tmux")
            .args(["display-message", "-p", "#{session_name}"])
            .output()
            .map_err(|e| PmanError::Tmux(e.to_string()))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(PmanError::Tmux(stderr.to_string()));
        }

        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    }

    pub fn current_path(&self) -> Result<PathBuf> {
        let output = Command::new("tmux")
            .args(["display-message", "-p", "#{pane_current_path}"])
            .output()
            .map_err(|e| PmanError::Tmux(e.to_string()))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(PmanError::Tmux(stderr.to_string()));
        }

        Ok(PathBuf::from(
            String::from_utf8_lossy(&output.stdout).trim(),
        ))
    }

    pub fn get_or_create_editor_window(&self) -> Result<String> {
        // Check if "editor" window exists
        let output = Command::new("tmux")
            .args([
                "list-windows",
                "-F",
                "#{window_name}\t#{window_id}",
            ])
            .output()
            .map_err(|e| PmanError::Tmux(e.to_string()))?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        for line in stdout.lines() {
            let parts: Vec<&str> = line.split('\t').collect();
            if parts.first() == Some(&"editor") {
                return Ok(parts.get(1).unwrap_or(&"").to_string());
            }
        }

        // Create editor window
        let output = Command::new("tmux")
            .args(["new-window", "-n", "editor", "-P", "-F", "#{window_id}"])
            .output()
            .map_err(|e| PmanError::Tmux(e.to_string()))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(PmanError::Tmux(stderr.to_string()));
        }

        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    }

    pub fn send_keys(&self, target: &str, keys: &str) -> Result<()> {
        let output = Command::new("tmux")
            .args(["send-keys", "-t", target, keys, "Enter"])
            .output()
            .map_err(|e| PmanError::Tmux(e.to_string()))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(PmanError::Tmux(stderr.to_string()));
        }

        Ok(())
    }

    pub fn select_window(&self, window_id: &str) -> Result<()> {
        let output = Command::new("tmux")
            .args(["select-window", "-t", window_id])
            .output()
            .map_err(|e| PmanError::Tmux(e.to_string()))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(PmanError::Tmux(stderr.to_string()));
        }

        Ok(())
    }

    pub fn popup_command(&self, command: &str, width: &str, height: &str) -> Result<()> {
        let output = Command::new("tmux")
            .args([
                "display-popup",
                "-E",
                "-w", width,
                "-h", height,
                command,
            ])
            .output()
            .map_err(|e| PmanError::Tmux(e.to_string()))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            // Popup failures are often not critical (e.g., user closed it)
            if !stderr.is_empty() {
                eprintln!("Popup warning: {}", stderr);
            }
        }

        Ok(())
    }
}

impl Default for TmuxClient {
    fn default() -> Self {
        Self::new()
    }
}
