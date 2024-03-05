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

use thiserror::Error;

pub mod directory;
pub mod file;

#[derive(Debug, Error)]
pub enum Error {
    #[error("failed to serialize entry: {0}")]
    Serialize(#[from] toml::ser::Error),

    #[error("failed to deserialize entry: {0}")]
    Deserialize(#[from] toml::de::Error),

    #[error("failed to read/write entry: {0}")]
    Io(#[from] std::io::Error),

    #[error("failed to perform crypto operation: {0}")]
    Crypto(#[from] crate::crypto::Error),
}

pub type Result<T> = core::result::Result<T, Error>;
