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
    Artifact, AssetIndex, Downloads, JavaVersion, Logging, Rule, Stability,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CommonLibraryDownloads {
    pub artifact: Artifact,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct NativeLibraryDownloads {
    pub artifact: Option<Artifact>,
    pub classifiers: HashMap<String, Artifact>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CommonLibrary {
    pub name: String,
    pub downloads: CommonLibraryDownloads,
    pub rules: Option<Vec<Rule>>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct NativeKeys {
    pub linux: Option<String>,
    pub osx: Option<String>,
    pub windows: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ExtractOptions {
    pub exclude: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct NativeLibrary {
    pub name: String,
    pub downloads: NativeLibraryDownloads,
    pub natives: NativeKeys,
    pub extract: Option<ExtractOptions>,
    pub rules: Option<Vec<Rule>>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Library {
    Common(CommonLibrary),
    Native(NativeLibrary),
}

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
