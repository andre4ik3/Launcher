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

use reqwest::Client;

use launcher::net::java::download_java;
use launcher::net::meta::{get_game_index, get_game_version, get_index, get_java};
use launcher::store::config::CONFIG;
use launcher::utils::get_dirs;

#[tokio::main]
async fn main() {
    let path = get_dirs().config_dir().join("Config.toml");
    println!("Config: {:?}", path);

    let config = CONFIG.get().await;
    let data = config.get().await;
    println!("Config: {:?}", data);

    let client = Client::default();

    let data = get_index(&client).await.unwrap();
    println!("{:?}", data);

    let data = get_game_index(&client).await.unwrap();
    println!("{:?}", data);

    let data = get_game_version(&client, "1.20").await.unwrap();
    println!("{:?}", data);

    let data = get_java(&client, 8).await.unwrap();
    println!("{:?}", data);

    download_java(&client, data).await.unwrap();
}
