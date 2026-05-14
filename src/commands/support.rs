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
        }
        Ok(())
    }
}
