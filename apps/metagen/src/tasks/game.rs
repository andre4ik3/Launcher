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
use crate::CLIENT;
use anyhow::Result;
use launcher::models::game::GameVersion;

const INDEX_URL: &str = "https://launchermeta.mojang.com/mc/game/version_manifest_v2.json";

pub async fn run() -> Result<()> {
    let resp = CLIENT.get(INDEX_URL).send().await?.error_for_status()?;
    let mut resp: GameVersionInfoIndex = resp.json().await?;

    resp.versions.reverse();
    let opts = ron::Options::default();
    let pretty = ron::ser::PrettyConfig::default().struct_names(true);

    for version in resp.versions {
        let data = CLIENT.get(version.url).send().await?.error_for_status()?;
        let data: GameManifest = data.json().await?;
        let data: GameVersion = data.into();
        let data = opts.to_string_pretty(&data, pretty.clone()).unwrap();
    }

    Ok(())
}
