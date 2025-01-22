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

use uuid::Uuid;

use data::core::auth::{Account, AccountCredentials};
use net::Client;

use crate::{AuthenticationService, Result};

pub struct OfflineAuthenticationService;

impl AuthenticationService for OfflineAuthenticationService {
    /// The credentials for the offline authentication service -- that is, simply the username of
    /// the account that should be logged into.
    type Credentials = String;

    #[tracing::instrument(name = "OfflineAuthenticationService::authenticate", skip_all)]
    async fn authenticate(_client: &Client, credentials: Self::Credentials) -> Result<Account> {
        tracing::debug!("Authorizing offline account {credentials}");
        Ok(Account {
            id: Uuid::new_v4().to_string(),
            username: credentials,
            has_profile: true,
            token: "offline".to_string(),
            expires: None,
            credentials: AccountCredentials::Offline,
        })
    }

    #[tracing::instrument(name = "OfflineAuthenticationService::refresh", skip_all)]
    async fn refresh(_client: &Client, account: Account) -> Result<Account> {
        tracing::debug!("Refreshing offline account {}", account.username);
        Ok(account)
    }
}
