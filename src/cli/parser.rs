use clap::CommandFactory;
use clap_complete::generate;
use std::io;

use crate::{
    cli::{Cli, Execute, GitFlowCommands},
    error::AppError,
};

impl Execute for GitFlowCommands {
    fn execute(self) -> Result<(), AppError> {
        match self {
            Self::Init(args) => args.execute(),
            Self::Feature { action } => action.execute(),
            Self::Release { action } => action.execute(),
            Self::Hotfix { action } => action.execute(),
            Self::Support { action } => action.execute(),
            Self::Bugfix { action } => action.execute(),
            Self::Completions { shell } => {
                let mut cmd = Cli::command();
                let bin_name = cmd.get_name().to_string();
                generate(shell, &mut cmd, bin_name, &mut io::stdout());
                Ok(())
            }
        }
    }
}
