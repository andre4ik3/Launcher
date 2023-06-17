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
use std::collections::HashMap;
use std::fmt::Display;
use url::Url;

const AUTH_LOG_IN_URL: &str = "https://login.live.com/oauth20_authorize.srf";
const AUTH_REDIRECT_URL: &str = "https://login.live.com/oauth20_desktop.srf";

const AUTH_MS_TOKEN_URL: &str = "https://login.live.com/oauth20_token.srf";
const AUTH_XBL_TOKEN_URL: &str = "https://user.auth.xboxlive.com/user/authenticate";
const AUTH_XSTS_TOKEN_URL: &str = "https://xsts.auth.xboxlive.com/xsts/authorize";
const AUTH_GAME_TOKEN_URL: &str = "https://api.minecraftservices.com/launcher/login";
const AUTH_PROFILE_URL: &str = "https://api.minecraftservices.com/minecraft/profile";

const AUTH_CLIENT_ID: &str = "00000000402B5328";
const AUTH_SCOPE: &str = "service::user.auth.xboxlive.com::MBI_SSL";

const AUTH_RP_XBOX: &str = "http://auth.xboxlive.com";
const AUTH_RP_GAME: &str = "rp://api.minecraftservices.com/";

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct AuthCodeExchangeResponse {
    token_type: String,
    expires_in: u64,
    scope: String,
    access_token: String,
    refresh_token: String,
    user_id: String,
    foci: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct _AuthXboxLiveTokenRequestProperties {
    auth_method: &'static str,
    site_name: &'static str,
    rps_ticket: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct _AuthXboxSecureTokenRequestProperties {
    sandbox_id: &'static str,
    user_tokens: [String; 1],
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct AuthXboxTokenRequest<T> {
    properties: T,
    relying_party: &'static str,
    token_type: &'static str,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
struct AuthXboxTokenResponse {
    issue_instant: String,
    not_after: String,
    token: String,
    display_claims: HashMap<String, Vec<HashMap<String, String>>>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct AuthGameTokenRequest {
    platform: &'static str,
    xtoken: String,
}

#[derive(Debug, Deserialize, Serialize)]
// #[serde(deny_unknown_fields)] roles = {} metadata = {}
struct AuthGameTokenResponse {
    username: String,
    access_token: String,
    token_type: String,
    expires_in: u64,
}

#[derive(Debug, Deserialize, Serialize)]
struct AuthGameProfileResponse {
    id: String,
    name: String,
    // TODO
}

/// Builds a URL that can be opened in a WebView to redirect to a URL with a ?code= parameter.
pub fn get_auth_url() -> Url {
    let params = [
        ("client_id", AUTH_CLIENT_ID),
        ("prompt", "select_account"),
        ("redirect_uri", AUTH_REDIRECT_URL),
        ("response_type", "code"),
        ("scope", AUTH_SCOPE),
    ];
    Url::parse_with_params(AUTH_LOG_IN_URL, params).unwrap()
}

/// Exchanges the code received from the Microsoft login webpage to an access and refresh token.
pub async fn get_ms_token(
    client: &Client,
    code: impl AsRef<str> + Display,
) -> Result<(String, String)> {
    let params = [
        ("client_id", AUTH_CLIENT_ID),
        ("code", code.as_ref()),
        ("grant_type", "authorization_code"),
        ("redirect_uri", AUTH_REDIRECT_URL),
        ("scope", AUTH_SCOPE),
    ];

    let data = client.post(AUTH_MS_TOKEN_URL).form(&params).send().await?;
    let data: AuthCodeExchangeResponse = data.error_for_status()?.json().await?;
    Ok((data.access_token, data.refresh_token))
}

/// Refreshes an expired Microsoft token for another one.
pub async fn refresh_ms_token(
    client: &Client,
    token: impl AsRef<str> + Display,
) -> Result<(String, String)> {
    todo!()
}

/// Exchanges a Microsoft access token for an Xbox Live token.
pub async fn get_xbl_token(client: &Client, token: impl AsRef<str> + Display) -> Result<String> {
    let body = AuthXboxTokenRequest {
        properties: _AuthXboxLiveTokenRequestProperties {
            auth_method: "RPS",
            site_name: "user.auth.xboxlive.com",
            rps_ticket: token.as_ref().to_string(),
        },
        relying_party: AUTH_RP_XBOX,
        token_type: "JWT",
    };

    let data = client.post(AUTH_XBL_TOKEN_URL).json(&body).send().await?;
    let data: AuthXboxTokenResponse = data.error_for_status()?.json().await?;
    Ok(data.token)
}

/// Exchanges an Xbox Live token for an XSTS (Xbox Secure Token Service) token and user hash.
pub async fn get_xsts_token(
    client: &Client,
    token: impl AsRef<str> + Display,
) -> Result<(String, String)> {
    let body = AuthXboxTokenRequest {
        properties: _AuthXboxSecureTokenRequestProperties {
            sandbox_id: "RETAIL",
            user_tokens: [token.as_ref().to_string()],
        },
        relying_party: AUTH_RP_GAME,
        token_type: "JWT",
    };

    let data = client.post(AUTH_XSTS_TOKEN_URL).json(&body).send().await?;
    let data: AuthXboxTokenResponse = data.error_for_status()?.json().await?;

    // TODO: total hack, should rather enforce shape of XUI e.g. using #[serde(flatten)]
    let uhs = data
        .display_claims
        .get("xui")
        .unwrap()
        .first()
        .unwrap()
        .get("uhs")
        .unwrap();

    Ok((data.token, uhs.to_string()))
}

/// Exchanges an XSTS token for a game token.
pub async fn get_game_token(
    client: &Client,
    uhs: impl AsRef<str> + Display,
    xsts: impl AsRef<str> + Display,
) -> Result<String> {
    let body = AuthGameTokenRequest {
        platform: "PC_LAUNCHER",
        xtoken: format!("XBL3.0 x={};{}", uhs, xsts),
    };

    let data = client.post(AUTH_GAME_TOKEN_URL).json(&body).send().await?;
    let data: AuthGameTokenResponse = data.error_for_status()?.json().await?;
    Ok(data.access_token)
}

/// Returns some metadata about the account.
pub async fn get_profile(client: &Client, token: impl AsRef<str> + Display) -> Result<()> {
    let data = client
        .get(AUTH_PROFILE_URL)
        .bearer_auth(token)
        .send()
        .await?;

    // TODO json
    let data = data.text().await?;
    println!("Profile: {}", data);

    todo!()
}
