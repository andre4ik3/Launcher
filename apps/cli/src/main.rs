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

use launcher::{utils, net, fetch};
use url::Url;

const BASE_URL: &str = "https://launchermeta.lambda.prod.andre4ik3.net/";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // just a playground to test out WIP code
    let _guard = utils::log::setup();
    let client = net::Client::new().await;

    let url = Url::parse(BASE_URL)?;

    let index = fetch::meta::index(&client, &url).await?;

    println!("{:#?}", index);

    Ok(())
}
