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

use semver::Op;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use url::Url;

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LoggingFile {
    pub id: String,
    pub sha1: String,
    pub size: u64,
    pub url: Url,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ClientLogging {
    pub argument: String,
    pub file: LoggingFile,
    #[serde(rename = "type")]
    pub logging_type: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Logging {
    pub client: ClientLogging,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Stability {
    Release,
    Snapshot,
    OldBeta,
    OldAlpha,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct JavaVersion {
    pub component: String,
    pub major_version: u8,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Downloadable {
    pub sha1: String,
    pub size: u64,
    pub url: Url,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Downloads {
    pub client: Downloadable,
    pub server: Option<Downloadable>,
    pub client_mappings: Option<Downloadable>,
    pub server_mappings: Option<Downloadable>,
    pub windows_server: Option<Downloadable>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AssetIndex {
    pub id: String,
    pub sha1: String,
    pub size: u64,
    pub total_size: u64,
    pub url: Url,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Artifact {
    pub path: String,
    pub sha1: String,
    pub size: u64,
    pub url: Url,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum MaybeArray<T> {
    Single(T),
    Multiple(Vec<T>),
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum RuleAction {
    Allow,
    Disallow,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RuleOS {
    pub arch: Option<String>,
    pub name: Option<String>,
    pub version: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Rule {
    pub action: RuleAction,
    pub features: Option<HashMap<String, bool>>,
    pub os: Option<RuleOS>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RuleConditional<T> {
    pub rules: Vec<Rule>,
    pub value: T,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum MaybeConditional<T> {
    Unconditional(T),
    Conditional(RuleConditional<T>),
}
