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

use std::path::Path;

use anyhow::{anyhow, Result};
use indicatif::ProgressStyle;
use ron::ser::{to_string_pretty, PrettyConfig};
use serde::Serialize;
use tokio::fs;

/// Base path where all files are written.
const BASE_PATH: &str = "_site/v1";

/// Generates a progress bar style.
pub fn prog_style() -> ProgressStyle {
    ProgressStyle::with_template("  {msg:<25.bold.green} [{bar:30}] {pos}/{len} {eta}")
        .expect("Failed making progress bar style")
        .progress_chars("=>-")
}

/// Dumps a file to a relative path in the meta site root.
pub async fn dump(path: impl AsRef<Path>, data: &impl Serialize) -> Result<()> {
    let base: &Path = BASE_PATH.as_ref();
    let path = base.join(path.as_ref());

    // Ensure parent directories exist
    let parent = path.parent().ok_or(anyhow!("No parent"))?;
    fs::create_dir_all(parent).await?;

    let config = PrettyConfig::default().struct_names(true);
    let mut data = to_string_pretty(data, config)?;
    data.push('\n'); // Trailing newline >:)

    fs::write(path, data).await?;
    Ok(())
}
