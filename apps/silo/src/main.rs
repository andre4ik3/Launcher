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

//! Launcher Silo
//! =============
//!
//! The Silo program generates dynamic data about Java builds, game versions, asset catalogs, and
//! mod loaders. This dynamic data is fetched, converted to a universal format (see [data::core]),
//! and saved in a directory (`_site` by default). The directory is typically then uploaded to a
//! web server that clients can get data from using the [net::meta] module.
//!
//! Each main function of Silo is represented as a [task::Task]. Tasks can depend on one another and
//! are run in parallel when possible.

use std::path::PathBuf;
use anyhow::anyhow;
use tokio::fs;
use tokio::sync::OnceCell;

use net::Client;

mod cli;
mod task;
pub(crate) mod macros;

pub(crate) use data::web::meta::VERSION;

const PACKAGE_VERSION: &str = env!("CARGO_PKG_VERSION");
const META_VERSIONS: [u64; 1] = [0];

static CLIENT: OnceCell<Client> = OnceCell::const_new();
static ROOT: OnceCell<PathBuf> = OnceCell::const_new();

pub(crate) fn root<'a>() -> &'a PathBuf {
    ROOT.get().unwrap()
}

pub(crate) fn client<'a>() -> &'a Client {
    CLIENT.get().unwrap()
}


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = cli::parse();
    let _guard = utils::log::setup();

    tracing::info!("Running Silo v{PACKAGE_VERSION}");
    fs::create_dir_all(&args.output).await.expect("failed to create output folder");

    // Initialize statics
    CLIENT.set(Client::new().await).map_err(|_| anyhow!("failed to setup client")).unwrap();
    ROOT.set(fs::canonicalize(&args.output).await?).unwrap();

    // Index
    let index = task::index::run(vec![]).await?;
    tracing::info!("Successfully generated index file with {} announcements.", index.announcements.len());

    // Java builds
    // let java_builds = task::java::run(vec![8, 17]).await?;
    // tracing::info!("Successfully fetched {} Java builds.", java_builds.len());

    // Game versions
    let game_versions = task::game::run(args.power_wash).await?;
    tracing::info!("Successfully fetched {} game versions.", game_versions.len());
    
    // let loader_versions = task::loaders::run(game_versions).await?;
    // tracing::info!("Successfully loaded mod loaders.");

    CLIENT.get().unwrap().destroy().await;
    Ok(())
}
