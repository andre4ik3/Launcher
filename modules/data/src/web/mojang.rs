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

use url::Url;

use macros::api_response;

pub const PROFILE_URL: &str = "https://api.minecraftservices.com/minecraft/profile";

#[api_response(untagged = false, rename = "SCREAMING_SNAKE_CASE")]
pub enum UserCosmeticState {
    Inactive,
    Active,
}

#[api_response(untagged = false, rename = "SCREAMING_SNAKE_CASE")]
pub enum UserSkinVariant {
    Classic,
    Slim,
}

#[api_response(strict = false)]
pub struct UserSkin {
    pub id: String,
    pub state: UserCosmeticState,
    pub url: Url,
    pub variant: UserSkinVariant,
}

#[api_response(strict = false)]
pub struct UserCape {
    #[serde(rename = "alias")]
    pub name: String,
    pub id: String,
    pub state: UserCosmeticState,
    pub url: Url,
}

#[api_response(strict = false)]
pub struct UserProfile {
    pub id: String,
    #[serde(rename = "name")]
    pub username: String,
    pub skins: Box<[UserSkin]>,
    pub capes: Box<[UserCape]>,
}
