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

use platforms::OS;
use url::Url;

use macros::data_structure;

use crate::core::conditional::{Condition, MaybeConditional};
use crate::core::maven::MavenIdentifier;

/// Abstract type for a downloadable game file.
#[data_structure]
pub struct LibraryDownloadable {
    /// The relative path where the file will be saved.
    pub path: String,
    /// The SHA1 checksum of the file.
    pub checksum: String,
    /// The size of the file in bytes.
    pub size: u64,
    /// The URL of the file.
    pub url: Url,
}

/// A library is a JAR file that is downloaded and put into the `classpath` to be loaded by the JVM.
#[data_structure]
pub struct Library {
    /// The name of the library, in this format: `com.example:hello:1.0`.
    pub name: MavenIdentifier,
    /// The library file itself.
    pub file: LibraryDownloadable,
}

// === conversion ===

#[cfg(feature = "silo")]
impl From<crate::silo::game::ApiLibraryArtifact> for LibraryDownloadable {
    fn from(value: crate::silo::game::ApiLibraryArtifact) -> Self {
        Self {
            path: value.path,
            checksum: value.sha1,
            size: value.size,
            url: value.url,
        }
    }
}

#[cfg(feature = "silo")]
impl From<crate::silo::game::ApiCommonLibrary> for MaybeConditional<Library> {
    fn from(value: crate::silo::game::ApiCommonLibrary) -> Self {
        let library = Library {
            name: value.name,
            file: value.downloads.artifact.into(),
        };

        match value.rules {
            None => MaybeConditional::Unconditional(library),
            Some(rules) => MaybeConditional::Conditional {
                when: Condition::And(rules.into_iter().map(Condition::from).collect()).simplify(),
                then: library,
            },
        }
    }
}

#[cfg(feature = "silo")]
impl From<crate::silo::game::ApiNativeLibrary> for Vec<MaybeConditional<Library>> {
    fn from(mut value: crate::silo::game::ApiNativeLibrary) -> Self {
        let mut libraries = Vec::<MaybeConditional<Library>>::new();

        // TODO: Make this more readable
        let keys: Vec<(String, OS)> = vec![
            value.natives.linux,
            value.natives.osx,
            value.natives.windows,
        ]
        .into_iter()
        .zip(vec![OS::Linux, OS::MacOS, OS::Windows])
        .filter(|(key, _)| key.is_some())
        .map(|(key, os)| (key.unwrap(), os))
        .collect();

        // Base condition for the library (if exists).
        let base_condition: Option<Condition> = value
            .rules
            .map(|s| Condition::And(s.into_iter().map(Condition::from).collect()).simplify());

        for (key, os) in keys {
            // Mojang classic. In some libraries, there are native keys, but no corresponding
            // artifact in downloads. See, for example, second library in rd-132211.
            let artifact = match value.downloads.classifiers.remove(&key) {
                Some(artifact) => artifact,
                None => continue,
            };

            // TODO: In some cases, there are two conditions back-to-back, e.g. (macos, *) and (macos, =10.5).
            // They should be de-duped. Preferably in Condition::simplify.
            let os_condition = Condition::OS(os);
            let when = match base_condition.clone() {
                Some(base_condition) => Condition::And(vec![base_condition, os_condition]),
                None => os_condition,
            };

            libraries.push(MaybeConditional::Conditional {
                when,
                then: Library {
                    name: value.name.clone(),
                    file: artifact.into(),
                },
            });
        }

        if let Some(artifact) = value.downloads.artifact {
            libraries.push(MaybeConditional::Unconditional(Library {
                name: value.name.clone(),
                file: artifact.into(),
            }));
        }

        libraries
    }
}

#[cfg(feature = "silo")]
impl From<crate::silo::game::ApiLibrary> for Vec<MaybeConditional<Library>> {
    fn from(value: crate::silo::game::ApiLibrary) -> Self {
        match value {
            crate::silo::game::ApiLibrary::Common(lib) => vec![lib.into()],
            crate::silo::game::ApiLibrary::Native(libs) => libs.into(),
        }
    }
}
