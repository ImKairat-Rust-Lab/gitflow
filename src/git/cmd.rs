// Copyright (C) 2026 Kairat Kubanychbek uulu <https://github.com/ImKairat>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

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
