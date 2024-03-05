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

use tokio::fs;
use data::core::game::{GameVersion, GameVersionSnippet};
use data::silo::game::{ApiGameVersion, GameManifest};

use crate::{client, vpath};
use crate::macros::write_to_ron_file;

const INDEX_URL: &str = "https://launchermeta.mojang.com/mc/game/version_manifest_v2.json";

pub async fn run(power_wash: bool) -> anyhow::Result<Vec<GameVersionSnippet>> {
    let client = client();
    let mut output = Vec::new();

    // First, retrieve the manifest of all available game versions.
    let manifest: GameManifest = client.get(INDEX_URL).await?.json().await?;

    // Then, for every version...
    for version in manifest.versions {
        let path = vpath!("game/{}.ron", version.id);

        // ...if we don't already have it (and we're not set to do a power wash)...
        if fs::try_exists(&path).await.unwrap_or(false) && !power_wash {
            if let Ok(data) = ron::from_str::<GameVersion>(&fs::read_to_string(&path).await?) {
                tracing::info!("Skipping version {} as it appears we already have it.", version.id);
                output.push(GameVersionSnippet::from(data));
                continue;
            } else {
                tracing::warn!("Removing malformed version {} at {}", version.id, path.display());
                fs::remove_file(&path).await?;
            }
        }

        // ...fetch the details of the game version...
        let data: ApiGameVersion = client.get(version.url.clone()).await?.json().await?;

        // ...convert it to a better format...
        let data = GameVersion::from(data);

        // ...and save it to disk.
        tracing::info!("Fetched version {}.", version.id);
        write_to_ron_file(&path, &data).await?;
        output.push(GameVersionSnippet::from(data));
    }

    // Finally, write an index of all available versions.
    write_to_ron_file(vpath!("game.ron"), &output).await?;
    tracing::info!("Loaded {} game versions", output.len());

    Ok(output)
}

#[cfg(test)]
mod tests {
    use data::silo::game::ApiGameVersion;
    use super::*;

    // TODO: Is it legal to include these in the repo? Maybe let's fetch them when testing?
    // const TEST_VERSIONS: [&str; 8] = [
        // include_str!("../../res/game/rd-132211.json"),
        // include_str!("../../res/game/13w38c.json"),
        // include_str!("../../res/game/14w27b.json"),
        // include_str!("../../res/game/16w04a.json"),
        // include_str!("../../res/game/1.12.2.json"),
        // include_str!("../../res/game/17w43a.json"),
        // include_str!("../../res/game/23w17a.json"),
        // include_str!("../../res/game/24w09a.json"),
    // ];

    #[tokio::test]
    async fn game_version() {
        // for string in TEST_VERSIONS {
        //     let version: ApiGameVersion = serde_json::from_str(string).expect("failed to deserialize game version");
        //     let version: GameVersion = version.into();
        //     println!("Parsed game version {}", version.id);
        //     assert_ne!(version.libraries.len(), 0);
        // }
    }
}
