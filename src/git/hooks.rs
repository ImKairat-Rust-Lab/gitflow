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
use crate::git::repo::get_repo_root;
use std::process::Command;

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
