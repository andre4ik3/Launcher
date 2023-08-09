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

use chrono::{DateTime, Utc};
use serde::Deserialize;
use url::Url;

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GameVersionManifestStability {
    Release,
    Snapshot,
    OldBeta,
    OldAlpha,
}

#[derive(Clone, Debug, Deserialize)]
pub struct GameVersionManifestLatest {
    pub release: String,
    pub snapshot: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameVersionManifestEntry {
    pub id: String,
    #[serde(rename = "type")]
    pub stability: GameVersionManifestStability,
    pub url: Url,
    pub time: DateTime<Utc>,
    pub release_time: DateTime<Utc>,
    pub sha1: String,
    pub compliance_level: u8,
}

#[derive(Clone, Debug, Deserialize)]
pub struct GameVersionManifest {
    pub latest: GameVersionManifestLatest,
    pub versions: Vec<GameVersionManifestEntry>,
}
