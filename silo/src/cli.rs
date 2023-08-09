// Copyright © 2023 andre4ik3
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

use std::collections::HashSet;
use std::path::PathBuf;

use clap::{Parser, ValueEnum};

/// Data generation utility
#[derive(Parser)]
#[command(version, about)]
pub struct Cli {
    /// The directory where files will be saved (existing files will be overwritten)
    #[arg(short, long, default_value = "_site")]
    pub output: PathBuf,
    /// The tasks to run (can run other tasks if specified tasks depend on output)
    #[arg(short, long)]
    pub task: Vec<Mode>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Mode {
    /// Run swiftly
    Fast,
    /// Crawl slowly but steadily
    ///
    /// This paragraph is ignored because there is no long help text for possible values.
    Slow,
}

pub fn parse() -> Cli {
    Cli::parse()
}
