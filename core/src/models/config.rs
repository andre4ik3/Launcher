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

use serde::{Deserialize, Serialize};
use url::Url;

/// Default URL for the metadata server.
const METADATA_SERVER: &str = "https://launchermeta.pages.dev/v1";

/// Configuration for the launcher. Saved as `Config.toml` in platform-appropriate directory.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    /// The URL where game and Java metadata will be fetched from.
    pub metadata_server: Url,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            metadata_server: Url::parse(METADATA_SERVER).unwrap(),
        }
    }
}
