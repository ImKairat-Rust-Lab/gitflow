use crate::cli::{BugfixAction, Execute};
use crate::config::GitFlowConfig;
use crate::error::AppError;
use crate::git;
use tracing::info;

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
        }
        Ok(())
    }
}
