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

use anyhow::Result;
use async_once_cell::OnceCell;
use tracing::info;

use net::Client;

static CLIENT: OnceCell<Client> = OnceCell::new();

#[tokio::main]
async fn main() -> Result<()> {
    let _guard = utils::log::setup();
    info!("Hello, world!");

    let client = CLIENT.get_or_init(Client::new()).await;

    let mut bytes: Vec<u8> = Vec::new();
    client
        .download("https://httpbin.org/drip", &mut bytes)
        .await?;

    info!("Final length: {}", bytes.len());

    info!("RAW data: {:?}", bytes);
    let string = String::from_utf8(bytes)?;
    info!("String?: {string}");

    info!("Joining client...");
    client.destroy().await;

    Ok(())
}
