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
use url::Url;

use macros::api_response;

use crate::silo::game::GameManifestStability;

#[api_response]
#[serde(rename_all = "camelCase")]
pub struct GameVersionLegacyJavaVersion {
    pub component: String,
    pub major_version: u64,
}

#[api_response]
#[serde(rename_all = "camelCase")]
pub struct AssetIndex {
    pub id: String,
    pub sha1: String,
    pub size: u64,
    pub total_size: u64,
    pub url: Url,
}

#[api_response]
pub struct Downloadable {
    pub sha1: String,
    pub size: u64,
    pub url: Url,
}

#[api_response(strict = false)]
pub struct Downloads {
    pub client: Downloadable,
    pub server: Option<Downloadable>,
    // pub client_mappings: Option<Downloadable>,
    // pub server_mappings: Option<Downloadable>,
}

#[api_response(untagged = false)]
#[serde(rename_all = "camelCase")]
pub enum LibraryRuleAction {
    Allow,
    Disallow,
}

#[api_response(untagged = false)]
#[serde(rename_all = "camelCase")]
pub enum Os {
    Linux,
    #[serde(rename = "osx")]
    MacOS,
    Windows,
}

#[api_response(untagged = false)]
pub enum Arch {
    #[serde(rename = "x86")]
    X86_64,
}

#[api_response]
pub struct LibraryRuleOs {
    pub name: Option<Os>,
    pub arch: Option<Arch>,
    pub version: Option<String>,
}

#[api_response]
pub struct LibraryRule {
    pub action: LibraryRuleAction,
    pub features: Option<HashMap<String, bool>>,
    pub os: Option<LibraryRuleOs>,
}

#[api_response]
pub struct LibraryArtifact {
    pub path: String,
    pub sha1: String,
    pub size: u64,
    pub url: Url,
}

#[api_response]
pub enum LibraryDownloads {
    WithArtifact {
        artifact: LibraryArtifact,
        classifiers: Option<HashMap<String, LibraryArtifact>>,
    },
    WithoutArtifact {
        classifiers: HashMap<String, LibraryArtifact>,
    },
}

#[api_response]
pub struct LibraryNativeKeys {
    pub linux: Option<String>,
    pub osx: Option<String>,
    pub windows: Option<String>,
}

#[api_response]
pub struct LibraryExtract {
    pub exclude: Option<Vec<String>>,
}

#[api_response]
pub struct Library {
    pub name: String,
    pub downloads: LibraryDownloads,
    pub rules: Option<Vec<LibraryRule>>,
    pub extract: Option<LibraryExtract>,
    pub natives: Option<LibraryNativeKeys>,
}

#[api_response]
pub struct LoggingDownloadable {
    pub id: String,
    pub sha1: String,
    pub size: u64,
    pub url: Url,
}

#[api_response]
pub struct LoggingClient {
    pub argument: String,
    pub file: LoggingDownloadable,
    #[serde(rename = "type")]
    pub logging_type: String,
}

#[api_response]
pub struct Logging {
    pub client: LoggingClient,
}

#[api_response]
#[serde(rename_all = "camelCase")]
pub struct GameVersionLegacy {
    pub asset_index: AssetIndex,
    pub assets: String,
    pub compliance_level: Option<u8>,
    pub downloads: Downloads,
    pub id: String,
    pub java_version: Option<GameVersionLegacyJavaVersion>,
    // assume java 8 if none? maybe check launcher version?
    pub main_class: String,
    pub minecraft_arguments: String,
    pub minimum_launcher_version: u64,
    pub release_time: DateTime<Utc>,
    pub time: DateTime<Utc>,
    #[serde(rename = "type")]
    pub stability: GameManifestStability,
    pub libraries: Vec<Library>,
    pub logging: Option<Logging>,
}
