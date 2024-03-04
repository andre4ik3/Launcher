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

use std::collections::HashMap;
use macros::{api_request, api_response};

// Web URLs
pub const AUTH_LOG_IN_URL: &str = "https://login.live.com/oauth20_authorize.srf";
pub const AUTH_REDIRECT_URL: &str = "https://login.live.com/oauth20_desktop.srf";

// API endpoints
pub const AUTH_MS_TOKEN_URL: &str = "https://login.live.com/oauth20_token.srf";
pub const AUTH_XBL_TOKEN_URL: &str = "https://user.auth.xboxlive.com/user/authenticate";
pub const AUTH_XSTS_TOKEN_URL: &str = "https://xsts.auth.xboxlive.com/xsts/authorize";
pub const AUTH_GAME_TOKEN_URL: &str = "https://api.minecraftservices.com/launcher/login";

// MS token exchange properties
pub const AUTH_CLIENT_ID: &str = "00000000402B5328";
pub const AUTH_SCOPE: &str = "service::user.auth.xboxlive.com::MBI_SSL";

// Xbox relying parties
pub const AUTH_RP_XBOX: &str = "http://auth.xboxlive.com";
pub const AUTH_RP_GAME: &str = "rp://api.minecraftservices.com/";

#[api_response]
pub struct AuthCodeExchangeResponse {
    pub token_type: String,
    pub expires_in: u64,
    pub scope: String,
    pub access_token: String,
    pub refresh_token: String,
    pub user_id: String,
    pub foci: String,
}

#[api_request(rename = "PascalCase")]
pub struct AuthXboxLiveTokenRequestProperties {
    pub auth_method: &'static str,
    pub site_name: &'static str,
    pub rps_ticket: String,
}

#[api_request(rename = "PascalCase")]
pub struct AuthXboxSecureTokenRequestProperties {
    pub sandbox_id: &'static str,
    pub user_tokens: [String; 1],
}

#[api_request(rename = "PascalCase")]
pub struct AuthXboxTokenRequest<T> {
    pub properties: T,
    pub relying_party: &'static str,
    pub token_type: &'static str,
}

#[api_response(rename = "PascalCase")]
pub struct AuthXboxTokenResponse {
    pub issue_instant: String,
    pub not_after: String,
    pub token: String,
    pub display_claims: HashMap<String, Vec<HashMap<String, String>>>,
}

#[api_request(rename = "PascalCase")]
pub struct AuthGameTokenRequest {
    pub platform: &'static str,
    pub xtoken: String,
}

#[api_response]
pub struct AuthGameTokenResponse {
    pub access_token: String,
    pub expires_in: i64,
}
