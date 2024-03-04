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

//! [assets] - Types for asset indeces.
//! [version] - Types for the files that `version_manifest_v2.json` links to.
//! [version_manifest] - Types for `version_manifest_v2.json` file.

pub use assets::*;
pub use version::*;
pub use version_manifest::*;

mod version;
mod version_manifest;
mod assets;
