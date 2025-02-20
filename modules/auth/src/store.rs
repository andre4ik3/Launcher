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

use async_once_cell::OnceCell;
use tokio::sync::RwLockReadGuard;

use data::core::auth::Account;
use persistence::{FileRegistry, Result};

type Registry = FileRegistry<Vec<Account>>;

static REGISTRY: OnceCell<Registry> = OnceCell::new();

pub struct CredentialStore<'a> {
    registry: &'a Registry,
}

impl CredentialStore {
    async fn get() -> Self {
        Self {
            registry: REGISTRY
                .get_or_init(async {
                    FileRegistry::new_encrypted("Credentials.dat")
                        .await
                        .expect("failed to create registry")
                })
                .await,
        }
    }

    /// Retrieves all accounts from the credential store in borrowed form.
    async fn accounts(&self) -> RwLockReadGuard<'_, Vec<Account>> {
        self.registry.get().await
    }

    /// Retrieves all accounts from the credential store in owned form.
    async fn accounts_owned(&self) -> Vec<Account> {
        self.accounts().await.clone()
    }

    /// Adds a new account to the credential store.
    async fn insert(&mut self, account: Account) -> Result<()> {
        self.registry.get_mut().await.push(account);
        self.registry.save().await?;
        Ok(())
    }
}
