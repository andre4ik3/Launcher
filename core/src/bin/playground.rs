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
use async_once_cell::OnceCell;
use reqwest::{Method, Request};
use tracing::info;

use net::Client;

static CLIENT: OnceCell<Client> = OnceCell::new();

#[tokio::main]
async fn main() -> Result<()> {
    let _guard = utils::log::setup();
    info!("Hello, world!");

    let client = CLIENT.get_or_init(Client::new()).await;

    let req = Request::new(Method::GET, "https://ipv4.icanhazip.com/".parse().unwrap());
    info!("Very epic request that we're gonna try execute: {req:?}");

    let resp = client.execute(req).await?;
    info!("Got a very epic response: {resp:?}");

    let ip = resp.text().await?;
    info!("Your ip address is {ip}");

    let _ = tokio::spawn(async move {
        let req = Request::new(Method::GET, "https://ipv4.icanhazip.com/".parse().unwrap());
        client.execute(req).await.expect("TODO: panic message")
    })
    .await;

    info!("Joining client...");
    client.destroy().await;

    info!("Trying a request now...");
    let req = Request::new(Method::GET, "https://ipv4.icanhazip.com/".parse().unwrap());
    info!("Very epic request that we're gonna try execute: {req:?}");

    let resp = client.execute(req).await?;
    info!("Got a very epic response: {resp:?}");

    Ok(())
}
