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

use std::collections::HashMap;

use anyhow::Result;
use indicatif::ProgressBar;

use crate::CLIENT;

const VERSIONS: &str =
    "https://files.minecraftforge.net/net/minecraftforge/forge/maven-metadata.json";
const PROMOTIONS: &str =
    "https://files.minecraftforge.net/net/minecraftforge/forge/promotions_slim.json";

pub async fn collect() -> Result<()> {
    let data = CLIENT.get(VERSIONS).send().await?;
    let data: HashMap<String, Vec<String>> = data.error_for_status()?.json().await?;
    println!("{:?}", data.get("1.20.1"));

    Ok(())
}
