use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct NvimBuffer {
    pub bufnr: i64,
    pub name: PathBuf,
    pub modified: bool,
}

impl NvimBuffer {
    pub fn display_name(&self) -> String {
        let name = self.name.file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| "[No Name]".to_string());

        if self.modified {
            format!("{} [+]", name)
        } else {
            name
        }
    }

    pub fn search_text(&self) -> String {
        self.name.to_string_lossy().to_string()
    }
}
