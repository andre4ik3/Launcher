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

use std::path::Path;

use serde::Serialize;
use tokio::fs;
use tracing::debug;

pub async fn write_to_ron_file<T>(path: impl AsRef<Path>, data: &T) -> anyhow::Result<()>
    where T: ?Sized + Serialize {
    let path = path.as_ref();

    // Ensure the parents of the path exist.
    if let Some(parent) = path.parent() {
        tokio::fs::create_dir_all(parent).await?;
    }

    // Finally, write the data to the file.
    debug!("Writing to {}.", path.display());
    fs::write(path, ron::ser::to_string_pretty(&data, ron::ser::PrettyConfig::default())?).await?;

    Ok(())
}
