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

use std::path::PathBuf;

use anyhow::{anyhow, Result};
use async_once::AsyncOnce;
use async_trait::async_trait;
use lazy_static::lazy_static;
use tokio::fs;
use tokio::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

use crate::models::Config;
use crate::store::StoreHolder;
use crate::utils::get_dirs;

lazy_static! {
    /// A global instance of [ConfigHolder].
    pub static ref CONFIG: AsyncOnce<ConfigHolder> = AsyncOnce::new(ConfigHolder::init());
}

/// Facilitates parallel reading and writing to the configuration. Access via [CONFIG].
pub struct ConfigHolder {
    lock: RwLock<Config>,
    path: PathBuf,
}

impl ConfigHolder {
    async fn init() -> Self {
        let path = get_dirs().config_dir().join("Config.toml");

        // Try to read the config from disk
        let data: Result<Config> = fs::read_to_string(&path)
            .await
            .map_err(|e| e.into()) // ⬅ convert to anyhow::Result ⬇
            .and_then(|s| toml::from_str(&s).map_err(|e| e.into()));

        let lock = RwLock::new(data.unwrap_or_default());
        Self { lock, path }
    }
}

#[async_trait]
impl StoreHolder<Config> for ConfigHolder {
    async fn get(&self) -> Config {
        let lock = self.lock.read().await;
        (*lock).clone()
    }

    async fn check(&self, func: impl FnOnce(RwLockReadGuard<Config>) -> bool + Send) -> bool {
        let lock = self.lock.read().await;
        func(lock)
    }

    async fn change(&self, func: impl FnOnce(RwLockWriteGuard<Config>) + Send) -> Result<()> {
        let lock = self.lock.write().await;
        func(lock); // Dropped after this, lock released, safe to write.
        self.flush().await
    }

    async fn flush(&self) -> Result<()> {
        let lock = self.lock.read().await;
        let data = toml::to_string_pretty(&*lock)?;

        fs::create_dir_all(&self.path.parent().ok_or(anyhow!("No parent"))?).await?;
        fs::write(&self.path, data).await?;

        Ok(())
    }
}
