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
use std::process::Command;

const PACKAGE_NAME: &str = env!("CARGO_PKG_NAME");
const PACKAGE_VERSION: &str = env!("CARGO_PKG_VERSION");

// Rust extension freaks out here, "extern types is experimental"
// Use VSC w/ rust-analyzer for good experience
#[swift_bridge::bridge]
mod ffi {
    extern "Rust" {
        type RustApp;

        #[swift_bridge(init)]
        fn new() -> RustApp;

        fn say_hello(&self, who: &str) -> String;
        async fn do_something(&self);
    }
}

pub struct RustApp;

impl RustApp {
    fn new() -> Self {
        RustApp {}
    }

    fn say_hello(&self, who: &str) -> String {
        format!(
            "Hello, {}! This is {} version {}.",
            who, PACKAGE_NAME, PACKAGE_VERSION
        )
    }

    async fn do_something(&self) {
        println!("This is rust!");
        let client = Client::new();
        let build = launcher::net::meta::get_java(&client, 17).await.unwrap();
        launcher::net::java::install(&client, build).await.unwrap();
        println!("End of rust!");
    }
}
