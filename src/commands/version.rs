use crate::cli::Execute;
use crate::error::AppError;
use clap::Args;

#[derive(Args, Debug)]
pub struct VersionArgs {}

impl Execute for VersionArgs {
    fn execute(self) -> Result<(), AppError> {
        println!("1.12.3 (Oxidized AVH Edition)");
        Ok(())
    }
}
