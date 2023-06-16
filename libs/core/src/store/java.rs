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

use std::path::PathBuf;

use anyhow::Result;
use semver::Version;
use serde::{Deserialize, Serialize};
use tokio::fs::{read_dir, read_to_string};

use crate::models::{JavaBuild, JavaProvider};
use crate::utils::get_dirs;

/// Information about an installed build of Java, stored in the `Java.toml` file.
#[derive(Debug, Serialize, Deserialize)]
pub struct JavaInfo {
    /// The provider that distributes this build.
    pub provider: JavaProvider,
    /// The version of the build.
    pub version: Version,
    /// Size of the uncompressed install in bytes.
    pub size: u64,
    /// Location of the `java` (or `javaw`) executable relative to the `Java.toml` file.
    pub executable: PathBuf,
}

#[allow(clippy::from_over_into)]
impl Into<JavaInfo> for JavaBuild {
    fn into(self) -> JavaInfo {
        JavaInfo {
            provider: self.provider,
            version: self.version,
            size: self.size,
            executable: self.executable,
        }
    }
}

/// Discovers Java builds installed on the system.
pub async fn discover_java() -> Result<Vec<JavaInfo>> {
    let path = get_dirs().data_local_dir().join("java");

    let mut builds = Vec::<JavaInfo>::new();
    let mut entries = read_dir(path).await?;

    while let Some(entry) = entries.next_entry().await? {
        let path = entry.path().join("Java.toml");
        if path.exists() {
            let data: Result<JavaInfo> = (|| async {
                let data = read_to_string(&path).await?;
                let data: JavaInfo = toml::from_str(&data)?;
                Ok(data)
            })()
            .await;
            if let Ok(data) = data {
                builds.push(data);
            }
        }
    }

    Ok(builds)
}
