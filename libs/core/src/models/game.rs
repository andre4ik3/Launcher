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

use chrono::NaiveDateTime;
use platforms::{Arch, OS};
use semver::VersionReq;
use serde::{Deserialize, Serialize};
use url::Url;

/// Condition for inclusion of arguments and libraries.
#[derive(Debug, Deserialize, Serialize)]
pub enum Condition {
    Feature(String),
    OS(OS),
    Arch(Arch),
    And(Vec<Condition>),
    Or(Vec<Condition>),
    Not(Vec<Condition>),
}

/// A condition for including game arguments and libraries, similar to an `if` statement.
#[derive(Debug, Deserialize, Serialize)]
pub struct GameConditional<T> {
    /// The condition to match.
    pub when: Condition,
    /// The structs to include when the condition is matched.
    pub then: Vec<T>,
}

/// Helper for including both conditional and unconditional types in one type.
#[derive(Debug, Deserialize, Serialize)]
pub enum GameMaybeConditional<T> {
    Unconditional(T),
    Conditional(GameConditional<T>),
}

/// The different stability types of game releases.
#[derive(Debug, Deserialize, Serialize)]
pub enum GameVersionStability {
    Release,
    Snapshot,
    Experimental,
    OldBeta,
    OldAlpha,
}

/// Abstract type for a downloadable game file.
#[derive(Debug, Deserialize, Serialize)]
pub struct GameDownloadable {
    /// The relative path where the file will be saved.
    pub path: String,
    /// The SHA1 checksum of the file.
    pub checksum: String,
    /// The size of the file in bytes.
    pub size: u64,
    /// The URL of the file.
    pub url: Url,
}

/// Information and location of the version's asset index.
#[derive(Debug, Deserialize, Serialize)]
pub struct GameAssetIndex {
    /// The ID of the asset index.
    pub id: String,
    /// The combined size of all downloaded and extracted assets.
    pub total_size: u64,
    /// The asset index file.
    pub file: GameDownloadable,
}

/// Information and location of a version's library.
#[derive(Debug, Deserialize, Serialize)]
pub struct GameLibrary {
    /// The name of the library, in this format: `com.example:hello:1.0`
    pub name: String,
    /// The library file.
    pub file: GameDownloadable,
}

/// The main game files - client, server, and their mappings.
#[derive(Debug, Deserialize, Serialize)]
pub struct GameFiles {
    /// The client.jar file.
    pub client: GameDownloadable,
    /// Mappings for the client.jar file.
    pub client_mappings: GameDownloadable,
    /// The server.jar file.
    pub server: GameDownloadable,
    /// Mappings for the server.jar file.
    pub server_mappings: GameDownloadable,
}

/// Information needed to launch a particular game version.
#[derive(Debug, Deserialize, Serialize)]
pub struct GameVersion {
    /// The unique game version. Note: doesn't follow SemVer.
    pub version: String,
    /// The stability of this game version.
    pub stability: GameVersionStability,
    /// The version of Java that the game requires.
    pub java: VersionReq,
    /// The Java class that serves as the entry point for this version.
    pub entrypoint: String,
    /// Date and time of release of this version.
    pub released: NaiveDateTime,
    /// Arguments to be passed to the game.
    pub arguments: Vec<GameMaybeConditional<String>>,
    /// Arguments to be passed to the Java virtual machine.
    pub arguments_java: Vec<GameMaybeConditional<String>>,
    /// Information about the game's asset index.
    pub assets: GameAssetIndex,
    /// The libraries that the game depends on.
    pub libraries: Vec<GameMaybeConditional<GameLibrary>>,
    /// Downloads for the main game files.
    pub files: GameFiles,
}
