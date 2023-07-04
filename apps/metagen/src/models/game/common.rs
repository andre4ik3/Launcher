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
use std::iter::once;

use platforms::{Arch, OS};
use semver::{Comparator, Op, Prerelease, VersionReq};
use serde::{Deserialize, Serialize};
use url::Url;

use launcher::models::{
    Condition, GameAssetIndex, GameConditional, GameDownloadable, GameLibrary,
    GameMaybeConditional, GameVersionStability,
};

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

#[allow(clippy::from_over_into)]
impl Into<GameMaybeConditional<GameLibrary>> for CommonLibrary {
    fn into(self) -> GameMaybeConditional<GameLibrary> {
        match self.rules {
            None => GameMaybeConditional::Unconditional(GameLibrary {
                name: self.name,
                file: GameDownloadable {
                    path: self.downloads.artifact.path,
                    checksum: self.downloads.artifact.sha1,
                    size: self.downloads.artifact.size,
                    url: self.downloads.artifact.url,
                },
            }),
            Some(rules) => GameMaybeConditional::Conditional(GameConditional {
                when: Condition::And(rules.into_iter().map(Rule::into).collect()).simplify(),
                then: Box::new([GameLibrary {
                    name: self.name,
                    file: self.downloads.artifact.into(),
                }]),
            }),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct NativeLibraryKeys {
    pub linux: Option<String>,
    pub osx: Option<String>,
    pub windows: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LibraryExtractOptions {
    pub exclude: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct NativeLibrary {
    pub name: String,
    pub downloads: NativeLibraryDownloads,
    pub natives: NativeLibraryKeys,
    pub extract: Option<LibraryExtractOptions>,
    pub rules: Option<Vec<Rule>>,
}

#[allow(clippy::from_over_into)]
impl Into<Vec<GameMaybeConditional<GameLibrary>>> for NativeLibrary {
    fn into(mut self) -> Vec<GameMaybeConditional<GameLibrary>> {
        let mut libraries: Vec<GameMaybeConditional<GameLibrary>> = vec![];

        // TODO: Make this more readable
        let keys: Vec<(String, OS)> =
            vec![self.natives.linux, self.natives.osx, self.natives.windows]
                .into_iter()
                .zip(vec![OS::Linux, OS::MacOS, OS::Windows])
                .filter(|(key, _)| key.is_some())
                .map(|(key, os)| (key.unwrap(), os))
                .collect();

        // Base condition for the library (if exists).
        let when: Option<Vec<Condition>> = self.rules.map(|s| {
            s.into_iter()
                .map(Rule::into)
                .map(Condition::simplify)
                .collect()
        });

        for (key, os) in keys {
            // Mojang classic. In some libraries, there are native keys, but no corresponding
            // artifact in downloads. See, for example, second library in rd-132211.
            let artifact = match self.downloads.classifiers.remove(&key) {
                Some(artifact) => artifact,
                None => continue,
            };

            // TODO: In some cases, there are two conditions back-to-back, e.g. (macos, *) and (macos, =10.5).
            // They should be de-duped. Preferably in Condition::simplify.
            let condition = Condition::OS((os, VersionReq::STAR));
            let when = match when.clone() {
                Some(when) => {
                    Condition::And(when.into_iter().chain(once(condition)).collect()).simplify()
                }
                None => Condition::OS((os, VersionReq::STAR)),
            };

            libraries.push(GameMaybeConditional::Conditional(GameConditional {
                when,
                then: Box::new([GameLibrary {
                    name: self.name.clone(),
                    file: artifact.into(),
                }]),
            }));
        }

        if let Some(artifact) = self.downloads.artifact {
            libraries.push(GameMaybeConditional::Unconditional(GameLibrary {
                name: self.name.clone(),
                file: artifact.into(),
            }));
        }

        libraries
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Library {
    Common(CommonLibrary),
    Native(NativeLibrary),
}

#[allow(clippy::from_over_into)]
impl Into<Vec<GameMaybeConditional<GameLibrary>>> for Library {
    fn into(self) -> Vec<GameMaybeConditional<GameLibrary>> {
        match self {
            Library::Common(val) => vec![val.into()],
            Library::Native(val) => val.into(),
        }
    }
}

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

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Stability {
    Release,
    Snapshot,
    OldBeta,
    OldAlpha,
}

#[allow(clippy::from_over_into)]
impl Into<GameVersionStability> for Stability {
    fn into(self) -> GameVersionStability {
        match self {
            Stability::Release => GameVersionStability::Release,
            Stability::Snapshot => GameVersionStability::Snapshot,
            Stability::OldBeta => GameVersionStability::OldBeta,
            Stability::OldAlpha => GameVersionStability::OldAlpha,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct JavaVersion {
    pub component: String,
    pub major_version: u8,
}

#[allow(clippy::from_over_into)]
impl Into<VersionReq> for JavaVersion {
    fn into(self) -> VersionReq {
        VersionReq {
            comparators: vec![Comparator {
                op: match self.major_version {
                    8 => Op::Exact, // Versions on Java 8 don't tend to play nice with modern versions
                    _ => Op::GreaterEq,
                },
                major: self.major_version as u64,
                minor: None,
                patch: None,
                pre: Prerelease::EMPTY,
            }],
        }
    }
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

#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AssetIndex {
    pub id: String,
    pub sha1: String,
    pub size: u64,
    pub total_size: u64,
    pub url: Url,
}

#[allow(clippy::from_over_into)]
impl Into<GameAssetIndex> for AssetIndex {
    fn into(self) -> GameAssetIndex {
        GameAssetIndex {
            id: self.id.clone(),
            total_size: self.total_size,
            file: GameDownloadable {
                path: format!("{}.json", self.id),
                checksum: self.sha1,
                size: self.size,
                url: self.url,
            },
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Artifact {
    pub path: String,
    pub sha1: String,
    pub size: u64,
    pub url: Url,
}

#[allow(clippy::from_over_into)]
impl Into<GameDownloadable> for Artifact {
    fn into(self) -> GameDownloadable {
        GameDownloadable {
            path: self.path,
            checksum: self.sha1,
            size: self.size,
            url: self.url,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum MaybeArray<T> {
    Single(T),
    Multiple(Box<[T]>),
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

#[allow(clippy::from_over_into)]
impl Into<Condition> for Rule {
    fn into(self) -> Condition {
        let features: Vec<Condition> = self
            .features
            .map(|m| m.into_keys().map(Condition::Feature).collect())
            .unwrap_or_default();

        let os: Vec<Condition> = self
            .os
            .map(|os| {
                let mut conditions: Vec<Condition> = vec![];

                if let Some(arch) = os.arch {
                    conditions.push(Condition::Arch(match arch.as_str() {
                        "x86" => Arch::X86,
                        _ => unreachable!("No such arch"),
                    }));
                }

                if let Some(name) = os.name {
                    let name = match name.as_str() {
                        "osx" => OS::MacOS,
                        "windows" => OS::Windows,
                        "linux" => OS::Linux,
                        _ => unreachable!("No such OS"),
                    };
                    let version = match os.version {
                        Some(version) => match version.as_str() {
                            "^10\\.5\\.\\d$" => VersionReq::parse("=10.5").unwrap(),
                            "^10\\." => VersionReq::parse("=10").unwrap(),
                            _ => unreachable!("No such version requirement"),
                        },
                        None => VersionReq::STAR,
                    };
                    conditions.push(Condition::OS((name, version)));
                }

                conditions
            })
            .unwrap_or_default();

        let conditions = features.into_iter().chain(os.into_iter()).collect();
        let condition = Condition::And(conditions).simplify();

        match self.action {
            RuleAction::Allow => condition,
            RuleAction::Disallow => Condition::Not(Box::from(condition)),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RuleConditional<T> {
    pub rules: Vec<Rule>,
    pub value: MaybeArray<T>,
}

#[allow(clippy::from_over_into)]
impl<T> Into<GameConditional<T>> for RuleConditional<T> {
    fn into(self) -> GameConditional<T> {
        GameConditional {
            when: Condition::And(self.rules.into_iter().map(Rule::into).collect()).simplify(),
            then: match self.value {
                MaybeArray::Single(v) => Box::from([v]),
                MaybeArray::Multiple(v) => v,
            },
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum MaybeConditional<T> {
    Unconditional(T),
    Conditional(RuleConditional<T>),
}

#[allow(clippy::from_over_into)]
impl<T> Into<GameMaybeConditional<T>> for MaybeConditional<T> {
    fn into(self) -> GameMaybeConditional<T> {
        match self {
            MaybeConditional::Unconditional(val) => GameMaybeConditional::Unconditional(val),
            MaybeConditional::Conditional(val) => GameMaybeConditional::Conditional(val.into()),
        }
    }
}
