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

use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

use util::directories;

use crate::registry::RegistryError;

type Result<T> = core::result::Result<T, RegistryError>;
type ReadFunction<T> = dyn FnMut(&Path) -> core::result::Result<T, std::io::Error>;
type WriteFunction<T> = dyn FnMut(&Path, &T) -> core::result::Result<(), std::io::Error>;

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
    encryption_key: Option<Vec<u8>>,
}

impl<T> FileRegistry<T>
where
    T: Default + for<'a> Deserialize<'a> + Serialize,
{
    /// Creates the registry and reads the file from disk into memory.
    pub async fn create(file: &'static str) -> Result<FileRegistry<T>> {
        let data = RwLock::new(T::default());
        let path = directories::CONFIG.join(file);

        let registry = FileRegistry {
            data,
            path,
            encryption_key: None,
        };

        Ok(registry)
    }

    /// Reads the data from disk into memory.
    pub async fn read(&mut self) -> Result<()> {
        Ok(())
    }

    /// Writes the data from memory to disk.
    pub async fn write(&mut self) -> Result<()> {
        Ok(())
    }
}
