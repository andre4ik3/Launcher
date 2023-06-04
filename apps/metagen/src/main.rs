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

use crate::models::game::legacy::GameManifestLegacy;
use crate::tasks::game;
use once_cell::sync::Lazy;
use reqwest::Client;

mod models;
mod tasks;

/// Client for HTTP requests.
pub static CLIENT: Lazy<Client> = Lazy::new(Client::new);

const OLD_VERSION: &str = include_str!("../../../scratch.json");

#[tokio::main]
async fn main() {
    // let data: GameManifestLegacy = serde_json::from_str(OLD_VERSION).unwrap();
    // println!("{:?}", data);
    game::run().await.unwrap();
}
