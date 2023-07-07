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

use std::collections::HashMap;

use anyhow::Result;
use async_trait::async_trait;

pub use java::JavaRepo;

mod java;

/// A repo is a place where multiple versions of something (Java, game versions) are stored.
/// In a way, it is similar to a store holder, but instead of only holding one, it holds multiple.
/// T is the type being stored (JavaInfo), U is the type needed for download (JavaBuild).
#[async_trait]
pub trait Repo<T, U> {
    async fn add(&mut self, data: &U) -> Result<T>;
    async fn delete(&mut self, id: impl AsRef<str> + Send) -> Result<()>;
    async fn get(&self, id: impl AsRef<str> + Send) -> Option<Box<T>>;
    async fn list(&self) -> Result<HashMap<String, T>>;
}
