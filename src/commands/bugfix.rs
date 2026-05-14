use crate::cli::{Execute, BugfixAction};
use crate::error::AppError;
use crate::git;
use crate::config::GitFlowConfig;
use tracing::info;

impl Execute for BugfixAction {
    fn execute(self) -> Result<(), AppError> {
        let config = GitFlowConfig::load()?;

        match self {
            Self::Start { name, base } => {
                let base = base.unwrap_or_else(|| config.develop_branch.clone());
                let branch_name = format!("{}{}", config.bugfix_prefix, name);
                
                info!("Starting bugfix: {}", name);
                git::run_git_silent(&["checkout", "-b", &branch_name, &base], false)?;
            }
            Self::Finish { name, common, rebase } => {
                let branch_name = format!("{}{}", config.bugfix_prefix, name);
                
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

                info!("Bugfix '{}' finished and merged into '{}'", name, config.develop_branch);
            }
        }
        Ok(())
    }
}
