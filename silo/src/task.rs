// Copyright © 2023-2024 andre4ik3
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

use std::path::Path;

use async_trait::async_trait;

pub use game_versions::TaskGameVersions;
pub use java::TaskJava;

mod game_versions;
mod java;

#[async_trait]
pub trait Task {
    /// The input of this task (allows the task to depend on other tasks).
    type Input;

    /// The output of this task (allows other tasks to depend on this task).
    type Output;

    /// Runs the task to completion.
    async fn run(root: impl AsRef<Path> + Send + Sync, input: Self::Input) -> anyhow::Result<Self::Output>;
}
