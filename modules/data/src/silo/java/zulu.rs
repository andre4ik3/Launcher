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

use url::Url;

use macros::api_response;

// A note about `strict = false` -- with Java we don't necessarily care about every single property,
// as we do with important metadata such as game versions (where we want to fail fast and early in
// case something is different from what we know of -- so all modifications are tested and accounted
// for).

/// Represents a build returned by Zulu Metadata API search function.
#[api_response(strict = false)]
pub struct ZuluMetadata {
    pub package_uuid: String,
    pub latest: bool,
    pub download_url: Url,
    pub java_version: (u64, u64, u64),
}

/// Represents additional information about a build returned by Zulu Metadata API details function.
#[api_response(strict = false)]
pub struct ZuluDetails {
    pub name: String,
    pub sha256_hash: String,
    pub size: u64,
}
