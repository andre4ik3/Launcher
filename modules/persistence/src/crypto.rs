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

use std::ffi::OsStr;
use std::fmt::Debug;
use std::path::Path;

use aes_gcm::{AeadCore, Aes256Gcm, Key, KeyInit};
use aes_gcm::aead::{Aead, Nonce, OsRng};
use keyring::Entry;
use thiserror::Error;
use tokio::{fs, task};

#[derive(Debug, Error)]
pub enum Error {
    #[error("failed to perform cryptographic operation")]
    Crypto,
    #[error("key is an invalid length, expected length 32 but found {0}")]
    KeySize(usize),
    #[error("nonce is an invalid length, expected length 12 but found {0}")]
    NonceSize(usize),
    #[error("failed to parse stored string in encrypted file: {0}")]
    Parse(#[from] std::string::FromUtf8Error),

    #[error("failed to perform keyring operation: {0}")]
    KeyringOp(#[from] keyring::Error),
    #[error("failed to parse key in system keychain: {0}")]
    KeyringParse(#[from] hex::FromHexError),
    #[error("failed to read/write keyfile: {0}")]
    KeyfileIo(#[from] std::io::Error),
    #[error("no key was found")]
    KeyNotFound,
}

type Result<T> = core::result::Result<T, Error>;

const KEYCHAIN_SERVICE: &str = "Launcher Encrypted Registry";

// === Credential Providers ===

/// Retrieves a key from the system keychain. This function is blocking!
fn keyring_read(stem: &str) -> Result<Key<Aes256Gcm>> {
    // task::spawn_blocking(async {});
    let key = Entry::new(KEYCHAIN_SERVICE, stem)?;
    let key = hex::decode(key.get_password()?)?;

    // convert the key to fixed-length byte slice, also checks for length mismatch
    let key: &[u8; 32] = key
        .as_slice()
        .try_into()
        .map_err(|_| Error::KeySize(key.len()))?;

    let key: &Key<Aes256Gcm> = key.into();
    Ok(*key)
}

/// Writes a key to the system keychain. This function is blocking!
fn keyring_write(stem: &str, key: &Key<Aes256Gcm>) -> Result<()> {
    let entry = Entry::new(KEYCHAIN_SERVICE, stem)?;
    let key = hex::encode(key);
    entry.set_password(&key)?;
    Ok(())
}

/// Retrieves a key from the filesystem from a side-by-side keyfile. This is "insecure" (as in not
/// encrypted) on purpose, and designed as a fallback. The file argument is the path to the
/// encrypted file.
async fn keyfile_read(file: impl AsRef<Path>) -> Result<Key<Aes256Gcm>> {
    let file = file.as_ref().with_extension("key");
    let key = fs::read(file).await?;

    // convert the key to fixed-length byte slice, also checks for length mismatch
    let key: &[u8; 32] = key
        .as_slice()
        .try_into()
        .map_err(|_| Error::KeySize(key.len()))?;

    let key: &Key<Aes256Gcm> = key.into();
    Ok(*key)
}

/// Writes a key to a side-by-side keyfile. This is "insecure" (as in not encrypted) on purpose, and
/// designed as a fallback. The file argument is the path to the encrypted file.
async fn keyfile_write(file: impl AsRef<Path>, key: &Key<Aes256Gcm>) -> Result<()> {
    let file = file.as_ref().with_extension("key");
    fs::write(file, key).await?;
    Ok(())
}

/// Attempts to remove a side-by-side keyfile. The file argument is the path to the encrypted file.
async fn keyfile_clean(file: impl AsRef<Path>) -> Result<()> {
    let file = file.as_ref().with_extension("key");
    if fs::try_exists(&file).await? {
        fs::remove_file(&file).await?;
    }

    Ok(())
}

// === Reading/Writing Credentials ===

/// Gets a credential for the specified file. Will try both system keychain and key file.
#[tracing::instrument(name = "crypto::read_key")]
pub async fn read_key(file: impl AsRef<Path> + Debug) -> Option<Key<Aes256Gcm>> {
    let file = file.as_ref().to_owned();

    let keyfile_key = keyfile_read(&file).await;
    let keyring_key: Result<Key<Aes256Gcm>> = task::spawn_blocking(move || {
        file.file_stem()
            .and_then(OsStr::to_str)
            .ok_or(Error::KeyNotFound)
            .and_then(keyring_read)
    })
        .await
        .expect("blocking thread panicked");

    tracing::debug!("Keyring key: {}", keyring_key.is_ok());
    tracing::debug!("Keyfile key: {}", keyfile_key.is_ok());

    keyring_key.ok().or(keyfile_key.ok())
}

/// Saves a credential for the specified file.
#[tracing::instrument(name = "crypto::write_key", skip(key))]
pub async fn write_key(file: impl AsRef<Path> + Debug, key: Key<Aes256Gcm>) -> Result<()> {
    let file = file.as_ref().to_owned();
    let elif = file.clone(); // hack, this one is moved into the closure below

    let keyring_result = task::spawn_blocking(move || {
        elif.file_stem()
            .and_then(OsStr::to_str)
            .ok_or(Error::KeyNotFound)
            .and_then(|stem| keyring_write(stem, &key))
    })
        .await
        .expect("blocking thread panicked");

    match keyring_result {
        Ok(()) => {
            if let Err(err) = keyfile_clean(&file).await {
                tracing::warn!("Failed to cleanup keyfile after successful keychain migration: {err}");
            };
            Ok(())
        }
        Err(err) => {
            tracing::debug!("Failed to write to keychain, trying keyfile instead: {err}");
            keyfile_write(&file, &key).await
        }
    }
}

// === Cryptographic Operations ===

/// Generates a new [Key\<Aes256Gcm\>].
pub async fn generate_key() -> Key<Aes256Gcm> {
    tracing::debug!("Generating a new key...");
    task::spawn_blocking(|| Aes256Gcm::generate_key(OsRng))
        .await
        .expect("blocking thread panicked")
}

/// Generates a new [Nonce\<Aes256Gcm\>].
pub async fn generate_nonce() -> Nonce<Aes256Gcm> {
    tracing::debug!("Generating a new nonce...");
    task::spawn_blocking(|| Aes256Gcm::generate_nonce(OsRng))
        .await
        .expect("blocking thread panicked")
}

/// Attempts to decrypt a read file. The first 12 bytes of the data are treated as the nonce (e.g.
/// from the [encrypt] function).
pub async fn decrypt(mut data: Vec<u8>, key: Key<Aes256Gcm>) -> Result<String> {
    tracing::debug!("Decrypting {} bytes of data", data.len());

    if data.len() < 12 {
        tracing::error!("Data length is too small, maybe corrupted?");
        return Err(Error::NonceSize(data.len()));
    }

    // Read nonce from the data
    let nonce: Vec<u8> = data.drain(0..12).collect();
    let nonce: [u8; 12] = nonce.try_into().unwrap(); // length was already checked above
    let nonce = Nonce::<Aes256Gcm>::from(nonce);

    // Decrypt data using the nonce we just read
    let data = task::spawn_blocking(move || Aes256Gcm::new(&key).decrypt(&nonce, data.as_slice()))
        .await
        .expect("blocking thread panicked")
        .map_err(|_| Error::Crypto)?; // aes_gcm::Error is opaque anyway

    // Convert the data to a string
    let data = String::from_utf8(data)?;
    tracing::debug!(
        "Decryption success, returning {} decrypted bytes",
        data.len()
    );
    Ok(data)
}

/// Attempts to encrypt some data. The result will be a concatenated nonce + payload, suitable for
/// use with [decrypt] function.
pub async fn encrypt(data: Vec<u8>, key: Key<Aes256Gcm>) -> Result<Vec<u8>> {
    tracing::debug!("Encrypting {} bytes of data", data.len());

    let nonce = generate_nonce().await;
    let data = task::spawn_blocking(move || Aes256Gcm::new(&key).encrypt(&nonce, data.as_slice()))
        .await
        .expect("blocking thread panicked")
        .map_err(|_| Error::Crypto)?;

    // chain the nonce and encrypted payload together
    let data: Vec<u8> = nonce.into_iter().chain(data.into_iter()).collect();
    tracing::debug!(
        "Encryption success, returning {} encrypted bytes",
        data.len()
    );
    Ok(data)
}
