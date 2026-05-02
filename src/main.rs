use clap::Parser;

use crate::parser::Execute;

mod parser;

fn main() -> anyhow::Result<()> {
    let args = parser::Cli::parse();

    args.command.execute()?;

    Ok(())
}
