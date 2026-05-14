mod cli;
mod commands;
mod config;
mod error;
mod git;

use crate::cli::Execute;
use clap::Parser;

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let args = cli::Cli::parse();

    args.command.execute()?;

    Ok(())
}
