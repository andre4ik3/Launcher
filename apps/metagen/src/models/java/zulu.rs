// Copyright © 2023-2024 andre4ik3
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

use platforms::{Arch, Env, OS};
use semver::Version;
use serde::{Deserialize, Serialize};
use url::Url;

use launcher::models::{Environment, JavaBuild, JavaProvider};

#[derive(Debug, Deserialize, Serialize)]
pub struct ZuluBuild {
    pub package_uuid: String,
    pub java_version: (u64, u64, u64),
    pub name: String,
    pub download_url: Url,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum _ZuluPackageOS {
    Linux,
    MacOS,
    Windows,
}

#[allow(clippy::from_over_into)]
impl Into<OS> for _ZuluPackageOS {
    fn into(self) -> OS {
        match self {
            _ZuluPackageOS::Linux => OS::Linux,
            _ZuluPackageOS::MacOS => OS::MacOS,
            _ZuluPackageOS::Windows => OS::Windows,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum _ZuluPackageArch {
    Arm,
    X64,
}

#[allow(clippy::from_over_into)]
impl Into<Arch> for _ZuluPackageArch {
    fn into(self) -> Arch {
        match self {
            _ZuluPackageArch::Arm => Arch::AArch64,
            _ZuluPackageArch::X64 => Arch::X86_64,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum _ZuluPackageCLib {
    Glibc,
    UClibc,
    Musl,
}

#[allow(clippy::from_over_into)]
impl Into<Env> for _ZuluPackageCLib {
    fn into(self) -> Env {
        match self {
            _ZuluPackageCLib::Glibc => Env::Gnu,
            _ZuluPackageCLib::UClibc => Env::UClibc,
            _ZuluPackageCLib::Musl => Env::Musl,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ZuluPackage {
    pub sha256_hash: String,
    pub size: u64,
    pub os: _ZuluPackageOS,
    pub lib_c_type: Option<_ZuluPackageCLib>,
    pub arch: _ZuluPackageArch,
}

/// Converts a Zulu build and package struct into a unified JavaBuild struct.
pub fn into_java_build(build: ZuluBuild, package: ZuluPackage) -> JavaBuild {
    JavaBuild {
        provider: JavaProvider::Zulu,
        version: Version::new(
            build.java_version.0,
            build.java_version.1,
            build.java_version.2,
        ),
        executable: match package.os {
            _ZuluPackageOS::Linux => "bin/java".into(),
            _ZuluPackageOS::Windows => "bin\\javaw.exe".into(),
            _ZuluPackageOS::MacOS => "Contents/Home/bin/java".into(),
        },
        environment: Environment {
            os: package.os.into(),
            arch: package.arch.into(),
        },
        name: build.name,
        size: package.size,
        url: build.download_url,
        checksum: package.sha256_hash,
    }
}
