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

use crate::models::game::v17w43a::GameManifest17w43a;
use crate::tasks::game;
use once_cell::sync::Lazy;
use reqwest::Client;

mod models;
mod tasks;

/// Client for HTTP requests.
pub static CLIENT: Lazy<Client> = Lazy::new(Client::new);

const STRING: &str = r#"{
            "downloads": {
                "artifact": {
                    "path": "org/lwjgl/lwjgl/3.1.2/lwjgl-3.1.2.jar",
                    "sha1": "28a4511b5bc6624dbc6c579ade1b25bc2b21733e",
                    "size": 271541,
                    "url": "https://libraries.minecraft.net/org/lwjgl/lwjgl/3.1.2/lwjgl-3.1.2.jar"
                },
                "classifiers": {
                    "natives-linux": {
                        "path": "org/lwjgl/lwjgl/3.1.2/lwjgl-3.1.2-natives-linux.jar",
                        "sha1": "eef24025434e3c7d735744987e9330d67d06bb7f",
                        "size": 75964,
                        "url": "https://libraries.minecraft.net/org/lwjgl/lwjgl/3.1.2/lwjgl-3.1.2-natives-linux.jar"
                    },
                    "natives-macos": {
                        "path": "org/lwjgl/lwjgl/3.1.2/lwjgl-3.1.2-natives-macos.jar",
                        "sha1": "5f3cd6a9e04938a943442be68dbcdb0e9dcec486",
                        "size": 34785,
                        "url": "https://libraries.minecraft.net/org/lwjgl/lwjgl/3.1.2/lwjgl-3.1.2-natives-macos.jar"
                    },
                    "natives-windows": {
                        "path": "org/lwjgl/lwjgl/3.1.2/lwjgl-3.1.2-natives-windows.jar",
                        "sha1": "0af9c28e4cc38f58ef34131ca3d300b985b2e265",
                        "size": 229565,
                        "url": "https://libraries.minecraft.net/org/lwjgl/lwjgl/3.1.2/lwjgl-3.1.2-natives-windows.jar"
                    },
                    "sources": {
                        "path": "org/lwjgl/lwjgl/3.1.2/lwjgl-3.1.2-sources.jar",
                        "sha1": "dcbb8a93025d205ac73614404159dc48e5ce9657",
                        "size": 225533,
                        "url": "https://libraries.minecraft.net/org/lwjgl/lwjgl/3.1.2/lwjgl-3.1.2-sources.jar"
                    }
                }
            },
            "name": "org.lwjgl:lwjgl:3.1.2",
            "natives": {
                "linux": "natives-linux",
                "osx": "natives-macos",
                "windows": "natives-windows"
            }
        }"#;

const OLD_VERSION: &str = include_str!("../../../scratch.json");

#[tokio::main]
async fn main() {
    // let data: GameManifest17w43a = serde_json::from_str(OLD_VERSION).unwrap();
    // let data: NativeLibrary = serde_json::from_str(STRING).unwrap();
    // println!("{:?}", data);
    game::run().await.unwrap();
}
