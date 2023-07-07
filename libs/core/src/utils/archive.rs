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

use anyhow::Result;

/// Extracts an archive into a directory.
pub async fn extract(from: impl AsRef<Path>, to: impl AsRef<Path>) -> Result<()> {
    let from = from.as_ref();
    let to = to.as_ref();

    if from.to_string_lossy().ends_with(".tar.gz") {
        println!("tar.gz");
    } else if from.to_string_lossy().ends_with(".tar.xz") {
        println!("tar.xz");
    } else if from.to_string_lossy().ends_with(".zip") {
        println!("zip");
    }

    todo!()
}

pub async fn extract_zip() -> Result<()> {
    todo!()
}

pub async fn extract_tar_gz() -> Result<()> {
    todo!()
}

pub async fn extract_lzma() -> Result<()> {
    todo!()
}
