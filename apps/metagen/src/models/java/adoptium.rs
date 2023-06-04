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

use launcher::models::java::{JavaBuild, JavaProvider};
use launcher::models::Environment;
use platforms::{Arch, Env, OS};
use semver::Version;
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Deserialize, Serialize)]
pub struct AdoptiumBuildDownloadable {
    pub name: String,
    pub link: Url,
    pub size: u64,
    pub checksum: String,
}

#[non_exhaustive]
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum AdoptiumBuildOS {
    Linux,
    Windows,
    Mac,
}

#[allow(clippy::from_over_into)]
impl Into<OS> for AdoptiumBuildOS {
    fn into(self) -> OS {
        match self {
            AdoptiumBuildOS::Linux => OS::Linux,
            AdoptiumBuildOS::Windows => OS::Windows,
            AdoptiumBuildOS::Mac => OS::MacOS,
        }
    }
}

#[non_exhaustive]
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum AdoptiumBuildArch {
    X64,
    AArch64,
}

#[allow(clippy::from_over_into)]
impl Into<Arch> for AdoptiumBuildArch {
    fn into(self) -> Arch {
        match self {
            AdoptiumBuildArch::X64 => Arch::X86_64,
            AdoptiumBuildArch::AArch64 => Arch::AArch64,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum AdoptiumBuildCLib {
    Glibc,
    Musl,
}

#[allow(clippy::from_over_into)]
impl Into<Env> for AdoptiumBuildCLib {
    fn into(self) -> Env {
        match self {
            AdoptiumBuildCLib::Glibc => Env::Gnu,
            AdoptiumBuildCLib::Musl => Env::Musl,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AdoptiumBinary {
    pub os: AdoptiumBuildOS,
    pub architecture: AdoptiumBuildArch,
    pub c_lib: Option<AdoptiumBuildCLib>,
    pub package: AdoptiumBuildDownloadable,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AdoptiumBuildVersionData {
    pub semver: Version,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AdoptiumBuild {
    pub binary: AdoptiumBinary,
    pub version: AdoptiumBuildVersionData,
}

#[allow(clippy::from_over_into)]
impl Into<JavaBuild> for AdoptiumBuild {
    fn into(self) -> JavaBuild {
        let env = if self.binary.os == AdoptiumBuildOS::Windows {
            Env::Msvc
        } else if self.binary.os == AdoptiumBuildOS::Linux {
            self.binary.c_lib.unwrap_or(AdoptiumBuildCLib::Glibc).into()
        } else {
            Env::None
        };

        JavaBuild {
            provider: JavaProvider::Adoptium,
            version: self.version.semver,
            environment: Environment {
                os: self.binary.os.into(),
                arch: self.binary.architecture.into(),
                env,
            },
            name: self.binary.package.name,
            size: self.binary.package.size,
            url: self.binary.package.link,
            checksum: self.binary.package.checksum,
        }
    }
}
