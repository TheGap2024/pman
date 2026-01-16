# pman

**Switch tmux sessions and git worktrees in 2 keystrokes.**

Tired of typing long session names? Losing track of git worktrees across projects? pman is a fast TUI that lets you fuzzy-search and jump between your dev environments instantly.

## Features

- **Session Picker** (`Prefix + s`) - Fuzzy search all tmux sessions
- **Command Palette** (`Prefix + p`) - Quick access to common actions
- **Git Worktree Management** - Create, switch, merge, and delete worktrees

## Installation

### Homebrew (macOS)

```bash
brew install golbin/tap/pman
```

### From Source

```bash
cargo install --path .
```

### Prerequisites

```bash
brew install tmux neovim git-delta
```

## Quick Start

```bash
# 1. Install tmux keybindings
pman install

# 2. Reload tmux config
tmux source-file ~/.tmux.conf

# 3. Inside tmux, press Prefix + s to open session picker
#    (Prefix is usually Ctrl+b)
```

## Keybindings

### Tmux Keybindings (after `pman install`)

| Key | Action |
|-----|--------|
| `Prefix + s` | Session Picker |
| `Prefix + p` | Command Palette |

### Session Picker

| Key | Action |
|-----|--------|
| Type | Fuzzy search |
| `Enter` | Switch to session |
| `n` | New session |
| `d` | Delete session |
| `Esc` | Close (or clear query) |

### Command Palette

| Key | Action |
|-----|--------|
| Type | Fuzzy search |
| `Enter` | Execute command |
| `Esc` | Close |

Available commands:
- **Open File** - Open file in nvim
- **New Session** - Create tmux session
- **Kill Session** - Kill current session
- **List Worktrees** - Manage git worktrees
- **Create Worktree** - Create new worktree
- **Git Status** - Show git diff

### Worktree Picker

| Key | Action |
|-----|--------|
| Type | Fuzzy search |
| `Enter` | Switch to worktree |
| `n` | New worktree |
| `d` | Delete worktree |
| `m` | Merge to main |
| `Esc` | Close |

## Navigation (All Views)

| Key | Action |
|-----|--------|
| `Ctrl+k` / `Up` | Move up |
| `Ctrl+j` / `Down` | Move down |
| `PageUp` / `PageDown` | Page navigation |

## Uninstall

```bash
pman uninstall
tmux source-file ~/.tmux.conf
brew uninstall pman
```

## License

MIT
