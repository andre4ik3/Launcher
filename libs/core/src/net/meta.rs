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

use std::fmt::Display;

use anyhow::Result;
use reqwest::Client;
use serde::Deserialize;

use crate::models::game::{GameVersion, GameVersionIndex};
use crate::models::java::JavaBuild;
use crate::models::{Environment, MetadataIndex};
use crate::store::config::CONFIG;

/// Helper function to run a request to a URL and decode the result.
async fn get<T: for<'a> Deserialize<'a>>(client: &Client, path: impl Display) -> Result<T> {
    let base = CONFIG.get().await.get().await.metadata_server;
    let resp = client.get(format!("{base}/{path}")).send().await?;
    let resp = resp.error_for_status()?.text().await?;
    Ok(ron::from_str(&resp)?)
}

/// Gets the metadata index from the metadata server.
pub async fn get_index(client: &Client) -> Result<MetadataIndex> {
    get(client, "index.ron").await
}

/// Gets information about a Java version from the metadata server.
pub async fn get_java(client: &Client, version: u8) -> Result<JavaBuild> {
    let env = Environment::default();
    let url = format!("java/{}/{}/{version}.ron", env.arch, env.os);
    get(client, url).await
}

/// Gets the game version index from the metadata server.
pub async fn get_game_index(client: &Client) -> Result<GameVersionIndex> {
    get(client, "versions.ron").await
}

/// Gets a specific game version from the metadata server.
pub async fn get_game_version(client: &Client, version: &str) -> Result<GameVersion> {
    get(client, format!("versions/{version}.ron")).await
}
