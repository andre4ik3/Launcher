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

use platforms::{Arch, OS};
use semver::Version;
use serde::{Deserialize, Serialize};
use url::Url;

/// A struct that describes the environment the binary is running in.
/// (Somewhat similar to a target triple)
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Deserialize, Serialize)]
pub struct Environment {
    pub os: OS,
    pub arch: Arch,
}

/// An enum of possible providers of Java builds.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub enum JavaProvider {
    /// Azul Zulu -- https://www.azul.com/downloads/#zulu
    Zulu,
}

/// The type of build (JDK or JRE).
#[derive(Clone, Debug, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub enum JavaEdition {
    /// JDKs are full builds that have everything to both run and develop Java programs.
    JDK,
    /// JREs are slimmer builds that take up less space by removing resources for development.
    JRE,
}

/// A single downloadable build of Java.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct JavaBuild {
    /// The provider that created this build -- used to determine how to extract the build.
    pub provider: JavaProvider,
    /// The version of Java that is contained in this build.
    pub version: Version,
    /// The edition/type of this build (JDK or JRE).
    pub edition: JavaEdition,
    /// The environment that this build was created for (OS and arch).
    pub environment: Environment,
    /// Relative path of the executable file (`java` or `javaw`) to the root of the archive file.
    pub executable: String,
    /// URL where this build can be downloaded.
    pub download: Url,
    /// Name of the archive file.
    pub name: String,
    /// Size of the archive file in bytes.
    pub size: u64,
    /// SHA256 checksum of the archive file.
    pub checksum: String,
}

/// Metadata stored within an extracted Java installation (`Java.toml` file).
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct JavaInfo {
    /// The provider that created this build -- shown in the UI.
    pub provider: JavaProvider,
    /// The version of Java that is contained in this build.
    pub version: Version,
    /// The edition/type of this build (JDK or JRE).
    pub edition: JavaEdition,
    /// The environment that this build was created for (OS and arch).
    pub environment: Environment,
    /// Relative path of the executable file (`java` or `javaw`) to the root of the installation.
    pub executable: String,
    /// Size of this installation on disk.
    pub size: u64,
}
