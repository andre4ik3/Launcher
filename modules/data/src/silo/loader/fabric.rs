// Copyright © 2023-2025 andre4ik3
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

use macros::api_response;
use url::Url;

use crate::core::maven::MavenIdentifier;

#[api_response]
pub struct FabricVersionLoader {
    pub separator: String,
    pub build: u64,
    pub maven: MavenIdentifier,
    pub version: String,
    pub stable: bool,
}

#[api_response]
pub struct FabricVersionIntermediary {
    pub maven: MavenIdentifier,
    pub version: String,
    pub stable: bool,
}

#[api_response(strict = false)]
pub struct FabricLibrary {
    pub name: MavenIdentifier,
    pub url: Option<Url>,
}

#[api_response(strict = false)]
pub struct FabricVersionLibraries {
    pub client: Vec<FabricLibrary>,
    pub common: Vec<FabricLibrary>,
    pub server: Vec<FabricLibrary>,
}

#[api_response(rename = "camelCase")]
pub struct FabricVersionLauncherMeta {
    pub version: u64,
    #[serde(rename = "min_java_version")]
    pub min_java_version: Option<u64>,
    pub libraries: FabricVersionLibraries,
    pub main_class: FabricVersionMainClass,
    pub arguments: Option<FabricVersionLauncherMetaArguments>,
    pub launchwrapper: Option<FabricVersionLauncherMetaLaunchWrapper>,
}

#[api_response]
pub struct FabricVersionLauncherMetaLaunchWrapperTweakers {
    pub client: Vec<String>,
    pub common: Vec<String>,
    pub server: Vec<String>,
}

#[api_response]
pub struct FabricVersionLauncherMetaArguments {
    pub client: Vec<String>,
    pub common: Vec<String>,
    pub server: Vec<String>,
}

#[api_response]
pub struct FabricVersionLauncherMetaLaunchWrapper {
    pub tweakers: FabricVersionLauncherMetaArguments,
}

#[api_response]
pub enum FabricVersionMainClass {
    Constant(String),
    Variable { client: String, server: String },
}

#[api_response(rename = "camelCase")]
pub struct FabricVersion {
    pub loader: FabricVersionLoader,
    pub intermediary: FabricVersionIntermediary,
    pub launcher_meta: FabricVersionLauncherMeta,
}
