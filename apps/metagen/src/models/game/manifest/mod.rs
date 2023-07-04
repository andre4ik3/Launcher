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

use serde::{Deserialize, Serialize};

use launcher::models::GameVersion;

use crate::models::game::common::AssetIndex;

pub mod legacy;
pub mod v17w43a;

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum GameManifest {
    Legacy(legacy::GameManifestLegacy),
    Modern(v17w43a::GameManifest17w43a),
}

#[allow(clippy::from_over_into)]
impl GameManifest {
    pub fn asset_index(&self) -> AssetIndex {
        match self {
            GameManifest::Legacy(manifest) => manifest.asset_index.clone(),
            GameManifest::Modern(manifest) => manifest.asset_index.clone(),
        }
    }
}

#[allow(clippy::from_over_into)]
impl Into<GameVersion> for GameManifest {
    fn into(self) -> GameVersion {
        match self {
            GameManifest::Legacy(manifest) => manifest.into(),
            GameManifest::Modern(manifest) => manifest.into(),
        }
    }
}
