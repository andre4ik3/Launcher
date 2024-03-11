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

use anyhow::anyhow;

use data::core::game::GameVersionSnippet;
use data::core::loader::{ModLoader, ModLoaderVersion};
use data::silo::loader::fabric::{FabricVersion, FabricVersionLoader, FabricVersionMainClass};
use net::Client;

use crate::{client, vpath};
use crate::macros::write_to_ron_file;

const FABRIC_BASE_URL: &str = "https://meta.fabricmc.net/v2/versions/loader";

pub const LOADERS: [ModLoader; 4] = [
    ModLoader {
        id: "forge",
        display_name: "Forge",
    },
    ModLoader {
        id: "fabric",
        display_name: "Fabric",
    },
    ModLoader {
        id: "quilt",
        display_name: "Quilt",
    },
    ModLoader {
        id: "neoforge",
        display_name: "NeoForge",
    },
];

async fn run_forge(client: &Client, game_versions: &Vec<GameVersionSnippet>) -> anyhow::Result<Vec<ModLoaderVersion>> {
    let mut output = Vec::new();

    for version in game_versions {}

    Ok(output)
}

async fn run_fabric(client: &Client, game_versions: &Vec<GameVersionSnippet>) -> anyhow::Result<Vec<ModLoaderVersion>> {
    let mut output = Vec::new();

    let loaders: Vec<FabricVersionLoader> = client.get(FABRIC_BASE_URL).await?.json().await?;
    let loader = loaders.into_iter().find(|it| it.stable);
    let loader = loader.ok_or(anyhow!("no fabric loader version"))?;

    for game_version in game_versions {
        let version: FabricVersion = client.get(format!("{FABRIC_BASE_URL}/{}/{}", game_version.id, loader.version)).await?.json().await?;
        output.push(ModLoaderVersion {
            loader_version: version.loader.version.to_string(),
            game_version: game_version.id.clone(),
            libraries: vec![],
            main_class: match version.launcher_meta.main_class {
                FabricVersionMainClass::Constant(val) => val,
                FabricVersionMainClass::Variable { client, .. } => client,
            },
            game_arguments: vec![],
            java_arguments: vec![],
        })
    }

    Ok(output)
}

async fn run_quilt(client: &Client, game_versions: &Vec<GameVersionSnippet>) -> anyhow::Result<Vec<ModLoaderVersion>> {
    let mut output = Vec::new();

    for version in game_versions {}

    Ok(output)
}

async fn run_neoforge(client: &Client, game_versions: &Vec<GameVersionSnippet>) -> anyhow::Result<Vec<ModLoaderVersion>> {
    let mut output = Vec::new();

    for version in game_versions {}

    Ok(output)
}

pub async fn run(game_versions: Vec<GameVersionSnippet>) -> anyhow::Result<HashMap<String, Vec<ModLoaderVersion>>> {
    let client = client();
    let versions: Vec<ModLoaderVersion> = vec![
        // run_forge(client, &game_versions).await?,
        run_fabric(client, &game_versions).await?,
        // run_quilt(client, &game_versions).await?,
        // run_neoforge(client, &game_versions).await?,
    ].into_iter().flatten().collect();

    // Group each mod loader version by game version.
    let mut output = HashMap::<String, Vec<ModLoaderVersion>>::new();
    for version in versions {
        output.entry(version.game_version.clone())
            .or_default()
            .push(version);
    }

    // Write each group to the corresponding file.
    for (version, data) in &output {
        write_to_ron_file(vpath!("loaders/{version}.ron"), &data).await?;
    }

    Ok(output)
}
