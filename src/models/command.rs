#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PaletteCommand {
    OpenFile,
    NewSession,
    KillSession,
    ListWorktrees,
    CreateWorktree,
    GitStatus,
}

impl PaletteCommand {
    pub fn all() -> Vec<Self> {
        vec![
            Self::OpenFile,
            Self::NewSession,
            Self::KillSession,
            Self::ListWorktrees,
            Self::CreateWorktree,
            Self::GitStatus,
        ]
    }

    pub fn git_commands() -> Vec<Self> {
        vec![
            Self::ListWorktrees,
            Self::CreateWorktree,
            Self::GitStatus,
        ]
    }

    pub fn non_git_commands() -> Vec<Self> {
        vec![
            Self::OpenFile,
            Self::NewSession,
            Self::KillSession,
        ]
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            Self::OpenFile => "Open File",
            Self::NewSession => "New Session",
            Self::KillSession => "Kill Session",
            Self::ListWorktrees => "List Worktrees",
            Self::CreateWorktree => "Create Worktree",
            Self::GitStatus => "Git Status",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::OpenFile => "Open a file in nvim editor window",
            Self::NewSession => "Create a new tmux session",
            Self::KillSession => "Kill the current tmux session",
            Self::ListWorktrees => "List and manage git worktrees",
            Self::CreateWorktree => "Create a new git worktree",
            Self::GitStatus => "Show git diff in popup",
        }
    }

    pub fn search_text(&self) -> String {
        format!("{} {}", self.display_name(), self.description())
    }
}
