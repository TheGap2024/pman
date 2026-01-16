use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct GitWorktree {
    pub path: PathBuf,
    pub branch: String,
    pub is_main: bool,
    pub commit_hash: String,
    pub has_changes: bool,
}

impl GitWorktree {
    pub fn display_name(&self) -> String {
        let status = if self.has_changes { "*" } else { "" };
        let main_marker = if self.is_main { " [main]" } else { "" };
        format!(
            "{}{} ({}){}",
            self.branch,
            status,
            &self.commit_hash[..7.min(self.commit_hash.len())],
            main_marker
        )
    }

    pub fn search_text(&self) -> String {
        format!(
            "{} {}",
            self.branch,
            self.path.to_string_lossy()
        )
    }
}
