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

//! Launcher Fetch Module
//! =====================
//!
//! This module contains high-level procedures for fetching and storing different things. It is a
//! combination of the [data], [net], and [persistence] modules.
//!
//! It consists of a few separate parts:
//!
//! - [meta]

pub mod meta;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("network error: {0}")]
    NetworkError(#[from] net::Error),
    #[error("url parse error: {0}")]
    UrlParseError(#[from] url::ParseError),
    #[error("ron parse error: {0}")]
    RonParseError(#[from] ron::de::SpannedError),
    #[error("unsupported api version: we only support {0}, api supports {1}")]
    ApiVersionMismatch(u64, String),
}

pub type Result<T> = core::result::Result<T, Error>;
