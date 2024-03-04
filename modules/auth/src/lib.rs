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

//! Launcher Authentication Module
//! ==============================
//!
//! This module deals with everything to do with online account authentication. Each authentication
//! service is divided

use async_trait::async_trait;
use thiserror::Error;

use data::auth::Account;
pub use microsoft::MicrosoftAuthenticationService;
use net::Client;
pub use offline::OfflineAuthenticationService;

pub(crate) mod microsoft;
pub(crate) mod offline;

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

/// A generic interface for interacting with an authentication service.
#[async_trait]
pub trait AuthenticationMethod {
    /// The type of credentials that this authentication service accepts.
    type Credentials;

    /// Authenticates with the service, returning an authenticated account ready to be persisted.
    async fn authenticate(client: &Client, credentials: Self::Credentials) -> Result<Account>;

    /// Refreshes an expired account so that it is ready to be used again.
    async fn refresh(client: &Client, account: Account) -> Result<Account>;
}
