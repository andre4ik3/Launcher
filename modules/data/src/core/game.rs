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

use semver::{Comparator, Op, Prerelease, VersionReq};
use serde::{Deserialize, Serialize};

use super::conditional::MaybeConditional;

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
/// version, Java version, etc).
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GameVersion {
    /// The ID or "name" of this version (e.g. "1.12.2" or "23w32a").
    pub id: String,
    /// The stability of this version (e.g. release vs snapshot).
    pub stability: GameVersionStability,
    /// The version of Java required to launch this version.
    pub java_version: VersionReq,
    /// The main Java class that contains the game's `main()` function.
    pub main_class: String,
    /// A list of arguments that should be passed before `main_class`. The arguments can contain
    /// variables enclosed in `${}`, which should be replaced.
    pub java_arguments: Vec<MaybeConditional<String>>,
    /// A list of arguments that should be passed after `main_class`. The arguments can contain
    /// variables enclosed in `${}`, which should be replaced.
    pub game_arguments: Vec<MaybeConditional<String>>,
    // pub libraries: Vec<MaybeConditional<Library>>
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
impl From<crate::silo::game::GameVersionLegacy> for GameVersion {
    fn from(value: crate::silo::game::GameVersionLegacy) -> Self {
        Self {
            id: value.id,
            stability: GameVersionStability::from(value.stability),
            java_version: VersionReq {
                comparators: vec![Comparator {
                    op: Op::Exact,
                    major: value.java_version.map(|it| it.major_version).unwrap_or(8),
                    minor: None,
                    patch: None,
                    pre: Prerelease::EMPTY,
                }],
            },
            main_class: value.main_class,
            java_arguments: vec![],
            game_arguments: vec![MaybeConditional::Unconditional(
                value
                    .minecraft_arguments
                    .split(' ')
                    .map(|it| it.to_string())
                    .collect(),
            )],
        }
    }
}

#[cfg(feature = "silo")]
impl From<crate::silo::game::GameVersion17w43a> for GameVersion {
    fn from(value: crate::silo::game::GameVersion17w43a) -> Self {
        Self {
            id: value.id,
            stability: GameVersionStability::from(value.stability),
            java_version: VersionReq {
                comparators: vec![Comparator {
                    op: Op::Exact,
                    major: value.java_version.major_version,
                    minor: None,
                    patch: None,
                    pre: Prerelease::EMPTY,
                }],
            },
            main_class: value.main_class,
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
impl From<crate::silo::game::GameVersion> for GameVersion {
    fn from(value: crate::silo::game::GameVersion) -> Self {
        match value {
            crate::silo::game::GameVersion::Legacy(val) => Self::from(val),
            crate::silo::game::GameVersion::Modern(val) => Self::from(val),
        }
    }
}
