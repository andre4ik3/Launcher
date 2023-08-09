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

use tokio::fs;
use tokio::sync::OnceCell;
use tracing::info;

use net::Client;
use task::*;

mod cli;
mod task;

const PACKAGE_VERSION: &str = env!("CARGO_PKG_VERSION");
static CLIENT: OnceCell<Client> = OnceCell::const_new();

pub(crate) async fn client<'a>() -> &'a Client {
    CLIENT.get_or_init(Client::new).await
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = cli::parse();
    let _guard = utils::log::setup();

    info!("Running Silo v{PACKAGE_VERSION}");
    info!("{:?}", args.task);

    fs::create_dir_all(&args.output).await?;
    let output = fs::canonicalize(&args.output).await?;

    info!("Output directory: {}", output.display());

    // === testing ===

    let game_versions = TaskGameVersions::run(()).await?;
    fs::write(
        output.join("game_versions.ron"),
        format!("{game_versions:#?}"),
    )
    .await?;

    client().await.destroy().await;
    Ok(())
}
