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

use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::Deserialize;
use url::Url;

use crate::silo::game::GameManifestStability;

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct GameVersionLegacyJavaVersion {
    pub component: String,
    pub major_version: u8,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct AssetIndex {
    pub id: String,
    pub sha1: String,
    pub size: u64,
    pub total_size: u64,
    pub url: Url,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Downloadable {
    pub sha1: String,
    pub size: u64,
    pub url: Url,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Downloads {
    pub client: Downloadable,
    pub server: Option<Downloadable>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub enum LibraryRuleAction {
    Allow,
    Disallow,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum Os {
    #[serde(rename = "osx")]
    MacOS,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LibraryRuleOs {
    pub name: Os,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LibraryRule {
    pub action: LibraryRuleAction,
    pub os: Option<LibraryRuleOs>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LibraryArtifact {
    pub path: String,
    pub sha1: String,
    pub size: u64,
    pub url: Url,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields, untagged)]
pub enum LibraryDownloads {
    WithArtifact {
        artifact: LibraryArtifact,
        classifiers: Option<HashMap<String, LibraryArtifact>>,
    },
    WithoutArtifact {
        classifiers: HashMap<String, LibraryArtifact>,
    },
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LibraryNativeKeys {
    pub linux: Option<String>,
    pub osx: Option<String>,
    pub windows: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LibraryExtract {
    pub exclude: Option<Vec<String>>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Library {
    pub name: String,
    pub downloads: LibraryDownloads,
    pub rules: Option<Vec<LibraryRule>>,
    pub extract: Option<LibraryExtract>,
    pub natives: Option<LibraryNativeKeys>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LoggingDownloadable {
    pub id: String,
    pub sha1: String,
    pub size: u64,
    pub url: Url,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LoggingClient {
    pub argument: String,
    pub file: LoggingDownloadable,
    #[serde(rename = "type")]
    pub logging_type: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Logging {
    pub client: LoggingClient,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct GameVersionLegacy {
    pub asset_index: AssetIndex,
    pub assets: String,
    pub compliance_level: u8,
    pub downloads: Downloads,
    pub id: String,
    pub java_version: GameVersionLegacyJavaVersion,
    pub main_class: String,
    pub minecraft_arguments: String,
    pub minimum_launcher_version: u64,
    pub release_time: DateTime<Utc>,
    pub time: DateTime<Utc>,
    #[serde(rename = "type")]
    pub stability: GameManifestStability,
    pub libraries: Vec<Library>,
    pub logging: Logging,
}
