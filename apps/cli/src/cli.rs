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

use clap::{Parser, Subcommand};

/// Launch Minecraft from the command line
///
/// This is a fully-featured launcher
#[derive(Debug, Parser)]
#[command(author, version, propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum InstanceSubcommand {
    /// Creates a new instance
    #[command(alias = "a", alias = "n", alias = "new")]
    Add,
    /// Edits an instance
    #[command(alias = "e")]
    Edit,
    /// Launches an instance
    #[command(alias = "l", alias = "r", alias = "run", alias = "start")]
    Launch,
    /// Lists all instances
    #[command(alias = "ls")]
    List,
    /// Removes an instance
    #[command(alias = "delete", alias = "rm")]
    Remove,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Create, manage, and launch instances
    #[command(alias = "i", alias = "in", alias = "ins")]
    Instance {
        /// Name of the instance
        name: String,

        #[command(subcommand)]
        command: InstanceSubcommand,
    },
}
