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

use std::collections::HashSet;
use std::{env::consts, str::FromStr};

use platforms::{Arch, OS};
use semver::{Version, VersionReq};
use serde::{Deserialize, Serialize};
use url::Url;

/// Condition for inclusion of arguments and libraries.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub enum Condition {
    Feature(String),
    OS((OS, VersionReq)),
    Arch(Arch),
    And(Box<[Condition]>),
    Or(Box<[Condition]>),
    Not(Box<Condition>),
}

impl Condition {
    /// Evaluates a condition to true or false.
    pub fn eval(&self, features: &[String]) -> bool {
        match self {
            Self::Feature(feat) => features.contains(feat),
            Self::OS((os, req)) => {
                let os_matches = os == &OS::from_str(consts::OS).unwrap();
                let ver_matches = match os_info::get().version() {
                    os_info::Version::Semantic(major, minor, patch) => {
                        req.matches(&Version::new(*major, *minor, *patch))
                    }
                    _ => true, // Unimplemented
                };
                os_matches && ver_matches
            }
            Self::Arch(arch) => arch == &Arch::from_str(consts::ARCH).unwrap(),
            Self::And(vals) => vals.iter().map(|v| v.eval(features)).all(|v| v),
            Self::Or(vals) => vals.iter().map(|v| v.eval(features)).any(|v| v),
            Self::Not(val) => !val.eval(features),
        }
    }

    /// Checks if a condition is empty (aka a no-op). Used to simplify expressions.
    pub fn is_empty(&self) -> bool {
        match self {
            Self::And(vals) | Self::Or(vals) => vals.is_empty(),
            Self::Not(val) => val.is_empty(),
            _ => false,
        }
    }

    /// Simplifies a condition.
    pub fn simplify(self) -> Self {
        match self {
            Self::And(vals) => {
                let vals: HashSet<Condition> = vals
                    .iter()
                    .cloned()
                    .filter(|condition| !condition.is_empty())
                    .map(Self::simplify)
                    .collect();

                let vals: Box<[Condition]> = vals.into_iter().collect();
                if vals.len() == 1 {
                    vals[0].clone()
                } else {
                    Self::And(vals)
                }
            }
            Self::Or(vals) => {
                let vals: HashSet<Condition> = vals
                    .iter()
                    .cloned()
                    .filter(|condition| !condition.is_empty())
                    .map(Self::simplify)
                    .collect();

                let vals: Box<[Condition]> = vals.into_iter().collect();
                if vals.len() == 1 {
                    vals[0].clone()
                } else {
                    Self::Or(vals)
                }
            }
            Self::Not(val) => Self::Not(Box::from(val.simplify())),
            _ => self,
        }
    }
}

/// A condition for including game arguments and libraries, similar to an `if` statement.
#[derive(Debug, Deserialize, Serialize)]
pub struct GameConditional<T> {
    /// The condition to match.
    pub when: Condition,
    /// The structs to include when the condition is matched.
    pub then: Box<[T]>,
}

impl<T> GameConditional<T> {
    /// Evaluates a condition, collapsing it to the inner value or None.
    pub fn eval(self, features: &[String]) -> Option<Box<[T]>> {
        if self.when.eval(features) {
            Some(self.then)
        } else {
            None
        }
    }
}

/// Helper for including both conditional and unconditional types in one type.
#[derive(Debug, Deserialize, Serialize)]
pub enum GameMaybeConditional<T> {
    Unconditional(T),
    Conditional(GameConditional<T>),
}

/// The different stability types of game releases.
#[derive(Debug, Deserialize, Serialize)]
pub enum GameVersionStability {
    Release,
    Snapshot,
    Experimental,
    OldBeta,
    OldAlpha,
}

/// Abstract type for a downloadable game file.
#[derive(Debug, Deserialize, Serialize)]
pub struct GameDownloadable {
    /// The relative path where the file will be saved.
    pub path: String,
    /// The SHA1 checksum of the file.
    pub checksum: String,
    /// The size of the file in bytes.
    pub size: u64,
    /// The URL of the file.
    pub url: Url,
}

/// Information and location of the version's asset index.
#[derive(Debug, Deserialize, Serialize)]
pub struct GameAssetIndex {
    /// The ID of the asset index.
    pub id: String,
    /// The combined size of all downloaded and extracted assets.
    pub total_size: u64,
    /// The asset index file.
    pub file: GameDownloadable,
}

/// Information and location of a version's library.
#[derive(Debug, Deserialize, Serialize)]
pub struct GameLibrary {
    /// The name of the library, in this format: `com.example:hello:1.0`
    pub name: String,
    /// The library file.
    pub file: GameDownloadable,
}

/// Information needed to launch a particular game version.
#[derive(Debug, Deserialize, Serialize)]
pub struct GameVersion {
    /// The unique game version. Note: doesn't follow SemVer.
    pub version: String,
    /// The stability of this game version.
    pub stability: GameVersionStability,
    /// The version of Java that the game requires.
    pub java: VersionReq,
    /// The Java class that serves as the entry point for this version.
    pub entrypoint: String,
    /// UTC date and time of release of this version.
    pub released: String,
    /// Arguments to be passed to the game.
    pub arguments: Box<[GameMaybeConditional<String>]>,
    /// Arguments to be passed to the Java virtual machine.
    pub arguments_java: Box<[GameMaybeConditional<String>]>,
    /// Information about the game's asset index.
    pub assets: GameAssetIndex,
    /// The libraries that the game depends on.
    pub libraries: Box<[GameMaybeConditional<GameLibrary>]>,
    /// Downloads for the main game files.
    pub client: GameDownloadable,
}

/// Surface-level information about a game version.
#[derive(Debug, Deserialize, Serialize)]
pub struct GameVersionSnippet {
    /// The unique game version. Note: doesn't follow SemVer.
    pub version: String,
    /// The stability of this game version.
    pub stability: GameVersionStability,
    /// UTC date and time of release of this version.
    pub released: String,
}

/// Information about available game versions.
#[derive(Debug, Deserialize, Serialize)]
pub struct GameVersionIndex {
    /// The latest available version ID with "Release" stability.
    pub latest_release: String,
    /// The latest available version ID with "Snapshot" stability.
    pub latest_snapshot: String,
    /// All game versions in order of release date (newest on top).
    pub versions: Box<[GameVersionSnippet]>,
}
