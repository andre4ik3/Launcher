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

use crate::models::{GameLibrary, GameMaybeConditional};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Supported mod loaders.
#[derive(Debug, Deserialize, Serialize)]
pub enum Loader {
    Forge,
    Fabric,
}

/// A loader is an overlay that is applied on top of the base game version. It adds libraries and
/// changes the entrypoint class to hook into the base game.
#[derive(Debug, Deserialize, Serialize)]
pub struct LoaderVersion {
    /// The loader type.
    pub loader: Loader,
    /// The unique mod loader version. Note: doesn't follow SemVer.
    pub version: String,
    /// The game version that this loader version is for. Note: doesn't follow SemVer.
    pub game_version: String,
    /// Whether this loader version is the recommended (stable) one for this game version.
    pub recommended: bool,
    /// The libraries that are added on top of the base game libraries.
    pub libraries: Box<[GameMaybeConditional<GameLibrary>]>,
    /// The Java class that serves as the entry point for this mod loader.
    pub entrypoint: String,
}

/// Surface-level information about a [LoaderVersion].
#[derive(Debug, Deserialize, Serialize)]
pub struct LoaderVersionSnippet {
    /// The unique mod loader version. Note: doesn't follow SemVer.
    pub version: String,
    /// Whether this loader version is the recommended (stable) one for this game version.
    pub recommended: bool,
}

/// Information about available loader versions.
#[derive(Debug, Deserialize, Serialize)]
pub struct LoaderVersionIndex {
    /// A map of game versions to Forge loader versions available.
    pub forge: HashMap<String, Box<[LoaderVersionSnippet]>>,
    /// A map of game versions to Fabric loader versions available.
    pub fabric: HashMap<String, Box<[LoaderVersionSnippet]>>,
}

