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

use macros::data_structure;

use crate::core::conditional::MaybeConditional;
use crate::core::library::Library;

/// Represents a particular mod loader (e.g. Forge or Fabric).
#[data_structure]
pub struct ModLoader {
    /// Unique ID of this mod loader.
    pub id: &'static str,
    /// User-visible display name of this mod loader.
    pub display_name: &'static str,
}

/// Represents a version of a mod loader, and the parameters needed to load it.
#[data_structure]
pub struct ModLoaderVersion {
    /// The ID of this particular loader version.
    pub loader_version: String,
    /// The game version that this loader version is targeting.
    pub game_version: String,
    /// Extra libraries that will be loaded alongside the game's libraries.
    pub libraries: Vec<MaybeConditional<Library>>,
    /// The main class that will be loaded instead of the game's main class.
    pub main_class: String,
    /// A list of arguments that will be appended to the game's main arguments.
    pub game_arguments: Vec<MaybeConditional<String>>,
    /// A list of arguments that will be appended to the game's JVM arguments.
    pub java_arguments: Vec<MaybeConditional<String>>,
}
