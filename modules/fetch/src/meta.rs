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

use url::Url;

use data::core::game::GameVersion;
use data::core::java::JavaBuild;
use data::web::meta::{MetaIndex, MetaIndexAnnouncement, VERSION};
use net::Client;
use utils::platforms::{CURRENT_ARCH, CURRENT_OS};

use super::Result;

macro_rules! fetch_impl {
    ($client:ident, $base:ident, $($arg:tt)*) => {{
        let url = $base.join(format!($($arg)*).as_str())?;
        let data = $client.get(url).await?.text().await.map_err(net::Error::from)?;
        Ok(ron::from_str(data.as_str())?)
    }};
}


pub struct MetaRepository<'a> {
    client: &'a Client,
    pub announcements: Vec<MetaIndexAnnouncement>,
}

impl<'a> MetaRepository<'a> {
    pub async fn new(client: &'a Client, base: &Url) -> Result<Self> {
        let index: Result<MetaIndex> = fetch_impl!(client, base, "index.ron");
        Ok(Self { client, announcements: index?.announcements })
    }
}

/// Fetches the index from the metadata server.
pub async fn index(client: &Client, base: &Url) -> Result<MetaIndex> {
    fetch_impl!(client, base, "index.ron")
}

/// Fetches a Java build from the metadata server.
pub async fn java_build(client: &Client, base: &Url, major_version: u64) -> Result<JavaBuild> {
    fetch_impl!(client, base, "v{VERSION}/java/{major_version}/{CURRENT_OS}-{CURRENT_ARCH}.ron")
}

/// Fetches a game version from the metadata server.
pub async fn game_version(client: &Client, base: &Url, id: &str) -> Result<GameVersion> {
    fetch_impl!(client, base, "v{VERSION}/game/{id}.ron")
}

/// Fetches a list of available game versions from the metadata server.
pub async fn game_versions(client: &Client, base: &Url) -> Result<Vec<GameVersion>> {
    fetch_impl!(client, base, "v{VERSION}/game.ron")
}
