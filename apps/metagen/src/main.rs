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

use anyhow::Result;
use once_cell::sync::Lazy;
use reqwest::Client;
use std::time::Duration;

mod models;
mod tasks;
mod utils;

/// Client for HTTP requests.
pub static CLIENT: Lazy<Client> = Lazy::new(|| {
    Client::builder()
        .brotli(true)
        .timeout(Duration::from_secs(30))
        .user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/114.0.0.0 Safari/537.36")
        .build()
        .expect("Failed to build client")
});

#[tokio::main]
async fn main() -> Result<()> {
    // Run a task to collect game metadata first. Then get Java versions based on what the different
    // game versions require.
    tasks::loaders::run().await?;
    // let (java_versions, assets) = tasks::game::run().await?;

    // tasks::java::run(java_versions).await?;
    // tasks::assets::run(assets).await?;
    // tasks::index::run().await?;

    println!("Done.");
    Ok(())
}
