use std::path::Path;

use crate::error::Result;
use crate::integrations::TmuxClient;

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
}
