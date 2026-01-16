#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PaletteCommand {
    // Sessions
    ListSessions,
    NewSession,
    KillSession,
    // Worktrees
    ListWorktrees,
    CreateWorktree,
    // Files
    FindFiles,
    ListBuffers,
    // Git
    GitDiff,
}

impl PaletteCommand {
    pub fn all() -> Vec<Self> {
        vec![
            Self::ListSessions,
            Self::NewSession,
            Self::KillSession,
            Self::ListWorktrees,
            Self::CreateWorktree,
            Self::FindFiles,
            Self::ListBuffers,
            Self::GitDiff,
        ]
    }

    pub fn non_git_commands() -> Vec<Self> {
        vec![
            Self::ListSessions,
            Self::NewSession,
            Self::KillSession,
            Self::FindFiles,
            Self::ListBuffers,
        ]
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            Self::ListSessions => "List Sessions",
            Self::NewSession => "New Session",
            Self::KillSession => "Kill Session",
            Self::ListWorktrees => "List Worktrees",
            Self::CreateWorktree => "Create Worktree",
            Self::FindFiles => "Find Files",
            Self::ListBuffers => "List Buffers",
            Self::GitDiff => "Git Diff",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::ListSessions => "Switch between tmux sessions",
            Self::NewSession => "Create a new tmux session",
            Self::KillSession => "Kill the current tmux session",
            Self::ListWorktrees => "List and manage git worktrees",
            Self::CreateWorktree => "Create a new git worktree",
            Self::FindFiles => "Find and open files with fzf",
            Self::ListBuffers => "List open buffers in nvim",
            Self::GitDiff => "Show git diff in popup",
        }
    }

    pub fn search_text(&self) -> String {
        format!("{} {}", self.display_name(), self.description())
    }
}
