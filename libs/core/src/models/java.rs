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

use semver::Version;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::models::Environment;

/// An enum for currently defined providers of Java builds, subject to change in the future.
#[non_exhaustive]
#[derive(Debug, Serialize, Deserialize)]
pub enum JavaProvider {
    Adoptium,
    Zulu,
}

/// Information about an available build of Java for installation.
#[derive(Debug, Serialize, Deserialize)]
pub struct JavaBuild {
    /// The provider that distributes this build.
    pub provider: JavaProvider,
    /// The version of this build.
    pub version: Version,
    /// Environment (OS, Arch, Env) that the build is for.
    pub environment: Environment,
    /// Name of the installer file.
    pub name: String,
    /// Size of the installer file in bytes.
    pub size: u64,
    /// URL of the installer file.
    pub url: Url,
    /// SHA256 checksum of the installer file.
    pub checksum: String,
}

/// Information about an installed build of Java, stored in the `Java.toml` file.
#[derive(Debug, Serialize, Deserialize)]
pub struct JavaInfo {
    /// The provider that distributes this build.
    pub provider: JavaProvider,
    /// The version of the build.
    pub version: Version,
    /// Size of the uncompressed install in bytes.
    pub size: u64,
}
