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

use crate::models::java::{into_java_build, ZuluBuild, ZuluPackage};
use crate::tasks::java::Provider;
use crate::CLIENT;
use anyhow::{bail, Result};
use async_trait::async_trait;
use launcher::models::java::JavaBuild;
use launcher::models::Environment;
use platforms::{Arch, Env, OS};
use url::Url;

const ZULU_BASE: &str = "https://api.azul.com/metadata/v1/zulu/packages";

/// Maps Platform enums to strings that the Zulu API accepts.
fn map(env: Environment) -> (&'static str, &'static str, &'static str) {
    let clib = match env.env {
        Env::Musl => "linux-musl",
        _ => "linux-glibc",
    };
    (
        match env.os {
            OS::MacOS | OS::Windows => env.os.as_str(),
            OS::Linux => clib,
            _ => unimplemented!(),
        },
        match env.arch {
            Arch::AArch64 => "aarch64",
            Arch::X86_64 => "x64",
            _ => unimplemented!(),
        },
        match env.os {
            OS::MacOS | OS::Linux => "tar.gz",
            OS::Windows => "zip",
            _ => unimplemented!(),
        },
    )
}

pub struct Zulu;

#[async_trait]
impl Provider for Zulu {
    async fn fetch(version: u8, env: Environment) -> Result<JavaBuild> {
        let (os, arch, archive) = map(env);
        let params = &[
            ("java_package_type", "jre"),
            ("javafx_bundled", "false"),
            ("latest", "true"),
            ("release_status", "ga"),
            ("java_version", &version.to_string()),
            ("os", os),
            ("arch", arch),
            ("archive_type", archive),
        ];

        let url = Url::parse_with_params(ZULU_BASE, params)?;
        let resp = CLIENT.get(url).send().await?.error_for_status()?;
        let mut build: Vec<ZuluBuild> = resp.json().await?;

        if build.is_empty() {
            bail!("Build not found");
        }

        let build = build.swap_remove(0);
        let package = CLIENT.get(format!("{}/{}", ZULU_BASE, build.package_uuid));
        let package: ZuluPackage = package.send().await?.error_for_status()?.json().await?;

        let build = into_java_build(build, package);
        Ok(build)
    }
}
