# pman

**Run 10 AI coding agents in parallel. Switch between them in 2 keystrokes.**

Claude Code, Codex, Cursor, Aider... AI agents are powerful, but slow. Why wait for one when you can run ten? pman turns tmux into a multi-agent cockpit—spawn agents in separate sessions, give each its own git worktree, and jump between them instantly with fuzzy search.

```
Session 1: Claude implementing auth      [████████░░] 80%
Session 2: Codex writing tests           [██████░░░░] 60%
Session 3: Claude refactoring API        [████░░░░░░] 40%
Session 4: Aider fixing bugs             [██░░░░░░░░] 20%

> Press Prefix+s to switch... _
```

## Why pman?

- **Parallel Execution** - Run multiple AI agents simultaneously, each in its own tmux session
- **Isolated Workspaces** - Each agent gets its own git worktree, no merge conflicts
- **Instant Context Switch** - Fuzzy search across all sessions, switch in milliseconds
- **Zero Overhead** - Lightweight TUI, no electron, no bloat

## Installation

```bash
brew install golbin/tap/pman
```

## Quick Start

```bash
# 1. Install tmux keybindings
pman install && tmux source-file ~/.tmux.conf

# 2. Inside tmux, press Prefix+s (usually Ctrl+b, then s)
#    to open session picker and start switching!
```

## Usage Scenarios

### Scenario 1: Parallel Feature Development

You have 3 features to implement. Instead of doing them sequentially:

```bash
# Create worktrees for each feature
# (Use Prefix+p → Create Worktree)
feature/auth
feature/payment
feature/notifications

# Spawn Claude Code in each worktree
tmux new-session -s auth -c ~/project-auth
tmux new-session -s payment -c ~/project-payment
tmux new-session -s notifications -c ~/project-notifications

# Now run your AI agent in each session
# Switch between them with Prefix+s to monitor progress
```

### Scenario 2: Agent A/B Testing

Compare how different agents solve the same problem:

```bash
# Session: claude-refactor    → Claude Code refactoring your API
# Session: codex-refactor     → Codex doing the same task
# Session: aider-refactor     → Aider's approach

# Use Prefix+s to rapidly compare outputs
# Pick the best solution, discard the rest
```

### Scenario 3: Review While Agents Work

```bash
# Session: agent-working      → AI implementing feature
# Session: review             → You reviewing previous AI output
# Session: manual-fixes       → You fixing edge cases AI missed

# Agents don't block you. You don't block agents.
# True parallel workflow.
```

### Scenario 4: Large Codebase Refactoring

Split a massive refactoring task across multiple agents:

```bash
# Session: refactor-models    → Agent refactoring data models
# Session: refactor-api       → Agent updating API endpoints
# Session: refactor-tests     → Agent fixing broken tests
# Session: refactor-docs      → Agent updating documentation

# Each agent works on a separate worktree
# Merge when all complete
```

## Keybindings

### Tmux (after `pman install`)

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
| `Esc` | Close |

### Command Palette

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
| `Enter` | Switch to worktree |
| `n` | New worktree |
| `d` | Delete worktree |
| `m` | Merge to main |

### Navigation (All Views)

| Key | Action |
|-----|--------|
| `Ctrl+k` / `Up` | Move up |
| `Ctrl+j` / `Down` | Move down |
| `PageUp` / `PageDown` | Page navigation |

## Prerequisites

```bash
brew install tmux neovim git-delta
```

## Uninstall

```bash
pman uninstall
tmux source-file ~/.tmux.conf
brew uninstall pman
```

## License

MIT
