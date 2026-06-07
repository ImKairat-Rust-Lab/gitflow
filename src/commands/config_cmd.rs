use crate::cli::Execute;
use crate::error::AppError;
use crate::git;
use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum ConfigAction {
    /// Show the git-flow configurations
    List,
    /// Set the git-flow configuration option to the given value
    Set {
        option: String,
        value: String,
    },
    /// Set the given base for the given branch
    Base {
        #[arg(long)]
        get: bool,
        #[arg(long)]
        set: bool,
        branch: String,
        base: Option<String>,
    },
}

impl Execute for ConfigAction {
    fn execute(self) -> Result<(), AppError> {
        match self {
            Self::List => {
                match git::run_git(&["config", "--get-regexp", "^gitflow"], false) {
                    Ok(output) => {
                        println!("{}", output);
                    }
                    Err(_) => {
                        println!("No gitflow configuration found.");
                    }
                }
            }
            Self::Set { option, value } => {
                let key = if option.starts_with("gitflow.") {
                    option
                } else {
                    format!("gitflow.{}", option)
                };
                git::run_git_silent(&["config", &key, &value], false)?;
            }
            Self::Base { get, set, branch, base } => {
                let key = format!("gitflow.branch.{}.base", branch);
                if get || (!set && base.is_none()) {
                    if let Ok(val) = git::run_git(&["config", "--get", &key], false) {
                        println!("{}", val);
                    }
                } else if set || base.is_some() {
                    if let Some(b) = base {
                        git::run_git_silent(&["config", &key, &b], false)?;
                    } else {
                        return Err(AppError::Config("Must provide a base value when setting".to_string()));
                    }
                }
            }
        }
        Ok(())
    }
}
