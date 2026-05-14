use crate::cli::{Execute, FeatureAction};
use crate::error::AppError;
use crate::git;
use crate::config::GitFlowConfig;
use tracing::info;

impl Execute for FeatureAction {
    fn execute(self) -> Result<(), AppError> {
        let config = GitFlowConfig::load()?;

        match self {
            Self::Start { name, base } => {
                let base = base.unwrap_or_else(|| config.develop_branch.clone());
                let branch_name = format!("{}{}", config.feature_prefix, name);
                
                info!("Starting feature: {}", name);
                git::run_git_silent(&["checkout", "-b", &branch_name, &base], false)?;
                info!("Switched to a new branch '{}'", branch_name);
            }
            Self::Finish { name, common, rebase, squash, force_delete } => {
                let branch_name = format!("{}{}", config.feature_prefix, name);
                
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
                    git::run_git_silent(&["commit", "-m", &format!("Finish feature {}", name)], false)?;
                } else {
                    git::run_git_silent(&["merge", "--no-ff", &branch_name], false)?;
                }

                // Delete branch
                if !common.keep {
                    let delete_flag = if force_delete { "-D" } else { "-d" };
                    git::run_git_silent(&["branch", delete_flag, &branch_name], false)?;
                }

                info!("Feature '{}' finished and merged into '{}'", name, config.develop_branch);
            }
            Self::Publish { name } => {
                let branch_name = format!("{}{}", config.feature_prefix, name);
                git::run_git_silent(&["push", "-u", "origin", &branch_name], false)?;
                info!("Feature '{}' published to origin", name);
            }
            Self::Track { name } => {
                let branch_name = format!("{}{}", config.feature_prefix, name);
                git::run_git_silent(&["checkout", "-b", &branch_name, &format!("origin/{}", branch_name)], false)?;
                info!("Now tracking feature '{}' from origin", name);
            }
            Self::Delete { name, force, remote } => {
                let branch_name = format!("{}{}", config.feature_prefix, name);
                
                if remote {
                    git::run_git_silent(&["push", "origin", "--delete", &branch_name], false)?;
                    info!("Remote branch '{}' deleted", branch_name);
                }

                let delete_flag = if force { "-D" } else { "-d" };
                git::run_git_silent(&["branch", delete_flag, &branch_name], false)?;
                info!("Local branch '{}' deleted", branch_name);
            }
        }
        Ok(())
    }
}
