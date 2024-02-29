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
mod macros;

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
    let root = fs::canonicalize(&args.output).await?;

    info!("Output directory: {}", root.display());

    // === Game Versions ===
    if args.task.contains(&cli::TaskName::GameVersions) {
        info!("Running Game Versions task...");
        let versions = TaskGameVersions::run(&root, ()).await?;
        info!("Game Versions task complete. Successfully retrieved {} versions.", versions.len());
    }

    // === Java ===
    if args.task.contains(&cli::TaskName::Java) {
        info!("Running Java task...");
        let builds = TaskJava::run(&root, vec![8, 16, 17]).await?;
        info!("Java task complete. Successfully retrieved {} builds.", builds.len());
    }

    client().await.destroy().await;
    Ok(())
}
