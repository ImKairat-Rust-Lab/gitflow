use crate::cli::Execute;
use crate::error::AppError;
use crate::git;
use clap::Args;

#[derive(Args, Debug)]
pub struct LogArgs {}

impl Execute for LogArgs {
    fn execute(self) -> Result<(), AppError> {
        let current_branch = git::current_branch()?;
        let key = format!("gitflow.branch.{}.base", current_branch);
        
        let base_branch = match git::run_git(&["config", "--get", &key], false) {
            Ok(val) if !val.is_empty() => val,
            _ => {
                // Fallback to develop if base config is missing
                
                git::run_git(&["config", "--get", "gitflow.branch.develop"], false)
                    .unwrap_or_else(|_| "develop".to_string())
            }
        };

        git::run_git_interactive(
            &["log", "--no-merges", "--stat", "--reverse", &format!("{}..HEAD", base_branch)],
            false,
        )?;

        Ok(())
    }
}
