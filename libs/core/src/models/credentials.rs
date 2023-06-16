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

use serde::{Deserialize, Serialize};

/// The different types of accounts and their credentials.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum AccountCredentials {
    /// MSA (Microsoft Authentication) accounts.
    Microsoft {
        // todo
    },

    /// Mojang and Minecraft.net accounts.
    Yggdrasil { username: String, password: String },

    /// Offline-mode / demo-mode accounts.
    Offline { username: String },
}

/// An account used for logging into the game.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Account {
    /// The UUID of the account.
    pub id: String,
    /// The login credentials to use for the account.
    pub credentials: AccountCredentials,
}

/// The secure credentials store that is encrypted on disk.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Credentials {
    pub accounts: Vec<Account>,
}
