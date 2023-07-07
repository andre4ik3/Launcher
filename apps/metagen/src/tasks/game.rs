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

use std::collections::HashSet;
use std::path::PathBuf;

use tokio::fs;

use anyhow::Result;
use indicatif::ProgressBar;

use launcher::models::{GameVersion, GameVersionIndex};

use crate::models::game::{AssetIndex, GameManifest, GameVersionInfoIndex};
use crate::utils::{dump, prog_style};
use crate::CLIENT;

const INDEX_URL: &str = "https://launchermeta.mojang.com/mc/game/version_manifest_v2.json";

/// Collects metadata about *all* available game versions and writes them to disk to respective
/// files. Also collects the Java requirement for each version and returns them as a [HashSet].
pub async fn run() -> Result<(HashSet<u8>, HashSet<AssetIndex>)> {
    println!("Generating game metadata...");
    let resp = CLIENT.get(INDEX_URL).send().await?.error_for_status()?;
    let mut resp: GameVersionInfoIndex = resp.json().await?;

    let index: GameVersionIndex = resp.clone().into();
    dump("game.ron", &index).await?;

    resp.versions.reverse(); // Oldest -> newest
    let mut versions: HashSet<u8> = HashSet::new();
    let mut assets: HashSet<AssetIndex> = HashSet::new();

    let pb = ProgressBar::new(resp.versions.len() as u64);
    pb.set_style(prog_style());

    for version in resp.versions {
        pb.inc(1);
        pb.set_message(format!("Game {}", version.id));

        let path: PathBuf = format!("_site/v1/game/{}.ron", version.id).into();
        if !path.exists() {
            let data = CLIENT.get(version.url).send().await?.error_for_status()?;
            let data: GameManifest = data.json().await?;
            assets.insert(data.asset_index());
            let data: GameVersion = data.into();

            // Hopefully there's not more than 255 Java versions
            versions.insert(data.java.comparators[0].major as u8);
            dump(path, &data).await?;
        } else {
            let data = fs::read_to_string(path).await?;
            let data: GameVersion = ron::from_str(&data)?;
            assets.insert(AssetIndex {
                id: data.assets.id,
                sha1: data.assets.file.checksum,
                size: data.assets.file.size,
                total_size: data.assets.total_size,
                url: data.assets.file.url,
            });
        }
    }

    pb.finish_and_clear();
    Ok((versions, assets))
}
