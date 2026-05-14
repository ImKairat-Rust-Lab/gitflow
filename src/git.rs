use crate::error::AppError;
use std::process::{Command, Stdio};

pub fn run_git(args: &[&str], dry_run: bool) -> Result<String, AppError> {
    if dry_run {
        tracing::info!("[dry-run] git {}", args.join(" "));
        return Ok(String::new());
    }

    tracing::debug!("git {}", args.join(" "));

    let output = Command::new("git")
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .map_err(AppError::IoError)?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(AppError::GitCommand {
            args: args.join(" "),
            status: output.status.code().unwrap_or(-1),
            stderr: stderr.trim().to_string(),
        });
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    Ok(stdout.trim().to_string())
}

pub fn run_git_silent(args: &[&str], dry_run: bool) -> Result<(), AppError> {
    run_git(args, dry_run)?;
    Ok(())
}

use git2::Repository;

#[allow(dead_code)]
pub fn is_git_repo() -> bool {
    Repository::discover(".").is_ok()
}

pub fn get_repo_root() -> Result<std::path::PathBuf, AppError> {
    let repo = Repository::discover(".")
        .map_err(|e| AppError::Config(format!("Not a git repository: {}", e)))?;
    Ok(repo
        .workdir()
        .ok_or_else(|| AppError::Config("No working directory found".to_string()))?
        .to_path_buf())
}

pub fn current_branch() -> Result<String, AppError> {
    let repo = Repository::discover(".")
        .map_err(|e| AppError::Config(format!("Not a git repository: {}", e)))?;
    let head = repo
        .head()
        .map_err(|e| AppError::Config(format!("Could not get HEAD: {}", e)))?;
    Ok(head.shorthand().unwrap_or_default().to_string())
}

pub fn is_working_tree_clean() -> Result<bool, AppError> {
    let repo = Repository::discover(".")
        .map_err(|e| AppError::Config(format!("Not a git repository: {}", e)))?;
    let mut opts = git2::StatusOptions::new();
    opts.include_untracked(true);
    let statuses = repo
        .statuses(Some(&mut opts))
        .map_err(|e| AppError::Config(format!("Could not get status: {}", e)))?;
    Ok(statuses.is_empty())
}

pub fn branch_exists(name: &str) -> bool {
    let repo = Repository::discover(".").ok();
    if let Some(repo) = repo {
        repo.find_branch(name, git2::BranchType::Local).is_ok()
    } else {
        false
    }
}

pub fn git_config_get(key: &str) -> Option<String> {
    let repo = Repository::discover(".").ok();
    if let Some(repo) = repo {
        let config = repo.config().ok()?;
        config.get_string(key).ok()
    } else {
        // Fallback to global config if not in a repo (e.g. during init in some cases)
        run_git(&["config", "--get", key], false)
            .ok()
            .filter(|s| !s.is_empty())
    }
}

pub fn list_branches(prefix: &str) -> Result<Vec<String>, AppError> {
    let repo = Repository::discover(".")
        .map_err(|e| AppError::Config(format!("Not a git repository: {}", e)))?;
    let branches = repo
        .branches(Some(git2::BranchType::Local))
        .map_err(|e| AppError::Config(format!("Could not list branches: {}", e)))?;

    let mut result = Vec::new();
    for branch in branches {
        let (branch, _) = branch.map_err(|e| AppError::Config(format!("Branch error: {}", e)))?;
        if let Some(name) = branch
            .name()
            .map_err(|e| AppError::Config(format!("Name error: {}", e)))?
            && name.starts_with(prefix) {
                result.push(name.to_string());
            }
    }
    Ok(result)
}

pub fn run_hook(name: &str, args: &[&str]) -> Result<(), AppError> {
    let root = get_repo_root()?;
    let hook_path = root.join(".git").join("hooks").join(name);

    if hook_path.exists() {
        tracing::info!("Running hook: {}", name);
        let status = Command::new(&hook_path)
            .args(args)
            .status()
            .map_err(AppError::IoError)?;

        if !status.success() {
            return Err(AppError::Config(format!(
                "Hook '{}' failed with exit code {}",
                name,
                status.code().unwrap_or(-1)
            )));
        }
    }
    Ok(())
}
