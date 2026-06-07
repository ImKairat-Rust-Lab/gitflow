use crate::cli::Execute;
use crate::config::GitFlowConfig;
use crate::error::AppError;
use crate::git;
use crate::commands::common::{CommonFinishFlags, TaggingFlags};
use clap::Subcommand;
use tracing::info;

#[derive(Subcommand, Debug)]
pub enum HotfixAction {
    Start {
        version: String,
        base: Option<String>,
    },
    Finish {
        #[command(flatten)]
        common: CommonFinishFlags,

        #[command(flatten)]
        tagging: TaggingFlags,

        #[arg(short = 'p', long)]
        push: bool,

        version: String,
    },
    Publish {
        version: String,
    },
    Track {
        version: String,
    },
    Delete {
        #[arg(short = 'f', long)]
        force: bool,
        #[arg(short = 'r', long)]
        remote: bool,
        version: String,
    },
    List {
        #[arg(short = 'v', long)]
        verbose: bool,
    },
    Rebase {
        #[arg(short = 'i', long)]
        interactive: bool,
        #[arg(short = 'p', long)]
        preserve_merges: bool,
        version: Option<String>,
    },
}

impl Execute for HotfixAction {
    fn execute(self) -> Result<(), AppError> {
        let config = GitFlowConfig::load()?;

        match self {
            Self::Start { version, base } => {
                let base = base.unwrap_or_else(|| config.main_branch.clone());
                let branch_name = format!("{}{}", config.hotfix_prefix, version);

                git::run_hook("pre-flow-hotfix-start", &[&version, &base])?;

                info!("Starting hotfix: {}", version);
                git::run_git_silent(&["checkout", "-b", &branch_name, &base], false)?;

                git::run_hook("post-flow-hotfix-start", &[&version, &base])?;
            }
            Self::Finish {
                version,
                common,
                tagging,
                push,
            } => {
                let branch_name = format!("{}{}", config.hotfix_prefix, version);
                let tag_name = format!("{}{}", config.version_tag_prefix, version);

                // Workspace safety check
                if !git::is_working_tree_clean()? {
                    return Err(AppError::Config(
                        "Working tree is not clean. Commit or stash your changes first."
                            .to_string(),
                    ));
                }

                git::run_hook("pre-flow-hotfix-finish", &[&version])?;

                if common.fetch {
                    git::run_git_silent(&["fetch", "origin"], false)?;
                }

                // Merge into main
                git::run_git_silent(&["checkout", &config.main_branch], false)?;
                git::run_git_silent(&["merge", "--no-ff", &branch_name], false)?;

                // Tag
                if !tagging.notag {
                    let mut tag_args = vec!["tag".to_string()];
                    if tagging.sign {
                        tag_args.push("-s".to_string());
                    }

                    let msg = tagging
                        .message
                        .unwrap_or_else(|| format!("Hotfix {}", version));
                    tag_args.push("-m".to_string());
                    tag_args.push(msg);
                    tag_args.push(tag_name.clone());

                    let tag_args_refs: Vec<&str> = tag_args.iter().map(|s| s.as_str()).collect();
                    git::run_git_silent(&tag_args_refs, false)?;
                }

                // Merge into develop
                git::run_git_silent(&["checkout", &config.develop_branch], false)?;
                git::run_git_silent(&["merge", "--no-ff", &branch_name], false)?;

                // Delete branch
                if !common.keep {
                    git::run_git_silent(&["branch", "-d", &branch_name], false)?;
                }

                if push {
                    git::run_git_silent(&["push", "origin", &config.main_branch], false)?;
                    git::run_git_silent(&["push", "origin", &config.develop_branch], false)?;
                    if !tagging.notag {
                        git::run_git_silent(&["push", "origin", &tag_name], false)?;
                    }
                }

                info!(
                    "Hotfix '{}' finished. Merged into '{}' and '{}'.",
                    version, config.main_branch, config.develop_branch
                );

                git::run_hook("post-flow-hotfix-finish", &[&version])?;
            }
            Self::Publish { version } => {
                let branch_name = format!("{}{}", config.hotfix_prefix, version);
                git::run_git_silent(&["push", "-u", "origin", &branch_name], false)?;
                info!("Hotfix '{}' published to origin", version);
            }
            Self::Track { version } => {
                let branch_name = format!("{}{}", config.hotfix_prefix, version);
                git::run_git_silent(
                    &[
                        "checkout",
                        "-b",
                        &branch_name,
                        &format!("origin/{}", branch_name),
                    ],
                    false,
                )?;
                info!("Now tracking hotfix '{}' from origin", version);
            }
            Self::Delete { version, force, remote } => {
                let branch_name = format!("{}{}", config.hotfix_prefix, version);
                if remote {
                    git::run_git_silent(&["push", "origin", "--delete", &branch_name], false)?;
                }
                let delete_flag = if force { "-D" } else { "-d" };
                git::run_git_silent(&["branch", delete_flag, &branch_name], false)?;
                info!("Hotfix branch '{}' deleted", branch_name);
            }
            Self::List { verbose: _ } => {
                let prefix = config.hotfix_prefix.clone();
                let branches = git::list_branches(&prefix)?;
                if branches.is_empty() {
                    info!("No hotfix branches found.");
                } else {
                    info!("Hotfix branches:");
                    for b in branches {
                        println!("  {}", b.trim_start_matches(&prefix));
                    }
                }
            }
            Self::Rebase { interactive, preserve_merges, version } => {
                let current = git::current_branch()?;
                let hotfix_name = version.unwrap_or_else(|| current.replace(&config.hotfix_prefix, ""));
                let branch_name = format!("{}{}", config.hotfix_prefix, hotfix_name);
                let base = git::run_git(&["config", "--get", &format!("gitflow.branch.{}.base", branch_name)], false)
                    .unwrap_or_else(|_| config.main_branch.clone());

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
        }
        Ok(())
    }
}
