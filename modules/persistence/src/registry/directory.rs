// Copyright © 2023-2025 andre4ik3
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

use utils::directories;

use crate::registry::Result;

/// A DirectoryRegistry stores a list of T. The type T is serialized and stored inside a unique
/// directory in a sub-path of the data directory. You can configure the file name and top-level
/// directory. For example: JavaInfo is stored in a Java.toml file in parent directory Java.
pub struct DirectoryRegistry<T>
where
    T: for<'a> Deserialize<'a> + Serialize,
{
    /// The assembled base directory.
    base: PathBuf,
    /// The file name where T is stored (in a subdirectory of parent_dir).
    file: &'static str,
    /// The loaded entries of this registry.
    entries: HashMap<String, T>,
}

impl<T> DirectoryRegistry<T>
where
    T: for<'a> Deserialize<'a> + Serialize,
{
    /// Creates a new directory registry and loads any existing entries from the base into memory.
    #[tracing::instrument(name = "DirectoryRegistry::new")]
    pub async fn new(base: &str, file: &'static str) -> Result<DirectoryRegistry<T>> {
        let base = directories::DATA.join(base);
        fs::create_dir_all(&base).await?;

        tracing::trace!("Creating new directory registry at {}", base.display());
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

    /// Deletes an entry with a specific ID from the in-memory cache *and the disk*. Use carefully!
    pub async fn delete(&mut self, id: impl Into<String>) -> Result<()> {
        let id = id.into();

        if !self.entries.contains_key(&id) {
            return Ok(());
        }

        let dir = self.base.join(&id);
        fs::remove_dir_all(dir).await?;
        self.entries.remove(&id);
        Ok(())
    }

    /// Reads all entries from disk into memory.
    #[tracing::instrument(name = "DirectoryRegistry::refresh", skip(self), fields(base = % self.base.display()
    ))]
    pub async fn refresh(&mut self) -> Result<()> {
        tracing::trace!("Starting refresh.");

        let mut stream = fs::read_dir(&self.base).await?;
        while let Some(entry) = stream.next_entry().await? {
            if !entry.file_type().await?.is_dir() {
                continue;
            }

            let path = entry.path().join(self.file);
            tracing::trace!("Considering path {}", path.display());

            // read file to string, continue with loop if failed
            let data = match fs::read_to_string(&path).await {
                Ok(data) => data,
                Err(err) => {
                    tracing::error!("Failed to read entry at {}: {err}", path.display());
                    continue;
                }
            };

            // parse string to type, continue with loop if failed
            let data = match toml::from_str(&data) {
                Ok(data) => data,
                Err(err) => {
                    tracing::error!("Failed to read entry at {}: {err}", path.display());
                    continue;
                }
            };

            let id = entry.file_name().to_string_lossy().to_string();
            tracing::trace!("Adding {id} to entries");
            self.entries.insert(id, data);
        }

        tracing::trace!("Done refreshing.");
        Ok(())
    }
}
