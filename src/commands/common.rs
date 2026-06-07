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

use clap::Args;

#[derive(Args, Debug, Clone)]
pub struct CommonFinishFlags {
    /// Fetch from origin before performing finish
    #[arg(short = 'F', long)]
    pub fetch: bool,

    /// Keep branch after performing finish
    #[arg(short = 'k', long)]
    pub keep: bool,
}

#[derive(Args, Debug, Clone)]
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
