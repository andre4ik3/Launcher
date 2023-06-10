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
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use tokio::fs::{create_dir_all, read_to_string, write};
use tokio::sync::{OnceCell, RwLock, RwLockReadGuard, RwLockWriteGuard};

pub use v1::ConfigV1;

use crate::utils::get_dirs;

mod v1;

lazy_static! {
    /// A global instance of [ConfigHolder].
    pub static ref CONFIG: AsyncOnce<ConfigHolder> = AsyncOnce::new(ConfigHolder::init());
}

/// A boolean that dictates whether the [ConfigHolder] has already been initialized.
static HAS_INITIALIZED: OnceCell<bool> = OnceCell::const_new();

/// An enum that can contain any (supported) config version.
#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "version", content = "config", rename_all = "lowercase")]
enum Config {
    V1(ConfigV1),
}

#[allow(clippy::from_over_into)]
impl Into<ConfigV1> for Config {
    fn into(self) -> ConfigV1 {
        match self {
            // upgrading system, a bit useless right now
            Config::V1(data) => data,
        }
    }
}

/// Facilitates parallel reading and writing to the configuration. Access via [CONFIG].
pub struct ConfigHolder {
    /// An internal lock for controlling read/write access to the configuration.
    lock: RwLock<ConfigV1>,
    /// Path to the configuration file.
    path: PathBuf,
}

impl ConfigHolder {
    /// Initializes the ConfigHolder. Can only run once (see [HAS_INITIALIZED]).
    async fn init() -> Self {
        let path = get_dirs().config_dir().join("Config.toml");
        let data: Result<ConfigV1> = (|| async {
            let data = read_to_string(&path).await?;
            let data: Config = toml::from_str(&data)?;
            let data: ConfigV1 = data.into();
            Ok(data)
        })()
        .await;

        HAS_INITIALIZED.set(true).expect("Already initialized!");
        Self {
            lock: RwLock::new(data.unwrap_or_default()),
            path,
        }
    }

    /// Gets the inner configuration. This will obtain a frozen clone of the inner config.
    pub async fn get(&self) -> ConfigV1 {
        // TODO: Make this not clone?
        let lock = self.lock.read().await;
        (*lock).clone()
    }

    /// Runs a function against the inner configuration to check for something. This is preferred
    /// over [ConfigHolder::get] as it doesn't clone the config.
    pub async fn check(&self, func: impl FnOnce(RwLockReadGuard<ConfigV1>) -> bool) -> bool {
        let lock = self.lock.read().await;
        func(lock)
    }

    /// Modifies the inner configuration. This will obtain exclusive write access and run the passed
    /// function to modify the config.
    pub async fn change(&self, func: impl FnOnce(RwLockWriteGuard<ConfigV1>)) -> Result<()> {
        let lock = self.lock.write().await;
        func(lock); // Dropped after this, lock released, safe to write.
        self.flush().await
    }

    /// Flushes changes to disk. Called automatically after [ConfigHolder::change].
    pub async fn flush(&self) -> Result<()> {
        let lock = self.lock.read().await;
        // TODO: Make this not clone?
        let data = toml::to_string_pretty(&Config::V1((*lock).clone()))?;

        create_dir_all(&self.path.parent().ok_or(anyhow!("No parent"))?).await?;
        write(&self.path, data).await?;

        Ok(())
    }
}
