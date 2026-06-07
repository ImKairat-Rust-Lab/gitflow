// Copyright (C) 2026 Kairat Kubanychbek uulu <https://github.com/ImKairat>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use crate::commands::{
    BugfixAction, ConfigAction, FeatureAction, HotfixAction, InitArgs, LogArgs, ReleaseAction,
    SupportAction, VersionArgs,
};
use crate::error::AppError;
use clap::{Parser, Subcommand};

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

    /// Shows version information
    Version(VersionArgs),

    /// Show log deviating from base branch
    Log(LogArgs),

    /// Manage your git-flow configuration
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },
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
            Self::Version(args) => args.execute(),
            Self::Log(args) => args.execute(),
            Self::Config { action } => action.execute(),
        }
    }
}
