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
use std::collections::HashMap;
use url::Url;

#[derive(Debug, Deserialize, Serialize)]
pub enum GameManifestMaybeArray<T> {
    Single(T),
    Multiple(Vec<T>),
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum GameManifestRuleAction {
    Allow,
    Disallow,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GameManifestOS {
    pub name: Option<String>,
    pub version: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GameManifestRule {
    pub action: GameManifestRuleAction,
    pub features: Option<HashMap<String, bool>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GameManifestConditional<T> {
    pub rules: Vec<GameManifestRule>,
    pub value: GameManifestMaybeArray<T>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GameManifestJavaVersion {
    // pub component: String,
    pub major_version: u8,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GameManifestAssetIndex {
    pub id: String,
    pub sha1: String,
    pub size: u64,
    pub total_size: u64,
    pub url: Url,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GameManifestDownloadable {
    pub sha1: String,
    pub size: u64,
    pub url: Url,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GameManifestDownloads {
    pub client: GameManifestDownloadable,
    pub server: GameManifestDownloadable,
    pub client_mappings: GameManifestDownloadable,
    pub server_mappings: GameManifestDownloadable,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct GameManifest {
    pub id: String,
    pub java_version: GameManifestJavaVersion,
    pub asset_index: GameManifestAssetIndex,
    pub assets: String,
    pub compliance_level: u8,
}
