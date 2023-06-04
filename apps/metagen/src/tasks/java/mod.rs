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

use anyhow::Result;
use async_trait::async_trait;
use launcher::models::java::JavaBuild;
use launcher::models::Environment;

mod adoptium;
mod zulu;

pub use adoptium::Adoptium;
pub use zulu::Zulu;

/// An abstract interface to fetch a specific Java build from the provider
#[async_trait]
pub trait Provider {
    async fn fetch(version: u8, env: Environment) -> Result<JavaBuild>;
}

/// A task to fetch Java builds for all available major versions and OS'es
pub struct JavaTask;
