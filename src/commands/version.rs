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
