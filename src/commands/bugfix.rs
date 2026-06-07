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
use crate::commands::common::CommonFinishFlags;
use crate::config::GitFlowConfig;
use crate::error::AppError;
use crate::git;
use clap::Subcommand;
use tracing::info;

#[derive(Subcommand, Debug)]
pub enum BugfixAction {
    Start {
        name: String,
        base: Option<String>,
    },
    Finish {
        #[command(flatten)]
        common: CommonFinishFlags,

        #[arg(short = 'r', long)]
        rebase: bool,

        name: String,
    },
    Publish {
        name: String,
    },
    Track {
        name: String,
    },
    Diff {
        name: Option<String>,
    },
    Rebase {
        #[arg(short = 'i', long)]
        interactive: bool,
        #[arg(short = 'p', long)]
        preserve_merges: bool,
        name: Option<String>,
    },
    Checkout {
        name: Option<String>,
    },
    Pull {
        name: String,
    },
    Delete {
        #[arg(short = 'f', long)]
        force: bool,
        #[arg(short = 'r', long)]
        remote: bool,
        name: String,
    },
    List {
        #[arg(short = 'r', long)]
        remote: bool,
    },
}

impl Execute for BugfixAction {
    fn execute(self) -> Result<(), AppError> {
        let config = GitFlowConfig::load()?;

        match self {
            Self::Start { name, base } => {
                let base = base.unwrap_or_else(|| config.develop_branch.clone());
                let branch_name = format!("{}{}", config.bugfix_prefix, name);

                git::run_hook("pre-flow-bugfix-start", &[&name, &base])?;

                info!("Starting bugfix: {}", name);
                git::run_git_silent(&["checkout", "-b", &branch_name, &base], false)?;

                git::run_hook("post-flow-bugfix-start", &[&name, &base])?;
            }
            Self::Finish {
                name,
                common,
                rebase,
            } => {
                let branch_name = format!("{}{}", config.bugfix_prefix, name);

                // Workspace safety check
                if !git::is_working_tree_clean()? {
                    return Err(AppError::Config(
                        "Working tree is not clean. Commit or stash your changes first."
                            .to_string(),
                    ));
                }

                git::run_hook("pre-flow-bugfix-finish", &[&name])?;

                if common.fetch {
                    git::run_git_silent(&["fetch", "origin"], false)?;
                }

                git::run_git_silent(&["checkout", &config.develop_branch], false)?;

                if rebase {
                    git::run_git_silent(&["rebase", &branch_name], false)?;
                } else {
                    git::run_git_silent(&["merge", "--no-ff", &branch_name], false)?;
                }

                if !common.keep {
                    git::run_git_silent(&["branch", "-d", &branch_name], false)?;
                }

                info!(
                    "Bugfix '{}' finished and merged into '{}'",
                    name, config.develop_branch
                );

                git::run_hook("post-flow-bugfix-finish", &[&name])?;
            }
            Self::Publish { name } => {
                let branch_name = format!("{}{}", config.bugfix_prefix, name);
                git::run_git_silent(&["push", "-u", "origin", &branch_name], false)?;
                info!("Bugfix '{}' published to origin", name);
            }
            Self::Track { name } => {
                let branch_name = format!("{}{}", config.bugfix_prefix, name);
                git::run_git_silent(
                    &[
                        "checkout",
                        "-b",
                        &branch_name,
                        &format!("origin/{}", branch_name),
                    ],
                    false,
                )?;
                info!("Now tracking bugfix '{}' from origin", name);
            }
            Self::Diff { name } => {
                let current = git::current_branch()?;
                let bugfix_name =
                    name.unwrap_or_else(|| current.replace(&config.bugfix_prefix, ""));
                let branch_name = format!("{}{}", config.bugfix_prefix, bugfix_name);
                let base = git::run_git(
                    &[
                        "config",
                        "--get",
                        &format!("gitflow.branch.{}.base", branch_name),
                    ],
                    false,
                )
                .unwrap_or_else(|_| config.develop_branch.clone());

                git::run_git_interactive(&["diff", &format!("{}..{}", base, branch_name)], false)?;
            }
            Self::Rebase {
                interactive,
                preserve_merges,
                name,
            } => {
                let current = git::current_branch()?;
                let bugfix_name =
                    name.unwrap_or_else(|| current.replace(&config.bugfix_prefix, ""));
                let branch_name = format!("{}{}", config.bugfix_prefix, bugfix_name);
                let base = git::run_git(
                    &[
                        "config",
                        "--get",
                        &format!("gitflow.branch.{}.base", branch_name),
                    ],
                    false,
                )
                .unwrap_or_else(|_| config.develop_branch.clone());

                git::run_git_silent(&["checkout", &branch_name], false)?;

                let mut args = vec!["rebase"];
                if interactive {
                    args.push("-i");
                }
                if preserve_merges {
                    args.push("-p");
                }
                args.push(&base);

                git::run_git_interactive(&args, false)?;
            }
            Self::Checkout { name } => {
                if let Some(n) = name {
                    let branch_name = format!("{}{}", config.bugfix_prefix, n);
                    git::run_git_silent(&["checkout", &branch_name], false)?;
                } else {
                    return Err(AppError::Config("Branch name is required".to_string()));
                }
            }
            Self::Pull { name } => {
                let branch_name = format!("{}{}", config.bugfix_prefix, name);
                git::run_git_silent(&["checkout", &branch_name], false)?;
                git::run_git_silent(&["pull", "origin", &branch_name], false)?;
                info!("Bugfix '{}' updated from origin", name);
            }
            Self::Delete {
                name,
                force,
                remote,
            } => {
                let branch_name = format!("{}{}", config.bugfix_prefix, name);
                if remote {
                    git::run_git_silent(&["push", "origin", "--delete", &branch_name], false)?;
                    info!("Remote branch '{}' deleted", branch_name);
                }
                let delete_flag = if force { "-D" } else { "-d" };
                git::run_git_silent(&["branch", delete_flag, &branch_name], false)?;
                info!("Local branch '{}' deleted", branch_name);
            }
            Self::List { remote } => {
                let prefix = if remote {
                    format!("remotes/origin/{}", config.bugfix_prefix)
                } else {
                    config.bugfix_prefix.clone()
                };
                let branches = git::list_branches(&prefix)?;
                if branches.is_empty() {
                    info!("No bugfix branches found.");
                } else {
                    info!("Bugfix branches:");
                    for b in branches {
                        println!("  {}", b.trim_start_matches(&prefix));
                    }
                }
            }
        }
        Ok(())
    }
}
