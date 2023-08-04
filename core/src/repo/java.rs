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

use anyhow::Result;
use async_once::AsyncOnce;
use async_trait::async_trait;
use lazy_static::lazy_static;
use tokio::fs;
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::models::{JavaBuild, JavaInfo};
use crate::net::download::DownloadedArchive;
use crate::repo::Repo;
use crate::utils::{extract, get_dirs};

lazy_static! {
    pub static ref JAVA: AsyncOnce<JavaRepo> = AsyncOnce::new(JavaRepo::init());
}

pub struct JavaRepo {
    lock: RwLock<PathBuf>,
}

impl JavaRepo {
    async fn init() -> Self {
        Self {
            lock: RwLock::new(get_dirs().data_dir().join("Java")),
        }
    }
}

#[async_trait]
impl Repo<JavaInfo, JavaBuild> for JavaRepo {
    async fn add(&self, archive: DownloadedArchive<JavaBuild>) -> Result<JavaInfo> {
        let lock = self.lock.write().await;

        let self_id = Uuid::new_v4().to_string();
        let path = (*lock).join(&self_id);

        let metadata: JavaInfo = archive.metadata.into();

        extract(archive.data, archive.format, &path).await?;
        fs::write(path.join("Java.toml"), toml::to_string(&metadata)?).await?;

        drop(lock); // release the lock

        // Delete other builds with the same major version and provider
        for (id, build) in self.list().await? {
            if self_id != id
                && build.version.major == metadata.version.major
                && build.provider == metadata.provider
            {
                self.delete(id).await?;
            }
        }

        Ok(metadata)
    }

    async fn delete(&self, id: impl AsRef<str> + Send) -> Result<()> {
        let lock = self.lock.write().await;
        let path = (*lock).join(id.as_ref());
        fs::remove_dir_all(path).await?;
        Ok(())
    }

    async fn get(&self, id: impl AsRef<str> + Send) -> Option<Box<JavaInfo>> {
        if let Ok(entries) = self.list().await {
            entries.get(id.as_ref()).map(|data| Box::new(data.clone()))
        } else {
            None
        }
    }

    async fn list(&self) -> Result<HashMap<String, JavaInfo>> {
        let lock = self.lock.read().await;
        let mut results = HashMap::new();

        let mut dir = fs::read_dir(&*lock).await?;
        while let Some(entry) = dir.next_entry().await? {
            let path = entry.path().join("Java.toml");
            if path.exists() {
                if let Ok(data) = fs::read_to_string(path).await {
                    if let Ok(data) = toml::from_str(&data) {
                        results.insert(entry.file_name().to_string_lossy().to_string(), data);
                    }
                }
            }
        }

        Ok(results)
    }
}
