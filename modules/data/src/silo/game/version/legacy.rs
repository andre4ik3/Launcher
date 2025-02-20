// Copyright © 2023-2025 andre4ik3
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

use crate::core::maven::MavenIdentifier;
use macros::api_response;

use crate::silo::game::GameManifestStability;

#[api_response(rename = "camelCase")]
pub struct ApiGameVersionLegacyJavaVersion {
    pub component: String,
    pub major_version: u64,
}

#[api_response(rename = "camelCase")]
pub struct ApiGameVersionAssetIndex {
    pub id: String,
    pub sha1: String,
    pub size: u64,
    pub total_size: u64,
    pub url: Url,
}

#[api_response]
pub struct ApiGameVersionDownloadable {
    pub sha1: String,
    pub size: u64,
    pub url: Url,
}

#[api_response(strict = false)]
pub struct ApiGameVersionDownloads {
    pub client: ApiGameVersionDownloadable,
    pub server: Option<ApiGameVersionDownloadable>,
    // pub client_mappings: Option<Downloadable>,
    // pub server_mappings: Option<Downloadable>,
}

#[api_response(untagged = false, rename = "camelCase")]
pub enum ApiLibraryRuleAction {
    Allow,
    Disallow,
}

#[api_response(untagged = false, rename = "camelCase")]
pub enum ApiOs {
    Linux,
    #[serde(rename = "osx")]
    MacOS,
    Windows,
}

#[api_response(untagged = false)]
pub enum ApiArch {
    #[serde(rename = "x86")]
    X86_64,
}

#[api_response]
pub struct ApiLibraryRuleOs {
    pub name: Option<ApiOs>,
    pub arch: Option<ApiArch>,
    pub version: Option<String>,
}

#[api_response]
pub struct ApiLibraryRule {
    pub action: ApiLibraryRuleAction,
    pub features: Option<HashMap<String, bool>>,
    pub os: Option<ApiLibraryRuleOs>,
}

#[api_response]
pub struct ApiLibraryArtifact {
    pub path: String,
    pub sha1: String,
    pub size: u64,
    pub url: Url,
}

#[api_response]
pub struct ApiCommonLibraryDownloads {
    pub artifact: ApiLibraryArtifact,
}

#[api_response]
pub struct ApiNativeLibraryDownloads {
    pub artifact: Option<ApiLibraryArtifact>,
    pub classifiers: HashMap<String, ApiLibraryArtifact>,
}

#[api_response]
pub struct ApiLibraryNativeKeys {
    pub linux: Option<String>,
    pub osx: Option<String>,
    pub windows: Option<String>,
}

#[api_response]
pub struct ApiLibraryExtract {
    pub exclude: Option<Vec<String>>,
}

#[api_response]
pub enum ApiLibrary {
    Common(ApiCommonLibrary),
    Native(ApiNativeLibrary),
}

#[api_response]
pub struct ApiCommonLibrary {
    pub name: MavenIdentifier,
    pub downloads: ApiCommonLibraryDownloads,
    pub rules: Option<Vec<ApiLibraryRule>>,
}

#[api_response]
pub struct ApiNativeLibrary {
    pub name: MavenIdentifier,
    pub downloads: ApiNativeLibraryDownloads,
    pub natives: ApiLibraryNativeKeys,
    pub extract: Option<ApiLibraryExtract>,
    pub rules: Option<Vec<ApiLibraryRule>>,
}

#[api_response]
pub struct ApiGameVersionLoggingDownloadable {
    pub id: String,
    pub sha1: String,
    pub size: u64,
    pub url: Url,
}

#[api_response]
pub struct ApiGameVersionLoggingClient {
    pub argument: String,
    pub file: ApiGameVersionLoggingDownloadable,
    #[serde(rename = "type")]
    pub logging_type: String,
}

#[api_response]
pub struct ApiGameVersionLogging {
    pub client: ApiGameVersionLoggingClient,
}

#[api_response(rename = "camelCase")]
pub struct ApiGameVersionLegacy {
    pub asset_index: ApiGameVersionAssetIndex,
    pub assets: String,
    pub compliance_level: Option<u8>,
    pub downloads: ApiGameVersionDownloads,
    pub id: String,
    pub java_version: Option<ApiGameVersionLegacyJavaVersion>,
    pub main_class: String,
    pub minecraft_arguments: String,
    pub minimum_launcher_version: u64,
    pub release_time: DateTime<Utc>,
    pub time: DateTime<Utc>,
    #[serde(rename = "type")]
    pub stability: GameManifestStability,
    pub libraries: Vec<ApiLibrary>,
    pub logging: Option<ApiGameVersionLogging>,
}
