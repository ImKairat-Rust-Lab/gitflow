use std::process::{Command, Stdio};
use crate::error::AppError;


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
        .map_err(|e| AppError::IoError(e))?;

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

pub fn is_git_repo() -> bool {
    run_git(&["rev-parse", "--git-dir"], false).is_ok()
}

pub fn current_branch() -> Result<String, AppError> {
    run_git(&["rev-parse", "--abbrev-ref", "HEAD"], false)
}

pub fn is_working_tree_clean() -> Result<bool, AppError> {
    let output = run_git(&["status", "--porcelain"], false)?;
    Ok(output.is_empty())
}

pub fn branch_exists(name: &str) -> bool {
    run_git(&["rev-parse", "--verify", &format!("refs/heads/{}", name)], false).is_ok()
}

pub fn git_config_get(key: &str) -> Option<String> {
    run_git(&["config", "--get", key], false).ok().filter(|s| !s.is_empty())
}
