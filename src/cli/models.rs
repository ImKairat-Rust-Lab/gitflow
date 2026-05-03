use clap::{Args, Parser, Subcommand};
use clap_complete::Shell;

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

    /// Generate shell completions
    Completions {
        /// The shell to generate completions for
        #[arg(value_enum)]
        shell: Shell,
    },
}


#[derive(Args, Debug)]
pub struct CommonFinishFlags {
    /// Fetch from origin before performing finish
    #[arg(short = 'F', long)]
    pub fetch: bool,
    
    /// Keep branch after performing finish
    #[arg(short = 'k', long)]
    pub keep: bool,
}

#[derive(Args, Debug)]
pub struct TaggingFlags {
    /// Sign the tag cryptographically (GPG)
    #[arg(short = 's', long)]
    pub sign: bool,
    
    /// Use the given tag message
    #[arg(short = 'm', long)]
    pub message: Option<String>,
    
    /// Don't tag this release
    #[arg(short = 'n', long)]
    pub notag: bool,
}


#[derive(Args, Debug)]
pub struct InitArgs {
    /// Use default branch naming conventions
    #[arg(short = 'd', long)]
    pub default: bool,
    
    /// Force setting of gitflow branches, even if already configured
    #[arg(short = 'f', long)]
    pub force: bool,
}

#[derive(Subcommand, Debug)]
pub enum FeatureAction {
    /// Start a new feature branch
    Start {
        name: String,
        base: Option<String>,
    },
    /// Finish a feature branch
    Finish {
        #[command(flatten)]
        common: CommonFinishFlags,
        
        /// Rebase instead of merge
        #[arg(short = 'r', long, conflicts_with = "squash")]
        rebase: bool,
        
        /// Squash feature during merge
        #[arg(short = 'S', long, conflicts_with = "rebase")]
        squash: bool,
        
        /// Force delete feature branch after finish
        #[arg(short = 'D', long)]
        force_delete: bool,
        
        name: String,
    },
    Publish { name: String },
    Track { name: String },
    Delete {
        #[arg(short = 'f', long)]
        force: bool,
        #[arg(short = 'r', long)]
        remote: bool,
        name: String,
    },
}

#[derive(Subcommand, Debug)]
pub enum ReleaseAction {
    Start {
        version: String,
        base: Option<String>,
    },
    Finish {
        #[command(flatten)]
        common: CommonFinishFlags,
        
        #[command(flatten)]
        tagging: TaggingFlags,
        
        /// Push to remote after finishing
        #[arg(short = 'p', long)]
        push: bool,
        
        version: String,
    },
    Publish { version: String },
    Delete {
        #[arg(short = 'f', long)]
        force: bool,
        #[arg(short = 'r', long)]
        remote: bool,
        version: String,
    },
}

#[derive(Subcommand, Debug)]
pub enum HotfixAction {
    Start {
        version: String,
        base: Option<String>,
    },
    Finish {
        #[command(flatten)]
        common: CommonFinishFlags,
        
        #[command(flatten)]
        tagging: TaggingFlags,
        
        #[arg(short = 'p', long)]
        push: bool,
        
        version: String,
    },
}

#[derive(Subcommand, Debug)]
pub enum SupportAction {
    /// Start a new support branch
    Start {
        version: String,
        base: String,
    },
}

#[derive(Subcommand, Debug)]
pub enum BugfixAction {
    Start {
        name: String,
        base: Option<String>,
    },
    Finish {
        #[command(flatten)]
        common: CommonFinishFlags,
        
        #[arg(short = 'r', long)]
        rebase: bool,
        
        name: String,
    },
}

pub trait Execute {
    fn execute(self) -> anyhow::Result<()>;
}
