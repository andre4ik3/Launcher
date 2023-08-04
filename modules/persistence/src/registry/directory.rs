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
use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use tokio::fs;
use tracing::{debug, error, instrument, trace};

use util::directories;

use crate::registry::RegistryError;

type Result<T> = core::result::Result<T, RegistryError>;

/// A DirectoryRegistry stores a list of T. The type T is serialized and stored inside a unique
/// directory in a sub-path of the data directory. You can configure the file name and top-level
/// directory. For example: JavaInfo is stored in a Java.toml file in parent directory Java.
pub struct DirectoryRegistry<T>
where
    T: for<'a> Deserialize<'a> + Serialize,
{
    /// The assembled base directory.
    base: PathBuf,
    /// The file name where T is stored (in a sub-directory of parent_dir).
    file: &'static str,
    /// The loaded entries of this registry.
    entries: HashMap<String, T>,
}

impl<T> DirectoryRegistry<T>
where
    T: for<'a> Deserialize<'a> + Serialize,
{
    /// Creates the registry and loads any existing entries into memory.
    pub async fn create(base: impl AsRef<str>, file: &'static str) -> Result<DirectoryRegistry<T>> {
        let base = directories::DATA.join(base.as_ref());
        fs::create_dir_all(&base).await?;

        let mut registry = Self {
            base,
            file,
            entries: HashMap::new(),
        };

        registry.refresh().await?;
        Ok(registry)
    }

    /// Gets an entry from the in-memory entries cache.
    pub fn get(&self, id: impl AsRef<str>) -> Option<&T> {
        self.entries.get(id.as_ref())
    }

    /// Puts an entry with a specific ID into the registry and writes it to disk.
    pub async fn insert(&mut self, id: impl Into<String>, data: T) -> Result<()> {
        let id = id.into();
        let dir = self.base.join(&id);
        let serialized = toml::to_string(&data)?;

        fs::create_dir_all(&dir).await?;
        fs::write(dir.join(self.file), serialized).await?;
        self.entries.insert(id, data);

        Ok(())
    }

    /// Reads all entries from disk into memory.
    #[instrument(skip_all, fields(path = self.base.to_string_lossy().to_string()))]
    pub async fn refresh(&mut self) -> Result<()> {
        debug!("Starting refresh.");

        while let Some(entry) = fs::read_dir(&self.base).await?.next_entry().await? {
            if !entry.file_type().await?.is_dir() {
                trace!("{} is not a directory! Skipping", entry.path().display());
                continue;
            }

            let path = entry.path().join(self.file);

            // read file to string, continue with loop if failed
            let data = match fs::read_to_string(&path).await {
                Ok(data) => data,
                Err(err) => {
                    error!("Failed to read entry at {}: {err}", path.display());
                    continue;
                }
            };

            // parse string to type, continue with loop if failed
            let data = match toml::from_str(&data) {
                Ok(data) => data,
                Err(err) => {
                    error!("Failed to read entry at {}: {err}", path.display());
                    continue;
                }
            };

            let id = entry.file_name().to_string_lossy().to_string();
            self.entries.insert(id, data);
        }

        debug!("Done refreshing.");
        Ok(())
    }
}
