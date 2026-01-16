# pman

Tmux Session/Worktree Manager TUI

A terminal user interface for managing tmux sessions and git worktrees with fuzzy search.

## Features

- **Session Picker**: Fuzzy search and manage tmux sessions
- **Command Palette**: Quick access to common actions
- **Git Worktree Integration**: Manage git worktrees directly from tmux

## Installation

### Homebrew (macOS)

```bash
brew tap golbin/tap
brew install pman
```

Or in a single command:

```bash
brew install golbin/tap/pman
```

### From Source

```bash
cargo install --path .
```

## Prerequisites

- tmux
- neovim
- git-delta (for git diffs)

Install prerequisites with Homebrew:

```bash
brew install tmux neovim git-delta
```

## Setup

After installation, set up tmux keybindings:

```bash
pman install
tmux source-file ~/.tmux.conf
```

This adds the following keybindings:

| Keybinding | Action |
|------------|--------|
| `Prefix + s` | Session Picker |
| `Prefix + p` | Command Palette |

## Usage

### Commands

```bash
pman                    # Open session picker (default)
pman session-picker     # Open session picker
pman command-palette    # Open command palette
pman install            # Install tmux keybindings
pman uninstall          # Remove tmux keybindings
```

### Session Picker

- Type to fuzzy search sessions
- `Enter` to switch to selected session
- `Ctrl+d` to delete session
- `Esc` to close

### Command Palette

- Type to fuzzy search commands
- `Enter` to execute selected command
- `Esc` to close

## Uninstall

```bash
pman uninstall
tmux source-file ~/.tmux.conf
brew uninstall pman
```

## License

MIT
