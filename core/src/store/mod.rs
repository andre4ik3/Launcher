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
use tokio::sync::{RwLockReadGuard, RwLockWriteGuard};

pub use config::{ConfigHolder, CONFIG};
pub use credentials::{CredentialsHolder, CREDENTIALS};

mod config;
mod credentials;

/// An object that holds type T. It is used as "global state" for the whole application.
#[async_trait]
pub trait StoreHolder<T> {
    async fn get(&self) -> T;
    async fn check(&self, func: impl FnOnce(RwLockReadGuard<T>) -> bool + Send) -> bool;
    async fn change(&self, func: impl FnOnce(RwLockWriteGuard<T>) + Send) -> Result<()>;
    async fn flush(&self) -> Result<()>;
}
