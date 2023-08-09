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

#[cfg(feature = "silo")]
impl From<crate::silo::game::GameVersionManifestStability> for GameVersionStability {
    fn from(value: crate::silo::game::GameVersionManifestStability) -> Self {
        match value {
            crate::silo::game::GameVersionManifestStability::Release => Self::Release,
            crate::silo::game::GameVersionManifestStability::Snapshot => Self::Snapshot,
            crate::silo::game::GameVersionManifestStability::OldBeta => Self::OldBeta,
            crate::silo::game::GameVersionManifestStability::OldAlpha => Self::OldAlpha,
        }
    }
}

/// TODO
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GameVersion {}
