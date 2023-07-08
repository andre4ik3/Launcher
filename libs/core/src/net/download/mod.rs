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

use bytes::Bytes;

pub mod java;

/// The different possible archive formats.
#[non_exhaustive]
#[derive(Debug)]
pub enum ArchiveFormat {
    TarGz,
    TarXz,
    Zip,
}

/// A downloaded archive is something that was downloaded by the network code and is now handed off
/// to the actual download implementation for installation.
#[derive(Debug)]
pub struct DownloadedArchive<T> {
    /// The format of the archive for decompression purposes.
    pub format: ArchiveFormat,
    /// The additional metadata attached to this archive.
    pub metadata: T,
    /// The archive data.
    pub data: Bytes,
}
