use anyhow::Result;
use clap::CommandFactory;
use clap_complete::generate;
use std::io;

use crate::parser::{
    Cli, GitFlowCommands, InitArgs, FeatureAction, ReleaseAction, 
    HotfixAction, SupportAction, BugfixAction, Execute
};

impl Execute for GitFlowCommands {
    fn execute(self) -> Result<()> {
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

impl Execute for InitArgs {
    fn execute(self) -> Result<()> {
        println!("Выполнение Init: default={}, force={}", self.default, self.force);
        Ok(())
    }
}

impl Execute for FeatureAction {
    fn execute(self) -> Result<()> {
        match self {
            Self::Start { name, base } => {
                println!("Выполнение Feature Start: {} (base: {:?})", name, base);
            }
            Self::Finish { name, common, .. } => {
                println!("Выполнение Feature Finish: {} (fetch: {})", name, common.fetch);
            }
            Self::Publish { name } => println!("Выполнение Feature Publish: {}", name),
            Self::Track { name } => println!("Выполнение Feature Track: {}", name),
            Self::Delete { name, force, remote } => {
                println!("Выполнение Feature Delete: {} (force: {}, remote: {})", name, force, remote);
            }
        }
        Ok(())
    }
}

impl Execute for ReleaseAction {
    fn execute(self) -> Result<()> {
        match self {
            Self::Start { version, base } => {
                println!("Выполнение Release Start: {} (base: {:?})", version, base);
            }
            Self::Finish { version, common, tagging, push } => {
                println!(
                    "Выполнение Release Finish: {} (fetch: {}, push: {}, sign: {})", 
                    version, common.fetch, push, tagging.sign
                );
            }
            Self::Publish { version } => println!("Выполнение Release Publish: {}", version),
            Self::Delete { version, force, remote } => {
                println!("Выполнение Release Delete: {} (force: {}, remote: {})", version, force, remote);
            }
        }
        Ok(())
    }
}

impl Execute for HotfixAction {
    fn execute(self) -> Result<()> {
        match self {
            Self::Start { version, base } => {
                println!("Выполнение Hotfix Start: {} (base: {:?})", version, base);
            }
            Self::Finish { version, common, tagging, push } => {
                println!(
                    "Выполнение Hotfix Finish: {} (fetch: {}, push: {}, sign: {})", 
                    version, common.fetch, push, tagging.sign
                );
            }
        }
        Ok(())
    }
}

impl Execute for SupportAction {
    fn execute(self) -> Result<()> {
        match self {
            Self::Start { version, base } => {
                println!("Выполнение Support Start: {} (base: {})", version, base);
            }
        }
        Ok(())
    }
}

impl Execute for BugfixAction {
    fn execute(self) -> Result<()> {
        match self {
            Self::Start { name, base } => {
                println!("Выполнение Bugfix Start: {} (base: {:?})", name, base);
            }
            Self::Finish { name, common, rebase } => {
                println!("Выполнение Bugfix Finish: {} (fetch: {}, rebase: {})", name, common.fetch, rebase);
            }
        }
        Ok(())
    }
}
