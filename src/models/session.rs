use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct TmuxSession {
    pub name: String,
    pub attached: bool,
    pub path: Option<PathBuf>,
    pub windows: usize,
    pub created: Option<u64>,
}

impl TmuxSession {
    pub fn display_name(&self) -> String {
        let status = if self.attached { "●" } else { "○" };
        let path_str = self
            .path
            .as_ref()
            .and_then(|p| p.file_name())
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_default();

        if path_str.is_empty() {
            format!("{} {}", status, self.name)
        } else {
            format!("{} {} ({})", status, self.name, path_str)
        }
    }

    pub fn search_text(&self) -> String {
        let path_str = self
            .path
            .as_ref()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_default();
        format!("{} {}", self.name, path_str)
    }
}
