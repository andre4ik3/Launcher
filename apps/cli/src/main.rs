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

use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader};
use auth::{AuthenticationService, MicrosoftAuthenticationService};
use data::web::microsoft::AUTH_URL;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // just a playground to test out WIP code
    let _guard = utils::log::setup();
    let client = net::Client::new().await;
    
    // let thing = 

    // === Ask the user for the token ===
    println!("Please open the following URL in the browser, sign in, then copy the code from the redirect URL: {AUTH_URL}");
    println!("Paste your token: ");
    let mut token = String::new();
    io::stdout().flush().await?;
    BufReader::new(io::stdin()).read_line(&mut token).await?;
    let token = token.trim();

    // === Try authenticate ===
    let account = MicrosoftAuthenticationService::authenticate(&client, token.to_string()).await?;
    
    println!("Got account!!");
    println!("{:#?}", account);

    Ok(())
}
