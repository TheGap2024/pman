mod actions;
mod app;
mod components;
mod error;
mod integrations;
mod models;
mod tui;

use std::fs;
use std::path::PathBuf;

use clap::{Parser, Subcommand};

use app::{App, View};
use error::{PmanError, Result};
use tui::terminal::install_panic_hook;

#[derive(Parser)]
#[command(name = "pman")]
#[command(about = "Tmux Session/Worktree Manager TUI")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Open session picker (default)
    SessionPicker,
    /// Open command palette
    CommandPalette,
    /// Open worktree picker
    Worktrees,
    /// Find files with fzf
    FindFiles,
    /// Show git diff
    GitDiff,
    /// Install tmux keybindings
    Install,
    /// Uninstall tmux keybindings
    Uninstall,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Install) => install_keybindings(),
        Some(Commands::Uninstall) => uninstall_keybindings(),
        Some(Commands::SessionPicker) | None => {
            check_prerequisites()?;
            install_panic_hook();
            let mut app = App::new(View::SessionPicker)?;
            app.run()
        }
        Some(Commands::CommandPalette) => {
            check_prerequisites()?;
            install_panic_hook();
            let mut app = App::new(View::CommandPalette)?;
            app.run()
        }
        Some(Commands::Worktrees) => {
            check_prerequisites()?;
            install_panic_hook();
            let mut app = App::new(View::WorktreePicker)?;
            app.run()
        }
        Some(Commands::FindFiles) => {
            run_find_files()
        }
        Some(Commands::GitDiff) => {
            run_git_diff()
        }
    }
}

fn check_prerequisites() -> Result<()> {
    let prerequisites = [
        ("tmux", "tmux is required. Install with: brew install tmux"),
        ("nvim", "nvim is required. Install with: brew install neovim"),
        ("fd", "fd is required for file finding. Install with: brew install fd"),
        ("fzf", "fzf is required for fuzzy finding. Install with: brew install fzf"),
        ("bat", "bat is required for file preview. Install with: brew install bat"),
        ("delta", "delta is required for git diffs. Install with: brew install git-delta"),
    ];

    for (cmd, install_msg) in prerequisites {
        if which::which(cmd).is_err() {
            eprintln!("Error: {}", install_msg);
            return Err(PmanError::MissingPrerequisite(cmd.to_string()));
        }
    }

    // Check if running inside tmux
    if std::env::var("TMUX").is_err() {
        eprintln!("Error: pman must be run inside a tmux session");
        return Err(PmanError::Tmux("Not inside tmux".to_string()));
    }

    Ok(())
}

fn install_keybindings() -> Result<()> {
    let home = std::env::var("HOME").map_err(|_| PmanError::Io(std::io::Error::other("HOME not set")))?;
    let tmux_conf_path = PathBuf::from(&home).join(".tmux.conf");

    let pman_path = std::env::current_exe()
        .map_err(|e| PmanError::Io(e))?
        .display()
        .to_string();

    let keybindings = format!(
        r#"
# pman keybindings (managed by pman)
bind s display-popup -E -w 80% -h 80% "{pman_path} session-picker"
bind p display-popup -E -w 80% -h 80% "{pman_path} command-palette"
bind w display-popup -E -w 80% -h 80% "{pman_path} worktrees"
bind f display-popup -E -w 90% -h 90% "{pman_path} find-files"
bind d display-popup -E -w 90% -h 90% "{pman_path} git-diff"
# end pman keybindings
"#
    );

    // Read existing config
    let existing_config = fs::read_to_string(&tmux_conf_path).unwrap_or_default();

    // Check if already installed
    if existing_config.contains("# pman keybindings") {
        println!("pman keybindings are already installed.");
        println!("Run 'pman uninstall' first to update.");
        return Ok(());
    }

    // Append keybindings
    let new_config = format!("{}\n{}", existing_config.trim_end(), keybindings);
    fs::write(&tmux_conf_path, new_config)?;

    println!("✓ pman keybindings installed to ~/.tmux.conf");
    println!();
    println!("Reload tmux config with:");
    println!("  tmux source-file ~/.tmux.conf");
    println!();
    println!("Keybindings:");
    println!("  Prefix + s  →  Sessions");
    println!("  Prefix + p  →  Command Palette");
    println!("  Prefix + w  →  Worktrees");
    println!("  Prefix + f  →  Find Files");
    println!("  Prefix + d  →  Git Diff");

    Ok(())
}

fn uninstall_keybindings() -> Result<()> {
    let home = std::env::var("HOME").map_err(|_| PmanError::Io(std::io::Error::other("HOME not set")))?;
    let tmux_conf_path = PathBuf::from(&home).join(".tmux.conf");

    let existing_config = fs::read_to_string(&tmux_conf_path).unwrap_or_default();

    // Remove pman section
    let mut new_lines = Vec::new();
    let mut in_pman_section = false;

    for line in existing_config.lines() {
        if line.contains("# pman keybindings (managed by pman)") {
            in_pman_section = true;
            continue;
        }
        if line.contains("# end pman keybindings") {
            in_pman_section = false;
            continue;
        }
        if !in_pman_section {
            new_lines.push(line);
        }
    }

    let new_config = new_lines.join("\n");
    fs::write(&tmux_conf_path, new_config)?;

    println!("✓ pman keybindings removed from ~/.tmux.conf");
    println!();
    println!("Reload tmux config with:");
    println!("  tmux source-file ~/.tmux.conf");

    Ok(())
}

fn run_find_files() -> Result<()> {
    use std::process::Command;

    let status = Command::new("sh")
        .arg("-c")
        .arg(r#"file=$(fd --type f --hidden --exclude .git | fzf --preview 'bat --color=always --style=numbers --line-range=:500 {}') && [ -n "$file" ] && nvim "$file""#)
        .status()
        .map_err(|e| PmanError::Io(e))?;

    if !status.success() {
        // User cancelled fzf, not an error
    }
    Ok(())
}

fn run_git_diff() -> Result<()> {
    use std::process::Command;

    let status = Command::new("sh")
        .arg("-c")
        .arg("git diff HEAD | delta")
        .status()
        .map_err(|e| PmanError::Io(e))?;

    if !status.success() {
        eprintln!("git diff failed");
    }
    Ok(())
}
