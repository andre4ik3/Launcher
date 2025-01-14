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

use std::ops::Deref;

use anyhow::Result;
use sha1::Sha1;
use sha2::{Digest, Sha256};

pub use archive::*;

mod archive;

/// Calculates a SHA256 checksum from some data.
pub fn sha256(data: impl Deref<Target = [u8]>) -> Result<impl AsRef<[u8]>> {
    let mut hasher = Sha256::new();

    for chunk in data.chunks(1024) {
        hasher.update(chunk);
    }

    Ok(hasher.finalize())
}

/// Calculates a SHA1 checksum from some data.
pub fn sha1(data: impl Deref<Target = [u8]>) -> Result<impl AsRef<[u8]>> {
    let mut hasher = Sha1::new();

    for chunk in data.chunks(1024) {
        hasher.update(chunk);
    }

    Ok(hasher.finalize())
}
