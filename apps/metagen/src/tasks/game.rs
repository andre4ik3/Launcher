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

use crate::models::game::{GameManifest, GameVersionInfoIndex};
use crate::utils::{dump, prog_style};
use crate::CLIENT;
use anyhow::Result;
use indicatif::ProgressBar;
use launcher::models::game::{GameVersion, GameVersionIndex};
use std::collections::HashSet;

const INDEX_URL: &str = "https://launchermeta.mojang.com/mc/game/version_manifest_v2.json";

/// Collects metadata about *all* available game versions and writes them to disk to respective
/// files. Also collects the Java requirement for each version and returns them as a [HashSet].
pub async fn run(skip: &[String]) -> Result<HashSet<u8>> {
    println!("Generating game metadata...");
    let resp = CLIENT.get(INDEX_URL).send().await?.error_for_status()?;
    let mut resp: GameVersionInfoIndex = resp.json().await?;

    let index: GameVersionIndex = resp.clone().into();
    dump("versions.ron", &index).await?;

    resp.versions.reverse(); // Oldest -> newest
    let mut versions: HashSet<u8> = HashSet::new();

    let pb = ProgressBar::new(resp.versions.len() as u64);
    pb.set_style(prog_style());

    for version in resp.versions {
        pb.inc(1);
        pb.set_message(format!("Game {}", version.id));

        if skip.contains(&version.id) {
            continue;
        }

        let data = CLIENT.get(version.url).send().await?.error_for_status()?;
        let data: GameManifest = data.json().await?;
        let data: GameVersion = data.into();

        // Hopefully there's not more than 255 Java versions
        versions.insert(data.java.comparators[0].major as u8);
        dump(format!("versions/{}.ron", version.id), &data).await?;
    }

    pb.finish_and_clear();
    Ok(versions)
}
