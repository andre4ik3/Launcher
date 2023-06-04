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

use crate::models::java::AdoptiumBuild;
use crate::{tasks::java::Provider, CLIENT};
use anyhow::{bail, Result};
use async_trait::async_trait;
use launcher::models::java::JavaBuild;
use launcher::models::Environment;
use platforms::{Arch, OS};
use url::Url;

const ADOPTIUM_BASE: &str = "https://api.adoptium.net/v3/assets/latest";

/// Maps Platform enums to strings that the Adoptium API accepts.
fn map(os: OS, arch: Arch) -> (&'static str, &'static str) {
    (
        match os {
            OS::MacOS => "mac",
            OS::Linux | OS::Windows => os.as_str(),
            _ => unimplemented!(),
        },
        match arch {
            Arch::AArch64 => arch.as_str(),
            Arch::X86_64 => "x64",
            _ => unimplemented!(),
        },
    )
}

pub struct Adoptium;

#[async_trait]
impl Provider for Adoptium {
    async fn fetch(version: u8, env: Environment) -> Result<JavaBuild> {
        let url = &format!("{}/{}/hotspot", ADOPTIUM_BASE, version);
        let (os, arch) = map(env.os, env.arch);
        let params = &[
            ("architecture", arch),
            ("os", os),
            ("image_type", "jre"),
            ("vendor", "eclipse"),
        ];

        let url = Url::parse_with_params(url, params)?;
        let resp = CLIENT.get(url).send().await?.error_for_status()?;
        let build: Vec<AdoptiumBuild> = resp.json().await?;

        let mut build: Vec<JavaBuild> = build
            .into_iter()
            .map(<AdoptiumBuild as Into<JavaBuild>>::into)
            .filter(|b| b.environment.env == env.env)
            .collect();

        if build.is_empty() {
            bail!("Build not found");
        }

        // This is the one part of Rust I don't like
        // https://stackoverflow.com/questions/27904864/what-does-cannot-move-out-of-index-of-mean
        Ok(build.swap_remove(0))
    }
}
