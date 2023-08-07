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

//! Launcher Networking Module
//! ==========================
//!
//! This module contains all network-related functionality. This crate is the only one that depends
//! on the [reqwest] crate, providing safe wrappers around it that add a request queue, retry logic,
//! download resuming, and some generally nice utilities.

use thiserror::Error;

pub use client::*;

pub(crate) mod client;
pub(crate) mod queue;

#[derive(Debug, Error)]
pub enum Error {
    #[error("network error: {0}")]
    Network(#[from] reqwest::Error),
    #[error("request failed after {0} attempts, last error: {1}")]
    RequestAttemptsExhausted(u64, reqwest::Error),
    #[error("request cannot be cloned (required for retrying)")]
    RequestCloneFail,
    #[error("queue has been shut down")]
    QueueShutDown,
}
