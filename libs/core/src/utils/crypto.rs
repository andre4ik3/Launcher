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

use aes_gcm::aead::{Aead, OsRng};
use aes_gcm::{aead::Nonce, AeadCore, Aes256Gcm, Key, KeyInit};
use anyhow::{anyhow, bail, Result};
use keyring::Entry;
use tokio::fs;

use crate::models::Credentials;
use crate::utils::get_dirs;

pub type CKey = Key<Aes256Gcm>;
pub type CNonce = Nonce<Aes256Gcm>;

const ENTRY_SERVICE: &str = "dev.andre4ik3.Launcher";
const ENTRY_NAME: &str = "Launcher Credentials Store";

const DATAFILE: &str = "Credentials.dat";
const KEYFILE: &str = "Credentials.key";

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum Status {
    /// An existing credential store was decrypted and returned.
    Decrypted,
    /// No credential store was found, so one was created.
    Created,
    /// An existing store was found, but it could not be decrypted. A new one was created and it
    /// overwrote the existing one. A warning should be presented to the user that their accounts
    /// are no longer available and they should re-authenticate.
    Overwritten,
}

/// Retrieves the credentials decryption key from the system keychain.
async fn get_credentials_keychain() -> Result<CKey> {
    let key = Entry::new(ENTRY_SERVICE, ENTRY_NAME)?;
    let key = hex::decode(key.get_password()?)?;
    let key: &[u8; 32] = key.as_slice().try_into()?;
    let key: &CKey = key.into();
    Ok(*key)
}

/// Retrieves the credentials decryption key from a side-by-side file.
/// This is "insecure" (as in not encrypted) on purpose, and designed as a fallback.
async fn get_credentials_keyfile() -> Result<CKey> {
    let key = get_dirs().config_dir().join(KEYFILE);
    let key = hex::decode(fs::read_to_string(key).await?)?;
    let key: &[u8; 32] = key.as_slice().try_into()?;
    let key: &CKey = key.into();
    Ok(*key)
}

/// Tries reading an existing credential store. On success, returns nonce and encrypted data.
async fn read_credentials() -> Result<(CNonce, Vec<u8>)> {
    let mut data = fs::read(get_dirs().config_dir().join(DATAFILE)).await?;
    if data.len() <= 13 {
        bail!("Corrupted credential store");
    }

    let nonce: Vec<u8> = data.drain(0..12).collect();
    let nonce: [u8; 12] = nonce.try_into().unwrap();
    let nonce = Nonce::<Aes256Gcm>::from(nonce);

    Ok((nonce, data))
}

/// Tries decrypting credentials with a particular key.
fn decrypt_credentials(key: CKey, nonce: &CNonce, data: &[u8]) -> Result<(Credentials, CKey)> {
    let cipher = Aes256Gcm::new(&key);
    let data = cipher
        .decrypt(nonce, data)
        .map_err(|_| anyhow!("Decryption failed"))?;

    let data = String::from_utf8(data)?;
    let data: Credentials = toml::from_str(&data)?;

    Ok((data, key))
}

/// Saves credentials to disk.
pub async fn write_credentials(data: &Credentials, key: &CKey) -> Result<()> {
    let nonce = Aes256Gcm::generate_nonce(OsRng);
    let cipher = Aes256Gcm::new(key);

    let data = toml::to_string(data)?.into_bytes();
    let data = cipher
        .encrypt(&nonce, data.as_slice())
        .map_err(|_| anyhow!("Encryption failed"))?;

    let data: Vec<u8> = nonce.into_iter().chain(data.into_iter()).collect();
    let path = get_dirs().config_dir().join(DATAFILE);

    fs::create_dir_all(&path.parent().ok_or(anyhow!("No parent"))?).await?;
    fs::write(&path, data).await?;

    Ok(())
}

/// Writes a key to the keychain.
fn write_keychain(key: &CKey) -> Result<()> {
    let entry = Entry::new(ENTRY_SERVICE, ENTRY_NAME)?;
    entry.set_password(&hex::encode(key))?;
    Ok(())
}

/// Writes a key to the keyfile.
/// This is "insecure" (as in not encrypted) on purpose, and designed as a fallback.
async fn write_keyfile(key: &CKey) -> Result<()> {
    fs::write(get_dirs().config_dir().join(KEYFILE), key).await?;
    Ok(())
}

/// Writes the decryption key either to the keychain or a side-by-side keyfile.
pub async fn write_key(key: &CKey) -> Result<()> {
    match write_keychain(key) {
        Ok(_) => Ok(()),
        Err(_) => write_keyfile(key).await,
    }
}

/// Reads or creates a new credential store. See [Status] for possible outcomes.
pub async fn get_credentials() -> (Status, Credentials, Key<Aes256Gcm>) {
    if let Ok((nonce, data)) = read_credentials().await {
        let keychain = get_credentials_keychain()
            .await
            .and_then(|key| decrypt_credentials(key, &nonce, &data));

        let keyfile = get_credentials_keyfile()
            .await
            .and_then(|key| decrypt_credentials(key, &nonce, &data));

        // Read and decrypted from keychain, nothing else needed
        if let Ok((data, key)) = keychain {
            return (Status::Decrypted, data, key);
        }

        // Read and decrypted from keyfile, try upgrading to keychain
        if let Ok((data, key)) = keyfile {
            // Upgrade key, if this fails then it's (probably) fine (js iife ftw)
            let _: Result<()> = (|| async {
                write_keychain(&key)?;
                fs::remove_file(get_dirs().config_dir().join(KEYFILE)).await?;
                Ok(())
            })()
            .await;

            return (Status::Decrypted, data, key);
        }

        // By this point, we've exhausted both key storages. Make a new one and warn the user.
        let key = Aes256Gcm::generate_key(OsRng);
        let _ = write_key(&key).await; // if this fails then there's a problem with the machine
        (Status::Overwritten, Credentials::default(), key)
    } else {
        // Nothing exists (first run). Make a new one.
        let key = Aes256Gcm::generate_key(OsRng);
        let _ = write_key(&key).await;
        (Status::Created, Credentials::default(), key)
    }
}
