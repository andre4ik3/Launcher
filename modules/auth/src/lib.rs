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

//! Launcher Authentication Module
//! ==============================
//!
//! This module deals with everything to do with online account authentication. Each authentication
//! service is divided into its own module, such as [MicrosoftAuthenticationService] for MSA
//! accounts, or [OfflineAuthenticationService] for offline-mode accounts. All authentication
//! services implement the [AuthenticationService] trait and return [Account]s, ready for use in
//! higher-level code.

use thiserror::Error;

pub use services::*;

mod services;
mod store;

#[derive(Debug, Error)]
pub enum Error {
    #[error("wrong account type in authentication service: expected {0} but got {1}")]
    WrongAccountType(&'static str, &'static str),
    #[error("reauthentication is required")]
    ReauthenticationRequired,
    #[error("network error: {0}")]
    NetworkError(#[from] net::Error),
    #[error("failed decoding response from xbox api")]
    DecodingError,
}

pub type Result<T> = core::result::Result<T, Error>;
