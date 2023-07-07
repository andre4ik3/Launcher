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

use std::fs;
use std::io::{Cursor, Read};
use std::path::{Path, PathBuf};

use anyhow::{bail, Result};
use bytes::{Buf, Bytes};
use flate2::read::GzDecoder;
use tar::Archive as TarArchive;
use tokio::task;
use xz2::read::XzDecoder;
use zip::ZipArchive;

use crate::net::download::ArchiveFormat;

/// Extracts an archive into a directory.
pub async fn extract(data: Bytes, format: ArchiveFormat, dest: impl AsRef<Path>) -> Result<()> {
    let dest = dest.as_ref().to_owned();

    match format {
        ArchiveFormat::TarGz => extract_tar_gz(data, dest).await?,
        ArchiveFormat::TarXz => extract_tar_xz(data, dest).await?,
        ArchiveFormat::Zip => extract_zip(data, dest).await?,
    }

    Ok(())
}

async fn extract_tar<T: Read + Send + 'static>(
    mut archive: TarArchive<T>,
    dest: PathBuf,
) -> Result<()> {
    // run the whole thing on a blocking thread because the tar operation is blocking and for some
    // random rust lifetime reason it doesn't let me wrap just the unpack() call.
    task::spawn_blocking(move || {
        for mut entry in archive.entries()?.flatten() {
            let path = dest.join(entry.path()?.as_ref()).canonicalize()?;

            // === CRITICAL SECURITY CHECK ===
            // This prevents path traversal exploits (zip files can contain paths like ..)
            // It's also critical that .canonicalize() runs before this. (to resolve the ..)
            let safe = path.ancestors().any(|p| p == dest);
            if !safe {
                bail!(
                    "Path of {} is outside the destination {}. Aborting!",
                    path.to_string_lossy(),
                    dest.to_string_lossy()
                );
            }

            fs::create_dir_all(path.parent().unwrap())?;
            entry.unpack(path)?;
        }

        Ok(())
    })
    .await?
}

async fn extract_tar_gz(data: Bytes, dest: PathBuf) -> Result<()> {
    let decoder = GzDecoder::new(data.reader());
    let archive = TarArchive::new(decoder);
    extract_tar(archive, dest).await
}

async fn extract_tar_xz(data: Bytes, dest: PathBuf) -> Result<()> {
    let decoder = XzDecoder::new(data.reader());
    let archive = TarArchive::new(decoder);
    extract_tar(archive, dest).await
}

async fn extract_zip(data: Bytes, dest: PathBuf) -> Result<()> {
    let mut archive = ZipArchive::new(Cursor::new(data))?;
    task::spawn_blocking(move || archive.extract(dest)).await??;
    Ok(())
}
