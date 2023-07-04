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

use anyhow::Result;
use async_trait::async_trait;
use tokio::sync::RwLock;

use crate::models::{JavaBuild, JavaInfo};
use crate::repo::Repo;
use crate::utils::get_dirs;

fn path() -> PathBuf {
    get_dirs().data_dir().join("Java")
}

pub struct JavaRepo {
    lock: RwLock<()>,
}

#[async_trait]
impl Repo<JavaInfo, JavaBuild> for JavaRepo {
    async fn get(&self, id: impl AsRef<str> + Send) -> Option<JavaInfo> {
        todo!()
    }

    async fn list(&self) -> Vec<JavaInfo> {
        let lock = self.lock.read().await;
        todo!()
    }

    async fn install(&mut self, data: &JavaBuild) -> Result<JavaInfo> {
        let lock = self.lock.write().await;
        todo!()
    }

    async fn delete(&mut self, id: impl AsRef<str> + Send) -> Result<()> {
        let lock = self.lock.write().await;
        todo!()
    }
}
