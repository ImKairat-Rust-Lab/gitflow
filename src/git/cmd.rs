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

pub fn run_git_interactive(args: &[&str], dry_run: bool) -> Result<(), AppError> {
    if dry_run {
        tracing::info!("[dry-run] git {}", args.join(" "));
        return Ok(());
    }

    tracing::debug!("git (interactive) {}", args.join(" "));

    let status = Command::new("git")
        .args(args)
        .status()
        .map_err(AppError::IoError)?;

    if !status.success() {
        return Err(AppError::GitCommand {
            args: args.join(" "),
            status: status.code().unwrap_or(-1),
            stderr: String::new(),
        });
    }

    Ok(())
}
