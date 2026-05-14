use clap::{Parser, Subcommand};
use crate::error::AppError;
use crate::commands::{
    InitArgs, FeatureAction, ReleaseAction, HotfixAction, SupportAction, BugfixAction
};

#[derive(Parser, Debug)]
#[command(
    name = "gitflow",
    about = "Oxidized Gitflow: A high-performance Gitflow AVH implementation in Rust",
    version,
    arg_required_else_help = true,
    styles = clap::builder::Styles::styled()
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: GitFlowCommands,
}

#[derive(Subcommand, Debug)]
pub enum GitFlowCommands {
    /// Install the binary to ~/.local/bin and setup shell completions
    Install,

    /// Initialize a new git repo with support for the branching model.
    Init(InitArgs),

    /// Manage your feature branches.
    Feature {
        #[command(subcommand)]
        action: FeatureAction,
    },

    /// Manage your release branches.
    Release {
        #[command(subcommand)]
        action: ReleaseAction,
    },

    /// Manage your hotfix branches.
    Hotfix {
        #[command(subcommand)]
        action: HotfixAction,
    },

    /// Manage your support branches.
    Support {
        #[command(subcommand)]
        action: SupportAction,
    },

    /// Manage your bugfix branches.
    Bugfix {
        #[command(subcommand)]
        action: BugfixAction,
    },

    /// Automatically finish the current branch (feature, release, hotfix, or bugfix)
    Finish,
}

pub trait Execute {
    fn execute(self) -> Result<(), AppError>;
}

impl Execute for GitFlowCommands {
    fn execute(self) -> Result<(), AppError> {
        match self {
            Self::Init(args) => args.execute(),
            Self::Feature { action } => action.execute(),
            Self::Release { action } => action.execute(),
            Self::Hotfix { action } => action.execute(),
            Self::Support { action } => action.execute(),
            Self::Bugfix { action } => action.execute(),
            Self::Finish => crate::commands::run_finish_auto(),
            Self::Install => crate::commands::run_install(),
        }
    }
}
