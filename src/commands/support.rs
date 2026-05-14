use crate::cli::{Execute, SupportAction};
use crate::error::AppError;
use crate::git;
use crate::config::GitFlowConfig;
use tracing::info;

impl Execute for SupportAction {
    fn execute(self) -> Result<(), AppError> {
        let config = GitFlowConfig::load()?;

        match self {
            Self::Start { version, base } => {
                let branch_name = format!("{}{}", config.support_prefix, version);
                
                info!("Starting support branch: {}", version);
                git::run_git_silent(&["checkout", "-b", &branch_name, &base], false)?;
            }
        }
        Ok(())
    }
}
