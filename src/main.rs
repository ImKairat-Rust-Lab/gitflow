mod cli;
mod git;
mod error;
mod commands;
use clap::Parser;
use crate::cli::Execute;


fn main() -> anyhow::Result<()> {
    let args = cli::Cli::parse();

    args.command.execute()?;

    Ok(())
}
