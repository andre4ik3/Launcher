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
use serde::{Deserialize, Serialize};
use url::Url;

use launcher::models::game::{GameVersionIndex, GameVersionSnippet};

use crate::models::game::common::Stability;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct _GameVersionInfo {
    pub id: String,
    #[serde(rename = "type")]
    pub stability: Stability,
    pub url: Url,
    pub time: DateTime<Utc>,
    pub release_time: DateTime<Utc>,
    pub sha1: String,
    pub compliance_level: u8,
}

#[allow(clippy::from_over_into)]
impl Into<GameVersionSnippet> for _GameVersionInfo {
    fn into(self) -> GameVersionSnippet {
        GameVersionSnippet {
            version: self.id,
            stability: self.stability.into(),
            released: self.release_time,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct _GameVersionInfoIndexLatest {
    pub release: String,
    pub snapshot: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GameVersionInfoIndex {
    pub latest: _GameVersionInfoIndexLatest,
    pub versions: Vec<_GameVersionInfo>,
}

#[allow(clippy::from_over_into)]
impl Into<GameVersionIndex> for GameVersionInfoIndex {
    fn into(self) -> GameVersionIndex {
        GameVersionIndex {
            latest_release: self.latest.release,
            latest_snapshot: self.latest.snapshot,
            versions: self
                .versions
                .into_iter()
                .map(_GameVersionInfo::into)
                .collect(),
        }
    }
}
