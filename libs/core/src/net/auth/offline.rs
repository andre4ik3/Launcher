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
use uuid::Uuid;

use crate::models::{Account, AccountCredentials};
use crate::net::auth::AuthenticationService;

pub struct OfflineAuthenticationService;

#[async_trait]
impl AuthenticationService<String> for OfflineAuthenticationService {
    async fn authenticate(_client: &Client, credentials: String) -> Result<Account> {
        Ok(Account {
            id: Uuid::new_v4().to_string(),
            username: credentials,
            has_profile: true,
            token: Uuid::new_v4().to_string(), // can be literally anything
            expires: None,
            credentials: AccountCredentials::Offline,
        })
    }

    async fn refresh(_client: &Client, account: Account) -> Result<Account> {
        Ok(account)
    }
}
