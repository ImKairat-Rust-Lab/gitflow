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

use crate::cli::Execute;
use crate::config::GitFlowConfig;
use crate::error::AppError;
use crate::git;
use clap::Subcommand;
use tracing::info;

#[derive(Subcommand, Debug)]
pub enum SupportAction {
    /// Start a new support branch
    Start { version: String, base: String },
    List {
        #[arg(short = 'v', long)]
        verbose: bool,
    },
}

impl Execute for SupportAction {
    fn execute(self) -> Result<(), AppError> {
        let config = GitFlowConfig::load()?;

        match self {
            Self::Start { version, base } => {
                let branch_name = format!("{}{}", config.support_prefix, version);

                git::run_hook("pre-flow-support-start", &[&version, &base])?;

                info!("Starting support branch: {}", version);
                git::run_git_silent(&["checkout", "-b", &branch_name, &base], false)?;

                git::run_hook("post-flow-support-start", &[&version, &base])?;
            }
            Self::List { verbose: _ } => {
                let prefix = config.support_prefix.clone();
                let branches = git::list_branches(&prefix)?;
                if branches.is_empty() {
                    info!("No support branches found.");
                } else {
                    info!("Support branches:");
                    for b in branches {
                        println!("  {}", b.trim_start_matches(&prefix));
                    }
                }
            }
        }
        Ok(())
    }
}
