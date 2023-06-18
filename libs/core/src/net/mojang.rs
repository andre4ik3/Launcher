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
use reqwest::Client;
use serde::{Deserialize, Serialize};
use url::Url;

const PROFILE_URL: &str = "https://api.minecraftservices.com/minecraft/profile";

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UserCosmeticState {
    /// The cosmetic is inactive (unselected).
    Inactive,
    /// The cosmetic is active (selected).
    Active,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UserSkinVariant {
    /// The Classic "Steve" (4px wide arms) skin variant.
    Classic,
    /// The Slim "Alex" (3px wide arms) skin variant.
    Slim,
}

/// A skin is a cosmetic all players have.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UserSkin {
    /// The UUID of the skin.
    pub id: String,
    /// The state of the skin (whether it is active or not).
    pub state: UserCosmeticState,
    /// The URL to the skin image.
    pub url: Url,
    /// The variant of this skin.
    pub variant: UserSkinVariant,
}

/// A cape is a cosmetic a player can have.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UserCape {
    /// The name of the cape.
    #[serde(rename = "alias")]
    pub name: String,
    /// The UUID of the cape.
    pub id: String,
    /// The state of the cape (whether it is active or not).
    pub state: UserCosmeticState,
    /// The URL to the cape image.
    pub url: Url,
}

/// A game profile contains basic information about the player.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UserProfile {
    /// The UUID of the player.
    pub id: String,
    /// The username of the player.
    #[serde(rename = "name")]
    pub username: String,
    /// The skins of the player.
    pub skins: Box<[UserSkin]>,
    /// The capes of the player.
    pub capes: Box<[UserCape]>,
}

/// Gets a user's profile from a game token.
pub async fn get_profile(client: &Client, token: &str) -> Result<UserProfile> {
    let data = client.get(PROFILE_URL).bearer_auth(token).send().await?;
    let data: UserProfile = data.error_for_status()?.json().await?;
    Ok(data)
}
