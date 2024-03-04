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
pub struct _AdoptiumBuildDownloadable {
    pub name: String,
    pub link: Url,
    pub size: u64,
    pub checksum: String,
}

#[non_exhaustive]
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum _AdoptiumBuildOS {
    Linux,
    Windows,
    Mac,
}

#[allow(clippy::from_over_into)]
impl Into<OS> for _AdoptiumBuildOS {
    fn into(self) -> OS {
        match self {
            _AdoptiumBuildOS::Linux => OS::Linux,
            _AdoptiumBuildOS::Windows => OS::Windows,
            _AdoptiumBuildOS::Mac => OS::MacOS,
        }
    }
}

#[non_exhaustive]
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum _AdoptiumBuildArch {
    X64,
    AArch64,
}

#[allow(clippy::from_over_into)]
impl Into<Arch> for _AdoptiumBuildArch {
    fn into(self) -> Arch {
        match self {
            _AdoptiumBuildArch::X64 => Arch::X86_64,
            _AdoptiumBuildArch::AArch64 => Arch::AArch64,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum _AdoptiumBuildCLib {
    Glibc,
    Musl,
}

#[allow(clippy::from_over_into)]
impl Into<Env> for _AdoptiumBuildCLib {
    fn into(self) -> Env {
        match self {
            _AdoptiumBuildCLib::Glibc => Env::Gnu,
            _AdoptiumBuildCLib::Musl => Env::Musl,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct _AdoptiumBinary {
    pub os: _AdoptiumBuildOS,
    pub architecture: _AdoptiumBuildArch,
    pub c_lib: Option<_AdoptiumBuildCLib>,
    pub package: _AdoptiumBuildDownloadable,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct _AdoptiumBuildVersionData {
    pub semver: Version,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AdoptiumBuild {
    pub binary: _AdoptiumBinary,
    pub version: _AdoptiumBuildVersionData,
}

#[allow(clippy::from_over_into)]
impl Into<JavaBuild> for AdoptiumBuild {
    fn into(self) -> JavaBuild {
        JavaBuild {
            provider: JavaProvider::Adoptium,
            version: self.version.semver,
            executable: match self.binary.os {
                _AdoptiumBuildOS::Linux => "bin/java".into(),
                _AdoptiumBuildOS::Windows => "bin\\javaw.exe".into(),
                _AdoptiumBuildOS::Mac => "Contents/Home/bin/java".into(),
            },
            environment: Environment {
                os: self.binary.os.into(),
                arch: self.binary.architecture.into(),
            },
            name: self.binary.package.name,
            size: self.binary.package.size,
            url: self.binary.package.link,
            checksum: self.binary.package.checksum,
        }
    }
}
