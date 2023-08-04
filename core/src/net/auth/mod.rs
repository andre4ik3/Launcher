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

use anyhow::Result;
use async_trait::async_trait;
use reqwest::Client;

pub use microsoft::{get_auth_url, MicrosoftAuthenticationService};
pub use offline::OfflineAuthenticationService;

use crate::models::Account;

mod microsoft;
mod offline;

/// A generic interface for interacting with an authentication service.
#[async_trait]
pub trait AuthenticationService<T> {
    /// Authenticates with the service, giving back an account struct.
    async fn authenticate(client: &Client, credentials: T) -> Result<Account>;

    /// Refreshes an account for a new game token.
    async fn refresh(client: &Client, account: Account) -> Result<Account>;
}
