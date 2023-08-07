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

use aes_gcm::{Aes256Gcm, Key};
use serde::{Deserialize, Serialize};
use tokio::fs;
use tokio::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use tracing::{instrument, trace, warn};

use utils::directories;

use crate::crypto;
use crate::registry::Error;

type Result<T> = core::result::Result<T, Error>;

/// A FileRegistry stores a single instance of T in a file, managing access to it via a shared lock.
/// Custom read and write functions allow modifying how the data is stored on disk.
pub struct FileRegistry<T>
where
    T: Default + for<'a> Deserialize<'a> + Serialize,
{
    /// A lock holding the data. Used to control read/write access within the registry.
    data: RwLock<T>,
    /// Path to the file on disk.
    path: PathBuf,
    /// Optional encryption. Contains the encryption key, or None if encryption is disabled.
    encryption_key: Option<Key<Aes256Gcm>>,
}

impl<T> FileRegistry<T>
where
    T: Default + for<'a> Deserialize<'a> + Serialize,
{
    /// Creates the registry and reads the file from disk into memory. The file should have a .toml
    /// extension.
    #[instrument(name = "FileRegistry::new")]
    pub async fn new(file: &'static str) -> Result<FileRegistry<T>> {
        let data = RwLock::new(T::default());
        let path = directories::CONFIG.join(file);

        trace!("Creating new file registry at {}", path.display());
        let registry = FileRegistry {
            data,
            path,
            encryption_key: None,
        };

        if registry.load().await.is_err() {
            warn!("Failed to read registry. It will be overwritten and initialized with the defaults.");
        };

        registry.save().await?;
        Ok(registry)
    }

    /// Creates an encrypted registry. The file on disk will be encrypted, with the encryption key
    /// stored in the system's keychain or, if unavailable, a side-by-side keyfile. The file should
    /// have a .dat extension.
    #[instrument(name = "FileRegistry::new_encrypted")]
    pub async fn new_encrypted(file: &'static str) -> Result<FileRegistry<T>> {
        let data = RwLock::new(T::default());
        let path = directories::CONFIG.join(file);

        trace!("Creating new encrypted file registry at {}", path.display());

        // attempt to retrieve encryption key or make a new one
        let encryption_key = match crypto::read_key(&path).await {
            Some(key) => {
                trace!("Successfully found existing encryption key");
                key
            }
            None => {
                warn!("Could not find an encryption key, generating a new one...");
                crypto::generate_key().await
            }
        };

        // save encryption key, this will also attempt to migrate to keychain and remove keyfile
        crypto::write_key(&path, encryption_key).await?;

        let registry = FileRegistry {
            data,
            path,
            encryption_key: Some(encryption_key),
        };

        if registry.load().await.is_err() {
            warn!("Failed to read encrypted registry (possibly due to missing/wrong encryption key). It will be overwritten and initialized with the defaults.");
        };

        registry.save().await?;
        Ok(registry)
    }

    /// Returns a read guard containing a reference to the inner value.
    pub async fn get(&self) -> RwLockReadGuard<'_, T> {
        self.data.read().await
    }

    /// Returns a write guard containing a reference to the inner value. To avoid a deadlock, drop
    /// the guard after you are done with it. To flush data to disk, use [FileRegistry::save]!
    pub async fn get_mut(&self) -> RwLockWriteGuard<'_, T> {
        self.data.write().await
    }

    /// Loads the file from disk.
    #[instrument(name = "FileRegistry::load", skip(self), fields(file = % self.path.display()))]
    pub async fn load(&self) -> Result<()> {
        trace!("Reading file...");
        let mut lock = self.data.write().await;

        let data = match self.encryption_key {
            None => fs::read_to_string(&self.path).await?,
            Some(key) => crypto::decrypt(fs::read(&self.path).await?, key).await?,
        };

        let data = toml::from_str(&data)?;
        *lock = data;

        Ok(())
    }

    /// Saves the file to disk.
    #[instrument(name = "FileRegistry::save", skip(self), fields(file = % self.path.display()))]
    pub async fn save(&self) -> Result<()> {
        trace!("Writing file...");
        let lock = self.data.read().await;

        let data = toml::to_string(&*lock)?;
        let data = match self.encryption_key {
            None => data.into_bytes(),
            Some(key) => crypto::encrypt(data.into_bytes(), key).await?,
        };

        fs::write(&self.path, data).await?;
        Ok(())
    }
}
