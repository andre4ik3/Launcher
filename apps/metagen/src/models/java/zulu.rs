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
pub struct ZuluBuild {
    pub package_uuid: String,
    pub java_version: (u64, u64, u64),
    pub name: String,
    pub download_url: Url,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ZuluPackageOS {
    Linux,
    MacOS,
    Windows,
}

#[allow(clippy::from_over_into)]
impl Into<OS> for ZuluPackageOS {
    fn into(self) -> OS {
        match self {
            ZuluPackageOS::Linux => OS::Linux,
            ZuluPackageOS::MacOS => OS::MacOS,
            ZuluPackageOS::Windows => OS::Windows,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ZuluPackageArch {
    Arm,
    X64,
}

#[allow(clippy::from_over_into)]
impl Into<Arch> for ZuluPackageArch {
    fn into(self) -> Arch {
        match self {
            ZuluPackageArch::Arm => Arch::AArch64,
            ZuluPackageArch::X64 => Arch::X86_64,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ZuluPackageCLib {
    Glibc,
    UClibc,
    Musl,
}

#[allow(clippy::from_over_into)]
impl Into<Env> for ZuluPackageCLib {
    fn into(self) -> Env {
        match self {
            ZuluPackageCLib::Glibc => Env::Gnu,
            ZuluPackageCLib::UClibc => Env::UClibc,
            ZuluPackageCLib::Musl => Env::Musl,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ZuluPackage {
    pub sha256_hash: String,
    pub size: u64,
    pub os: ZuluPackageOS,
    pub lib_c_type: Option<ZuluPackageCLib>,
    pub arch: ZuluPackageArch,
}

pub fn into_java_build(build: ZuluBuild, package: ZuluPackage) -> JavaBuild {
    let env = if package.os == ZuluPackageOS::Windows {
        Env::Msvc
    } else if package.os == ZuluPackageOS::Linux {
        package.lib_c_type.unwrap_or(ZuluPackageCLib::Glibc).into()
    } else {
        Env::None
    };

    JavaBuild {
        provider: JavaProvider::Zulu,
        version: Version::new(
            build.java_version.0,
            build.java_version.1,
            build.java_version.2,
        ),
        environment: Environment {
            os: package.os.into(),
            arch: package.arch.into(),
            env,
        },
        name: build.name,
        size: package.size,
        url: build.download_url,
        checksum: package.sha256_hash,
    }
}
