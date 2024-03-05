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
use platforms::OS;
use semver::{Comparator, Op, Prerelease, VersionReq};
use serde::{Deserialize, Serialize};

use super::conditional::{Condition, MaybeConditional};
use super::library::Library;

/// The different classes of game versions (e.g. release vs snapshot).
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum GameVersionStability {
    /// Stable versions that have been released to the public.
    Release,
    /// Previews of future versions that may contain bugs and are subject to change.
    Snapshot,
    /// Old beta versions before the game was released.
    OldBeta,
    /// Old alpha (so-called "classic") versions before the game was released.
    OldAlpha,
}

/// Detailed information about a specific game version (including needed libraries, asset index
/// version, Java version, etc.).
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GameVersion {
    /// The ID or "name" of this version (e.g. "1.12.2" or "23w32a").
    pub id: String,
    /// The date and time when this version was released.
    pub release_date: DateTime<Utc>,
    /// The stability of this version (e.g. release vs snapshot).
    pub stability: GameVersionStability,
    /// The version of Java required to launch this version.
    pub java_version: VersionReq,
    /// The main Java class that contains the game's `main()` function.
    pub main_class: String,
    /// A list of required libraries to run this version.
    pub libraries: Vec<MaybeConditional<Library>>,
    /// A list of arguments that should be passed before `main_class`. The arguments can contain
    /// variables enclosed in `${}`, which should be replaced.
    pub java_arguments: Vec<MaybeConditional<String>>,
    /// A list of arguments that should be passed after `main_class`. The arguments can contain
    /// variables enclosed in `${}`, which should be replaced.
    pub game_arguments: Vec<MaybeConditional<String>>,
}

/// A small snippet of game version information that is used in the [GameVersionIndex].
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GameVersionSnippet {
    /// The ID or "name" of this version (e.g. "1.12.2" or "23w32a").
    pub id: String,
    /// The stability of this version (e.g. release vs snapshot).
    pub stability: GameVersionStability,
}

// === conversion ===

impl From<GameVersion> for GameVersionSnippet {
    fn from(value: GameVersion) -> Self {
        Self {
            id: value.id,
            stability: value.stability,
        }
    }
}

#[cfg(feature = "silo")]
impl From<crate::silo::game::GameManifestStability> for GameVersionStability {
    fn from(value: crate::silo::game::GameManifestStability) -> Self {
        match value {
            crate::silo::game::GameManifestStability::Release => Self::Release,
            crate::silo::game::GameManifestStability::Snapshot => Self::Snapshot,
            crate::silo::game::GameManifestStability::OldBeta => Self::OldBeta,
            crate::silo::game::GameManifestStability::OldAlpha => Self::OldAlpha,
        }
    }
}

#[cfg(feature = "silo")]
impl From<crate::silo::game::ApiGameVersionLegacy> for GameVersion {
    fn from(value: crate::silo::game::ApiGameVersionLegacy) -> Self {
        let java_version = value.java_version.map(|it| it.major_version).unwrap_or(8);
        Self {
            id: value.id,
            release_date: value.release_time,
            stability: GameVersionStability::from(value.stability),
            java_version: VersionReq {
                comparators: vec![Comparator {
                    op: if java_version == 8 { Op::Exact } else { Op::GreaterEq },
                    major: java_version,
                    minor: None,
                    patch: None,
                    pre: Prerelease::EMPTY,
                }],
            },
            main_class: value.main_class,
            libraries: value.libraries.into_iter().flat_map(Vec::<MaybeConditional<Library>>::from).collect(),
            java_arguments: vec![
                MaybeConditional::Conditional {
                    when: Condition::OS(OS::MacOS),
                    then: "-XstartOnFirstThread".to_string(),
                },
                MaybeConditional::Conditional {
                    when: Condition::OS(OS::Windows),
                    then: "-XX:HeapDumpPath=MojangTricksIntelDriversForPerformance_javaw.exe_minecraft.exe.heapdump".to_string(),
                },
                MaybeConditional::Conditional {
                    when: Condition::OS(OS::Windows),
                    then: "-Dos.name=Windows 10".to_string(),
                },
                MaybeConditional::Conditional {
                    when: Condition::OS(OS::Windows),
                    then: "-Dos.version=10.0".to_string(),
                },
                MaybeConditional::Unconditional("-Djava.library.path=${natives_directory}".to_string()),
                MaybeConditional::Unconditional("-Dminecraft.launcher.brand=${launcher_name}".to_string()),
                MaybeConditional::Unconditional("-Dminecraft.launcher.version=${launcher_version}".to_string()),
                MaybeConditional::Unconditional("-cp".to_string()),
                MaybeConditional::Unconditional("${classpath}".to_string()),
            ],
            game_arguments: value
                .minecraft_arguments
                .split(' ')
                .map(|it| MaybeConditional::Unconditional(it.to_string()))
                .collect(),
        }
    }
}

#[cfg(feature = "silo")]
impl From<crate::silo::game::ApiGameVersion17w43a> for GameVersion {
    fn from(value: crate::silo::game::ApiGameVersion17w43a) -> Self {
        Self {
            id: value.id,
            release_date: value.release_time,
            stability: GameVersionStability::from(value.stability),
            java_version: VersionReq {
                comparators: vec![Comparator {
                    op: if value.java_version.major_version == 8 { Op::Exact } else { Op::GreaterEq },
                    major: value.java_version.major_version,
                    minor: None,
                    patch: None,
                    pre: Prerelease::EMPTY,
                }],
            },
            main_class: value.main_class,
            libraries: value.libraries.into_iter().flat_map(Vec::<MaybeConditional<Library>>::from).collect(),
            java_arguments: value
                .arguments
                .jvm
                .into_iter()
                .flat_map(Vec::<MaybeConditional<String>>::from)
                .collect(),
            game_arguments: value
                .arguments
                .game
                .into_iter()
                .flat_map(Vec::<MaybeConditional<String>>::from)
                .collect(),
        }
    }
}

#[cfg(feature = "silo")]
impl From<crate::silo::game::ApiGameVersion> for GameVersion {
    fn from(value: crate::silo::game::ApiGameVersion) -> Self {
        match value {
            crate::silo::game::ApiGameVersion::Legacy(val) => Self::from(val),
            crate::silo::game::ApiGameVersion::Modern(val) => Self::from(val),
        }
    }
}
