mod cli;
mod git;
mod error;
mod commands;
mod config;

use clap::Parser;
use crate::cli::Execute;

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let args = cli::Cli::parse();

    args.command.execute()?;

    Ok(())
}
