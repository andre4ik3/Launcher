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

//! [legacy] - Types for versions before 17w43a (first snapshot of 1.13).
//! [v17w43a] - Types for versions after 17w43a (first snapshot of 1.13).

pub use legacy::*;
pub use v17w43a::*;

mod legacy;
mod v17w43a;

#[macros::api_response]
pub enum ApiGameVersion {
    Legacy(ApiGameVersionLegacy),
    Modern(ApiGameVersion17w43a),
}
