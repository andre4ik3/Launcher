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

use std::path::Path;

use async_trait::async_trait;
use tokio::fs;
use tracing::info;

use data::core::game;
use data::silo::game::{GameManifest, GameVersion};

use crate::client;
use crate::macros::write_to_ron_file;

use super::Task;

const INDEX_URL: &str = "https://launchermeta.mojang.com/mc/game/version_manifest_v2.json";

/// This task retrieves a list of game versions, along with their respective metadata file URL.
pub struct TaskGameVersions;

#[async_trait]
impl Task for TaskGameVersions {
    type Input = ();

    /// An array that includes newly fetched game versions (not the ones from disk).
    type Output = Vec<game::GameVersion>;

    #[tracing::instrument(name = "TaskGameVersions", skip_all)]
    async fn run(root: impl AsRef<Path> + Send + Sync, _input: Self::Input) -> anyhow::Result<Self::Output> {
        let client = client().await;
        let mut output = Vec::new();

        // First, retrieve the manifest of all available game versions.
        let manifest: GameManifest = client.get(INDEX_URL).await?.json().await?;

        // Then, for every version...
        for version in manifest.versions {
            let path = root.as_ref().join(format!("game/{}.ron", version.id));
            // ...if we don't already have it...
            if fs::try_exists(&path).await.unwrap_or(false) {
                // TODO: Add a sort of "power wash" setting that fetches already existing versions.
                info!("Skipping version {} as it appears we already have it.", version.id);
                continue;
            }

            // ...fetch the details of the game version...
            let data: GameVersion = client.get(version.url.clone()).await?.json().await?;

            // ...convert it to a better format...
            let data = game::GameVersion::from(data);

            // ...and save it to disk.
            info!("Fetched version {}.", version.id);
            write_to_ron_file(&path, &data).await?;
            output.push(data);
        }

        info!("Loaded {} game versions", output.len());
        Ok(output)
    }
}
