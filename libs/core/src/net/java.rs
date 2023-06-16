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

use crate::models::JavaBuild;

pub async fn download_java(client: &Client, build: JavaBuild) -> Result<()> {
    let resp = client.get(build.url).send().await?;
    let mut resp = resp.bytes_stream();
    let mut hasher = Sha256::new();

    // Store the entire archive in memory. Should be OK since Java builds are at most 200MB-ish.
    // TODO: Figure out a way to download, verify checksum, and expand archive only after successful
    // TODO: verification, if it's even possible?
    let mut data = BytesMut::new();
    data.reserve(build.size as usize);

    while let Some(chunk) = resp.next().await {
        let chunk = chunk?;
        hasher.update(&chunk);
        data.put(chunk);
    }

    let hash = hasher.finalize();
    let build_hash = hex::decode(build.checksum)?;

    if build_hash != hash.to_vec() {
        bail!("Failed verification");
    }

    Ok(())
}
