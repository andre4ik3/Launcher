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

use chrono::{DateTime, Utc};
use macros::data_structure;

/// The different types of accounts and their credentials.
#[data_structure]
pub enum AccountCredentials {
    /// MSA (Microsoft Authentication) accounts.
    Microsoft { access: String, refresh: String },

    /// Offline-mode / demo-mode accounts. Uses random token and ID.
    Offline,
}

/// An account used for logging into the game.
#[data_structure]
pub struct Account {
    /// The UUID of the account.
    pub id: String,
    /// The in-game username of the account.
    pub username: String,
    /// Whether the account has a profile (and therefore rights to play the game).
    pub has_profile: bool,
    /// The last received game token belonging to the account.
    pub token: String,
    /// When the game token stored is set to expire.
    pub expires: Option<DateTime<Utc>>,
    /// The login credentials to use for the account.
    pub credentials: AccountCredentials,
}
