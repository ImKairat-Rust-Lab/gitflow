use crate::error::AppError;
use git2::Repository;
use std::path::PathBuf;


pub fn get_repo_root() -> Result<PathBuf, AppError> {
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
        // Fallback to global config if not in a repo
        crate::git::run_git(&["config", "--get", key], false)
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
