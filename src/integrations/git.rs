use std::path::{Path, PathBuf};
use std::process::Command;

use crate::error::{PmanError, Result};
use crate::models::GitWorktree;

pub struct GitClient {
    repo_root: PathBuf,
}

impl GitClient {
    pub fn new(path: &Path) -> Result<Self> {
        let repo_root = Self::find_repo_root(path)?;
        Ok(Self { repo_root })
    }

    pub fn is_git_repo(path: &Path) -> bool {
        Self::find_repo_root(path).is_ok()
    }

    fn find_repo_root(path: &Path) -> Result<PathBuf> {
        let output = Command::new("git")
            .args(["rev-parse", "--show-toplevel"])
            .current_dir(path)
            .output()
            .map_err(|e| PmanError::Git(e.to_string()))?;

        if !output.status.success() {
            return Err(PmanError::NotGitRepo);
        }

        Ok(PathBuf::from(
            String::from_utf8_lossy(&output.stdout).trim(),
        ))
    }

    pub fn repo_root(&self) -> &Path {
        &self.repo_root
    }

    pub fn list_worktrees(&self) -> Result<Vec<GitWorktree>> {
        let output = Command::new("git")
            .args(["worktree", "list", "--porcelain"])
            .current_dir(&self.repo_root)
            .output()
            .map_err(|e| PmanError::Git(e.to_string()))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(PmanError::Git(stderr.to_string()));
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let main_branch = self.get_main_branch()?;
        let mut worktrees = Vec::new();
        let mut current_worktree: Option<GitWorktree> = None;

        for line in stdout.lines() {
            if line.starts_with("worktree ") {
                if let Some(wt) = current_worktree.take() {
                    worktrees.push(wt);
                }
                let path = PathBuf::from(line.strip_prefix("worktree ").unwrap());
                current_worktree = Some(GitWorktree {
                    path,
                    branch: String::new(),
                    is_main: false,
                    commit_hash: String::new(),
                    has_changes: false,
                });
            } else if line.starts_with("HEAD ") {
                if let Some(ref mut wt) = current_worktree {
                    wt.commit_hash = line.strip_prefix("HEAD ").unwrap().to_string();
                }
            } else if line.starts_with("branch ") {
                if let Some(ref mut wt) = current_worktree {
                    let branch = line
                        .strip_prefix("branch refs/heads/")
                        .unwrap_or(line.strip_prefix("branch ").unwrap());
                    wt.branch = branch.to_string();
                    wt.is_main = branch == main_branch;
                }
            } else if line == "detached" {
                if let Some(ref mut wt) = current_worktree {
                    wt.branch = "(detached)".to_string();
                }
            }
        }

        if let Some(wt) = current_worktree {
            worktrees.push(wt);
        }

        // Check for uncommitted changes in each worktree
        for wt in &mut worktrees {
            wt.has_changes = self.has_uncommitted_changes(&wt.path)?;
        }

        Ok(worktrees)
    }

    pub fn has_uncommitted_changes(&self, path: &Path) -> Result<bool> {
        let output = Command::new("git")
            .args(["status", "--porcelain"])
            .current_dir(path)
            .output()
            .map_err(|e| PmanError::Git(e.to_string()))?;

        Ok(!output.stdout.is_empty())
    }

    pub fn get_main_branch(&self) -> Result<String> {
        // Try to get the default branch name
        let output = Command::new("git")
            .args(["symbolic-ref", "refs/remotes/origin/HEAD"])
            .current_dir(&self.repo_root)
            .output();

        if let Ok(output) = output {
            if output.status.success() {
                let branch = String::from_utf8_lossy(&output.stdout)
                    .trim()
                    .strip_prefix("refs/remotes/origin/")
                    .unwrap_or("main")
                    .to_string();
                return Ok(branch);
            }
        }

        // Fallback: check if main or master exists
        for branch in ["main", "master"] {
            let output = Command::new("git")
                .args(["rev-parse", "--verify", branch])
                .current_dir(&self.repo_root)
                .output()
                .map_err(|e| PmanError::Git(e.to_string()))?;

            if output.status.success() {
                return Ok(branch.to_string());
            }
        }

        Ok("main".to_string())
    }

    pub fn create_worktree(&self, branch_name: &str) -> Result<PathBuf> {
        let worktree_path = self.repo_root.parent().unwrap_or(&self.repo_root).join(branch_name);

        let output = Command::new("git")
            .args([
                "worktree",
                "add",
                "-b",
                branch_name,
                worktree_path.to_str().unwrap(),
            ])
            .current_dir(&self.repo_root)
            .output()
            .map_err(|e| PmanError::Git(e.to_string()))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(PmanError::Git(stderr.to_string()));
        }

        Ok(worktree_path)
    }

    pub fn delete_worktree(&self, path: &Path) -> Result<()> {
        if self.has_uncommitted_changes(path)? {
            return Err(PmanError::UncommittedChanges);
        }

        let output = Command::new("git")
            .args(["worktree", "remove", path.to_str().unwrap()])
            .current_dir(&self.repo_root)
            .output()
            .map_err(|e| PmanError::Git(e.to_string()))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(PmanError::Git(stderr.to_string()));
        }

        Ok(())
    }

    pub fn merge_to_main(&self, worktree_path: &Path, branch: &str) -> Result<()> {
        let main_branch = self.get_main_branch()?;

        // Checkout main
        let output = Command::new("git")
            .args(["checkout", &main_branch])
            .current_dir(&self.repo_root)
            .output()
            .map_err(|e| PmanError::Git(e.to_string()))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(PmanError::Git(stderr.to_string()));
        }

        // Merge branch
        let output = Command::new("git")
            .args(["merge", branch])
            .current_dir(&self.repo_root)
            .output()
            .map_err(|e| PmanError::Git(e.to_string()))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(PmanError::Git(stderr.to_string()));
        }

        // Delete worktree
        self.delete_worktree(worktree_path)?;

        // Delete branch
        let output = Command::new("git")
            .args(["branch", "-d", branch])
            .current_dir(&self.repo_root)
            .output()
            .map_err(|e| PmanError::Git(e.to_string()))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(PmanError::Git(stderr.to_string()));
        }

        Ok(())
    }

    pub fn get_diff(&self, path: &Path) -> Result<String> {
        let output = Command::new("git")
            .args(["diff", "HEAD"])
            .current_dir(path)
            .output()
            .map_err(|e| PmanError::Git(e.to_string()))?;

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
}
