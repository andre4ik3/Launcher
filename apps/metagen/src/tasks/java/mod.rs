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
use async_trait::async_trait;
use launcher::models::java::JavaBuild;
use launcher::models::Environment;
use platforms::{Arch, OS};

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

pub async fn run() -> Result<()> {
    for target in JAVA_TARGETS {
        let build = {
            if let Ok(build) = adoptium::Adoptium::fetch(17, &target).await {
                build
            } else {
                zulu::Zulu::fetch(17, &target).await?
            }
        };
        println!("{:?}", build);
    }
    Ok(())
}
