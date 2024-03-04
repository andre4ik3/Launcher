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

use macros::api_response;

use crate::silo::game::{
    GameVersionAssetIndex, Downloads, GameManifestStability, GameVersionLegacyJavaVersion, Library,
    LibraryRule, Logging,
};

#[api_response]
pub enum ModernGameRuleValue {
    String(String),
    Array(Vec<String>),
}

#[api_response]
pub enum ModernGameArgument {
    Plain(String),
    Conditional {
        rules: Vec<LibraryRule>,
        value: ModernGameRuleValue,
    },
}

#[api_response]
pub struct ModernGameArguments {
    pub game: Vec<ModernGameArgument>,
    pub jvm: Vec<ModernGameArgument>,
}

#[api_response(rename = "camelCase")]
pub struct GameVersion17w43a {
    pub arguments: ModernGameArguments,
    pub asset_index: GameVersionAssetIndex,
    pub assets: String,
    pub compliance_level: u8,
    pub downloads: Downloads,
    pub id: String,
    pub java_version: GameVersionLegacyJavaVersion,
    pub main_class: String,
    pub minimum_launcher_version: u64,
    pub release_time: DateTime<Utc>,
    pub time: DateTime<Utc>,
    #[serde(rename = "type")]
    pub stability: GameManifestStability,
    pub libraries: Vec<Library>,
    pub logging: Logging,
}
