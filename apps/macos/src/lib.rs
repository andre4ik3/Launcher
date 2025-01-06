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

use reqwest::Client;

use launcher::models::JavaInfo;
use launcher::net::auth::AuthenticationService;
use launcher::net::download::java::download;
use launcher::net::meta::get_java;
use launcher::repo::{Repo, JAVA};
use launcher::store::{ConfigHolder, CredentialsHolder, StoreHolder, CONFIG, CREDENTIALS};

use crate::ffi::FFIJava;

// Rust extension freaks out here, "extern types is experimental"
// Use VSC w/ rust-analyzer for good experience
#[swift_bridge::bridge]
mod ffi {
    #[swift_bridge(swift_repr = "struct")]
    pub(crate) struct FFIJava {
        pub id: String,
        pub provider: String,
        pub major: u64,
        pub version: String,
    }

    extern "Rust" {
        type LauncherBridge;

        #[swift_bridge(init)]
        fn new() -> LauncherBridge;
        async fn setup(&mut self);

        fn get_login_url(&self) -> String;
        async fn do_login(&self, code: String);

        async fn get_accounts(&mut self);

        // Java
        async fn java_len(&self) -> usize;
        async fn java_at(&self, index: usize) -> FFIJava;
        async fn java_install(&self, version: u8);
        async fn java_update(&self, id: String);
        async fn java_uninstall(&self, id: String);
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

    /* ========================================================================================== */
    /* Java                                                                                       */
    /* ========================================================================================== */

    /// Gets the total amount of Java installations. A hack because `swift-bridge` can't handle
    /// vectors of strings (or structs that have them).
    async fn java_len(&self) -> usize {
        JAVA.get()
            .await
            .list()
            .await
            .map(|list| list.len())
            .unwrap_or(0)
    }

    /// Gets a Java installation at an index from `java_len()`. A hack because `swift-bridge` can't
    /// handle vectors of strings (or structs that have them).
    async fn java_at(&self, index: usize) -> FFIJava {
        JAVA.get()
            .await
            .list()
            .await
            .ok()
            .and_then(|java| {
                let mut values: Vec<(String, JavaInfo)> = java.into_iter().collect();
                values.sort();
                values.into_iter().nth(index)
            })
            .map(|(id, info)| FFIJava::from_info(id, info))
            .expect("Invalid index (issue on Swift's side)")
    }

    /// Installs a major Java version from the UI.
    async fn java_install(&self, version: u8) {
        // TODO: Error handling
        let build = get_java(&self.client, version).await.unwrap();
        let archive = download(&self.client, build).await.unwrap();
        JAVA.get().await.add(archive).await.unwrap();
    }

    /// Updates a Java build from the UI.
    async fn java_update(&self, id: String) {
        todo!()
    }

    /// Uninstalls a Java build from the UI.
    async fn java_uninstall(&self, id: String) {
        // TODO: Error handling
        JAVA.get().await.delete(id).await.expect("Gone wrong")
    }
}

impl FFIJava {
    fn from_info(id: String, info: JavaInfo) -> Self {
        Self {
            id,
            provider: info.provider.to_string(),
            major: info.version.major,
            version: info.version.to_string(),
        }
    }
}
