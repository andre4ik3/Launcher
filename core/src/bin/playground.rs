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

use std::time::Duration;

use anyhow::Result;
use reqwest::{Method, Request};
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    let _guard = utils::log::setup();
    info!("Hello, world!");

    let (cancel_tx, cancel_rx) = tokio::sync::oneshot::channel();

    let tx = net::spawn_queue(cancel_rx).await;
    let client = net::NetQueueClient::new(tx);

    let req = Request::new(Method::GET, "https://ipv4.icanhazip.com/".parse().unwrap());
    info!("Very epic request that we're gonna try execute: {req:?}");

    let resp = client.execute(req).await?;
    info!("Got a very epic response: {resp:?}");

    let ip = resp.text().await?;
    info!("Your ip address is {ip}");

    info!("Trying to shutdown queue by dropping client...");
    drop(client);

    // info!("Trying to shutdown queue by sending across oneshot channel...");
    // cancel_tx.send(()).unwrap();

    info!("Sleeping a bit...");
    tokio::time::sleep(Duration::from_secs(3)).await;

    Ok(())
}
