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

use async_trait::async_trait;
use data::silo::game::{GameVersionManifest, GameVersionManifestEntry};
use tracing::info;

use super::Task;
use crate::client;

const INDEX_URL: &str = "https://launchermeta.mojang.com/mc/game/version_manifest_v2.json";

/// This task retrieves a list of game versions, along with their respective metadata file URL.
pub struct TaskGameVersions;

#[async_trait]
impl Task for TaskGameVersions {
    type Input = ();
    type Output = Vec<GameVersionManifestEntry>;

    #[tracing::instrument(name = "TaskGameVersions", skip_all)]
    async fn run(_input: Self::Input) -> anyhow::Result<Self::Output> {
        let manifest: GameVersionManifest = client().await.get(INDEX_URL).await?.json().await?;
        info!("Loaded {} game versions", manifest.versions.len());
        Ok(manifest.versions)
    }
}
