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
use async_once::AsyncOnce;
use async_trait::async_trait;
use lazy_static::lazy_static;
use tokio::sync::{OnceCell, RwLock, RwLockReadGuard, RwLockWriteGuard};
use zeroize::Zeroize;

use crate::models::Credentials;
use crate::store::StoreHolder;
use crate::utils::{get_credentials, write_credentials, write_key, CKey};

lazy_static! {
    /// A global instance of [CredentialsHolder].
    pub static ref CREDENTIALS: AsyncOnce<CredentialsHolder> = AsyncOnce::new(CredentialsHolder::init());
}

/// A boolean that dictates whether the holder has already been initialized.
static HAS_INITIALIZED: OnceCell<bool> = OnceCell::const_new();

/// Facilitates parallel reading and writing to the configuration. Access via [CONFIG].
pub struct CredentialsHolder {
    /// An internal lock for controlling read/write access.
    lock: RwLock<Credentials>,
    /// Key used for encryption and decryption.
    key: CKey,
}

impl Drop for CredentialsHolder {
    fn drop(&mut self) {
        // Zero out the memory of the key when it goes out of scope.
        self.key.zeroize();
    }
}

#[async_trait]
impl StoreHolder<Credentials> for CredentialsHolder {
    async fn init() -> Self {
        let (_status, data, key) = get_credentials().await;

        // TODO: Warn if status == [Status::Overwritten]

        // Panic if already initialized
        HAS_INITIALIZED.set(true).expect("Already initialized!");

        let lock = RwLock::new(data);
        Self { lock, key }
    }

    async fn get(&self) -> Credentials {
        let lock = self.lock.read().await;
        (*lock).clone()
    }

    async fn check(&self, func: impl FnOnce(RwLockReadGuard<Credentials>) -> bool + Send) -> bool {
        let lock = self.lock.read().await;
        func(lock)
    }

    async fn change(&self, func: impl FnOnce(RwLockWriteGuard<Credentials>) + Send) -> Result<()> {
        let lock = self.lock.write().await;
        func(lock); // Dropped after this, lock released, safe to write.
        self.flush().await
    }

    async fn flush(&self) -> Result<()> {
        let lock = self.lock.read().await;
        write_credentials(&lock, &self.key).await?;
        write_key(&self.key).await?;
        Ok(())
    }
}
