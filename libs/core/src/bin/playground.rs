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
use reqwest::Client;

use launcher::net::download::java::download;
use launcher::net::meta::get_java;
use launcher::repo::{Repo, JAVA};

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::new();
    let repo = JAVA.get().await;

    let build = get_java(&client, 8).await?;
    println!("{:#?}", build);

    let archive = download(&client, build).await?;
    println!("{:#?}", archive.metadata);

    repo.add(archive).await?;
    println!("i think it worked?");

    println!("{:#?}", repo.list().await?);
    Ok(())
}
