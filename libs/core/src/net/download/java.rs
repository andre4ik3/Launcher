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

use anyhow::{bail, Result};
use bytes::{BufMut, BytesMut};
use futures_util::StreamExt;
use reqwest::Client;
use sha2::{Digest, Sha256};

use crate::models::{JavaBuild, JavaInfo};
use crate::net::download::{ArchiveFormat, DownloadedArchive};
use crate::utils::try_request;

pub async fn download(client: &Client, build: JavaBuild) -> Result<DownloadedArchive<JavaBuild>> {
    // TODO: Resume interrupted downloads
    let request = client.get(build.url.clone()).build()?;
    let resp = try_request(client, request).await?;
    let mut resp = resp.bytes_stream();

    // Store the entire archive in memory. Should be OK since Java builds are at most 200MB-ish.
    let mut hasher = Sha256::new();
    let mut data = BytesMut::with_capacity(build.size as usize);

    // This calculates the hash _as the data is coming in_ so it should be smoother.
    // TODO: am I absolutely positively sure that hasher.update() doesn't block? Could cause hitches
    while let Some(chunk) = resp.next().await {
        let chunk = chunk?;
        hasher.update(&chunk);
        data.put(chunk);
    }

    let hash = hasher.finalize();
    let build_hash = hex::decode(&build.checksum)?;

    if build_hash != hash.to_vec() {
        bail!("Failed verification");
    }

    let format = if build.name.ends_with(".tar.gz") {
        ArchiveFormat::TarGz
    } else if build.name.ends_with(".tar.xz") {
        ArchiveFormat::TarXz
    } else if build.name.ends_with(".zip") {
        ArchiveFormat::Zip
    } else {
        bail!("Could not determine archive format: {}", build.name);
    };

    Ok(DownloadedArchive {
        data: data.into(),
        metadata: build,
        format,
    })
}
