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

use std::path::Path;

use ron::ser::PrettyConfig;
use serde::Serialize;
use tokio::fs;
use tokio::sync::OnceCell;
use tracing::debug;

static PRETTY_CONFIG: OnceCell<PrettyConfig> = OnceCell::const_new();

/// Shortcut for a path relative to the output directory.
#[macro_export]
macro_rules! path {
    ($($arg:tt)*) => { $crate::root().join(format!($($arg)*)) };
}

/// Shortcut for a versioned path relative to the output directory.
#[macro_export]
macro_rules! vpath {
    ($($arg:tt)*) => { $crate::root().join(format!("v{}", $crate::VERSION)).join(format!($($arg)*)) };
}

/// Shortcut to write a serializable struct to a `.ron` file.
pub async fn write_to_ron_file<T>(path: impl AsRef<Path>, data: &T) -> anyhow::Result<()>
where
    T: ?Sized + Serialize,
{
    let path = path.as_ref();
    let config = PRETTY_CONFIG
        .get_or_init(|| async { PrettyConfig::new().struct_names(true) })
        .await;

    // Ensure the parents of the path exist.
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).await?;
    }

    let mut contents = ron::ser::to_string_pretty(&data, config.clone())?;
    contents.push('\n');

    // Finally, write the data to the file.
    debug!("Writing to {}.", path.display());
    fs::write(path, contents).await?;

    Ok(())
}
