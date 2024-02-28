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
use url::Url;

use macros::api_response;

#[api_response(untagged = false)]
#[serde(rename_all = "snake_case")]
pub enum GameManifestStability {
    Release,
    Snapshot,
    OldBeta,
    OldAlpha,
}

#[api_response]
pub struct GameManifestLatest {
    pub release: String,
    pub snapshot: String,
}

#[api_response]
#[serde(rename_all = "camelCase")]
pub struct GameManifestEntry {
    pub id: String,
    #[serde(rename = "type")]
    pub stability: GameManifestStability,
    pub url: Url,
    pub time: DateTime<Utc>,
    pub release_time: DateTime<Utc>,
    pub sha1: String,
    pub compliance_level: u8,
}

#[api_response]
pub struct GameManifest {
    pub latest: GameManifestLatest,
    pub versions: Vec<GameManifestEntry>,
}
