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

use crate::utils::{dump, prog_style};
use anyhow::Result;
use async_trait::async_trait;
use indicatif::ProgressBar;
use launcher::models::java::JavaBuild;
use launcher::models::Environment;
use platforms::{Arch, OS};
use std::collections::HashSet;

mod adoptium;
mod zulu;

const JAVA_TARGETS: [Environment; 6] = [
    Environment {
        os: OS::Linux,
        arch: Arch::AArch64,
    },
    Environment {
        os: OS::Linux,
        arch: Arch::X86_64,
    },
    Environment {
        os: OS::MacOS,
        arch: Arch::AArch64,
    },
    Environment {
        os: OS::MacOS,
        arch: Arch::X86_64,
    },
    Environment {
        os: OS::Windows,
        arch: Arch::AArch64,
    },
    Environment {
        os: OS::Windows,
        arch: Arch::X86_64,
    },
];

/// An abstract interface to fetch a specific Java build from the provider
#[async_trait]
pub trait Provider {
    async fn fetch(version: u8, env: &Environment) -> Result<JavaBuild>;
}

pub async fn run(versions: HashSet<u8>) -> Result<()> {
    println!("Generating Java metadata...");

    let pb = ProgressBar::new((JAVA_TARGETS.len() * versions.len()) as u64);
    pb.set_style(prog_style());

    for target in JAVA_TARGETS {
        for version in versions.iter() {
            pb.inc(1);
            pb.set_message(format!("Java {} {}-{}", version, target.arch, target.os));

            let adoptium_build = adoptium::Adoptium::fetch(*version, &target).await.ok();
            let zulu_build = zulu::Zulu::fetch(*version, &target).await.ok();

            let build = adoptium_build.or(zulu_build);
            if let Some(build) = build {
                let path = format!("java/{}/{}/{}.ron", target.arch, target.os, version);
                dump(path, &build).await?;
            }
        }
    }

    Ok(())
}
