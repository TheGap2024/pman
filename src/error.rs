use thiserror::Error;

#[derive(Error, Debug)]
pub enum PmanError {
    #[error("Missing prerequisite: {0}")]
    MissingPrerequisite(String),

    #[error("Tmux error: {0}")]
    Tmux(String),

    #[error("Git error: {0}")]
    Git(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Terminal error: {0}")]
    Terminal(String),

    #[error("Not in a git repository")]
    NotGitRepo,

    #[error("Worktree has uncommitted changes")]
    UncommittedChanges,

    #[error("Operation cancelled")]
    Cancelled,
}

pub type Result<T> = std::result::Result<T, PmanError>;
