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

use launcher::net::auth::AuthenticationService;
use launcher::store::config::{ConfigHolder, CONFIG};
use launcher::store::credentials::{CredentialsHolder, CREDENTIALS};
use launcher::store::StoreHolder;
use reqwest::Client;

const PACKAGE_NAME: &str = env!("CARGO_PKG_NAME");
const PACKAGE_VERSION: &str = env!("CARGO_PKG_VERSION");

// Rust extension freaks out here, "extern types is experimental"
// Use VSC w/ rust-analyzer for good experience
#[swift_bridge::bridge]
mod ffi {
    extern "Rust" {
        type LauncherBridge;

        #[swift_bridge(init)]
        fn new() -> LauncherBridge;
        async fn setup(&mut self);

        fn get_login_url(&self) -> String;
        async fn do_login(&self, code: String);

        async fn get_accounts(&mut self);
    }
}

pub struct LauncherBridge {
    client: Client,
    config: Option<&'static ConfigHolder>,
    credentials: Option<&'static CredentialsHolder>,
}

impl LauncherBridge {
    fn new() -> Self {
        LauncherBridge {
            client: Client::new(),
            config: None,
            credentials: None,
        }
    }

    async fn setup(&mut self) {
        self.config = Some(CONFIG.get().await);
        self.credentials = Some(CREDENTIALS.get().await);
    }

    fn get_login_url(&self) -> String {
        let url = launcher::net::auth::get_auth_url();
        url.to_string()
    }

    async fn do_login(&self, code: String) {
        let account =
            launcher::net::auth::MicrosoftAuthenticationService::authenticate(&self.client, code)
                .await
                .unwrap();

        println!("{:?}", account);
    }

    async fn get_accounts(&self) {
        let credentials = self.credentials.unwrap().get().await;
        println!("{:?}", credentials);
    }
}
