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

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use launcher::models::{GameDownloadable, GameLibrary, GameMaybeConditional, GameVersion};

use crate::models::game::common::{
    AssetIndex, Downloads, JavaVersion, Library, Logging, MaybeConditional, Stability,
};

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct _Arguments {
    pub game: Vec<MaybeConditional<String>>,
    pub jvm: Vec<MaybeConditional<String>>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct GameManifest17w43a {
    pub id: String,
    pub downloads: Downloads,
    #[serde(rename = "type")]
    pub stability: Stability,
    pub java_version: JavaVersion,
    pub compliance_level: Option<u8>,
    pub assets: String,
    pub asset_index: AssetIndex,
    pub libraries: Vec<Library>,
    pub main_class: String,
    pub arguments: _Arguments,
    pub minimum_launcher_version: u64,
    pub release_time: DateTime<Utc>,
    pub time: DateTime<Utc>,
    pub logging: Option<Logging>,
}

#[allow(clippy::from_over_into)]
impl Into<GameVersion> for GameManifest17w43a {
    fn into(self) -> GameVersion {
        let libraries = self
            .libraries
            .into_iter()
            .flat_map::<Vec<GameMaybeConditional<GameLibrary>>, _>(Library::into)
            .collect();

        let arguments: Box<[GameMaybeConditional<String>]> = self
            .arguments
            .game
            .into_iter()
            .map(|arg| arg.into())
            .collect();

        let arguments_java: Box<[GameMaybeConditional<String>]> = self
            .arguments
            .jvm
            .into_iter()
            .map(|arg| arg.into())
            .collect();

        GameVersion {
            stability: self.stability.into(),
            java: self.java_version.into(),
            entrypoint: self.main_class,
            released: self.release_time,
            arguments,
            arguments_java,
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
