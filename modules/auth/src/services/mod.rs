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

use async_trait::async_trait;

use data::core::auth::Account;
pub use microsoft::MicrosoftAuthenticationService;
use net::Client;
pub use offline::OfflineAuthenticationService;

mod microsoft;
mod offline;

/// A generic interface for interacting with an authentication service.
#[async_trait]
pub trait AuthenticationService {
    /// The type of credentials that this authentication service accepts.
    type Credentials;

    /// Authenticates with the service, returning an authenticated account ready to be persisted.
    async fn authenticate(client: &Client, credentials: Self::Credentials) -> crate::Result<Account>;

    /// Refreshes an expired account so that it is ready to be used again.
    async fn refresh(client: &Client, account: Account) -> crate::Result<Account>;
}
