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

use std::str::FromStr;

use serde::{Deserialize, Serialize};
use url::Url;

/// Version 1 of the launcher configuration format.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct ConfigV1 {
    /// The URL where game and Java metadata will be fetched from.
    pub metadata_server: Url,
}

impl Default for ConfigV1 {
    fn default() -> Self {
        ConfigV1 {
            metadata_server: Url::from_str("https://master.launchermeta.pages.dev").unwrap(),
        }
    }
}
