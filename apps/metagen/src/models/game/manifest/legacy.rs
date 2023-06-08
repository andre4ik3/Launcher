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

use crate::models::game::common::{
    AssetIndex, Downloads, JavaVersion, Library, Logging, Stability,
};
use chrono::{DateTime, Utc};
use launcher::models::game::{GameDownloadable, GameLibrary, GameMaybeConditional, GameVersion};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct GameManifestLegacy {
    pub id: String,
    pub downloads: Downloads,
    #[serde(rename = "type")]
    pub stability: Stability,
    pub java_version: Option<JavaVersion>,
    pub compliance_level: Option<u8>,
    pub assets: String,
    pub asset_index: AssetIndex,
    pub libraries: Vec<Library>,
    pub main_class: String,
    pub minecraft_arguments: String,
    pub minimum_launcher_version: u64,
    pub release_time: DateTime<Utc>,
    pub time: DateTime<Utc>,
    pub logging: Option<Logging>,
}

#[allow(clippy::from_over_into)]
impl Into<GameVersion> for GameManifestLegacy {
    fn into(self) -> GameVersion {
        let arguments: Box<[GameMaybeConditional<String>]> = self
            .minecraft_arguments
            .split(' ')
            .map(|a| GameMaybeConditional::Unconditional(a.to_string()))
            .collect();

        let libraries = self
            .libraries
            .into_iter()
            .flat_map::<Vec<GameMaybeConditional<GameLibrary>>, _>(Library::into)
            .collect();

        GameVersion {
            stability: self.stability.into(),
            java: self
                .java_version
                .unwrap_or(JavaVersion {
                    major_version: 8,
                    component: "unknown".to_owned(),
                })
                .into(),
            entrypoint: self.main_class,
            released: self.release_time,
            arguments,
            arguments_java: Box::new([]), // TODO
            assets: self.asset_index.into(),
            libraries,
            client: GameDownloadable {
                path: format!("{}.jar", self.id),
                checksum: self.downloads.client.sha1,
                size: self.downloads.client.size,
                url: self.downloads.client.url,
            },
            version: self.id,
        }
    }
}
