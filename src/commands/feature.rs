use crate::cli::Execute;
use crate::config::GitFlowConfig;
use crate::error::AppError;
use crate::git;
use crate::commands::common::CommonFinishFlags;
use clap::Subcommand;
use tracing::info;

#[derive(Subcommand, Debug)]
pub enum FeatureAction {
    /// Start a new feature branch
    Start {
        name: String,
        base: Option<String>,
    },
    /// Finish a feature branch
    Finish {
        #[command(flatten)]
        common: CommonFinishFlags,

        /// Rebase instead of merge
        #[arg(short = 'r', long, conflicts_with = "squash")]
        rebase: bool,

        /// Squash feature during merge
        #[arg(short = 'S', long, conflicts_with = "rebase")]
        squash: bool,

        /// Force delete feature branch after finish
        #[arg(short = 'D', long)]
        force_delete: bool,

        name: String,
    },
    Publish {
        name: String,
    },
    Track {
        name: String,
    },
    /// Pull changes from remote feature branch
    Pull {
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
    /// List all feature branches
    List {
        /// List remote branches as well
        #[arg(short = 'r', long)]
        remote: bool,
    },
    Delete {
        #[arg(short = 'f', long)]
        force: bool,
        #[arg(short = 'r', long)]
        remote: bool,
        name: String,
    },
}

impl Execute for FeatureAction {
    fn execute(self) -> Result<(), AppError> {
        let config = GitFlowConfig::load()?;

        match self {
            Self::Start { name, base } => {
                let base = base.unwrap_or_else(|| config.develop_branch.clone());
                let branch_name = format!("{}{}", config.feature_prefix, name);

                git::run_hook("pre-flow-feature-start", &[&name, &base])?;

                info!("Starting feature: {}", name);
                git::run_git_silent(&["checkout", "-b", &branch_name, &base], false)?;
                info!("Switched to a new branch '{}'", branch_name);

                git::run_hook("post-flow-feature-start", &[&name, &base])?;
            }
            Self::Finish {
                name,
                common,
                rebase,
                squash,
                force_delete,
            } => {
                let branch_name = format!("{}{}", config.feature_prefix, name);

                // Workspace safety check
                if !git::is_working_tree_clean()? {
                    return Err(AppError::Config(
                        "Working tree is not clean. Commit or stash your changes first."
                            .to_string(),
                    ));
                }

                git::run_hook("pre-flow-feature-finish", &[&name])?;

                if common.fetch {
                    git::run_git_silent(&["fetch", "origin"], false)?;
                }

                // Checkout develop
                git::run_git_silent(&["checkout", &config.develop_branch], false)?;

                // Merge/Rebase/Squash
                if rebase {
                    git::run_git_silent(&["rebase", &branch_name], false)?;
                } else if squash {
                    git::run_git_silent(&["merge", "--squash", &branch_name], false)?;
                    git::run_git_silent(
                        &["commit", "-m", &format!("Finish feature {}", name)],
                        false,
                    )?;
                } else {
                    git::run_git_silent(&["merge", "--no-ff", &branch_name], false)?;
                }

                // Delete branch
                if !common.keep {
                    let delete_flag = if force_delete { "-D" } else { "-d" };
                    git::run_git_silent(&["branch", delete_flag, &branch_name], false)?;
                }

                info!(
                    "Feature '{}' finished and merged into '{}'",
                    name, config.develop_branch
                );

                git::run_hook("post-flow-feature-finish", &[&name])?;
            }
            Self::Publish { name } => {
                let branch_name = format!("{}{}", config.feature_prefix, name);
                git::run_hook("pre-flow-feature-publish", &[&name])?;

                git::run_git_silent(&["push", "-u", "origin", &branch_name], false)?;
                info!("Feature '{}' published to origin", name);

                git::run_hook("post-flow-feature-publish", &[&name])?;
            }
            Self::Track { name } => {
                let branch_name = format!("{}{}", config.feature_prefix, name);
                git::run_git_silent(
                    &[
                        "checkout",
                        "-b",
                        &branch_name,
                        &format!("origin/{}", branch_name),
                    ],
                    false,
                )?;
                info!("Now tracking feature '{}' from origin", name);
            }
            Self::Pull { name } => {
                let branch_name = format!("{}{}", config.feature_prefix, name);
                git::run_git_silent(&["checkout", &branch_name], false)?;
                git::run_git_silent(&["pull", "origin", &branch_name], false)?;
                info!("Feature '{}' updated from origin", name);
            }
            Self::List { remote } => {
                let prefix = if remote {
                    format!("remotes/origin/{}", config.feature_prefix)
                } else {
                    config.feature_prefix.clone()
                };
                let branches = git::list_branches(&prefix)?;
                if branches.is_empty() {
                    info!("No feature branches found.");
                } else {
                    info!("Feature branches:");
                    for b in branches {
                        println!("  {}", b.trim_start_matches(&prefix));
                    }
                }
            }
            Self::Delete {
                name,
                force,
                remote,
            } => {
                let branch_name = format!("{}{}", config.feature_prefix, name);

                if remote {
                    git::run_git_silent(&["push", "origin", "--delete", &branch_name], false)?;
                    info!("Remote branch '{}' deleted", branch_name);
                }

                let delete_flag = if force { "-D" } else { "-d" };
                git::run_git_silent(&["branch", delete_flag, &branch_name], false)?;
                info!("Local branch '{}' deleted", branch_name);
            }
            Self::Diff { name } => {
                let current = git::current_branch()?;
                let feature_name = name.unwrap_or_else(|| current.replace(&config.feature_prefix, ""));
                let branch_name = format!("{}{}", config.feature_prefix, feature_name);
                let base = git::run_git(&["config", "--get", &format!("gitflow.branch.{}.base", branch_name)], false)
                    .unwrap_or_else(|_| config.develop_branch.clone());

                git::run_git_interactive(&["diff", &format!("{}..{}", base, branch_name)], false)?;
            }
            Self::Rebase { interactive, preserve_merges, name } => {
                let current = git::current_branch()?;
                let feature_name = name.unwrap_or_else(|| current.replace(&config.feature_prefix, ""));
                let branch_name = format!("{}{}", config.feature_prefix, feature_name);
                let base = git::run_git(&["config", "--get", &format!("gitflow.branch.{}.base", branch_name)], false)
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
                    let branch_name = format!("{}{}", config.feature_prefix, n);
                    git::run_git_silent(&["checkout", &branch_name], false)?;
                } else {
                    return Err(AppError::Config("Branch name is required".to_string()));
                }
            }
        }
        Ok(())
    }
}
