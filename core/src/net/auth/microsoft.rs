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

use anyhow::{anyhow, bail, Result};
use async_trait::async_trait;
use chrono::{DateTime, Duration, Utc};
use reqwest::Client;
use url::Url;
use uuid::Uuid;

use crate::models::{Account, AccountCredentials};
use crate::net::auth::AuthenticationService;
use crate::net::mojang::get_profile;
use crate::utils::try_request;

// Web URLs
const AUTH_LOG_IN_URL: &str = "https://login.live.com/oauth20_authorize.srf";
const AUTH_REDIRECT_URL: &str = "https://login.live.com/oauth20_desktop.srf";

// API endpoints
const AUTH_MS_TOKEN_URL: &str = "https://login.live.com/oauth20_token.srf";
const AUTH_XBL_TOKEN_URL: &str = "https://user.auth.xboxlive.com/user/authenticate";
const AUTH_XSTS_TOKEN_URL: &str = "https://xsts.auth.xboxlive.com/xsts/authorize";
const AUTH_GAME_TOKEN_URL: &str = "https://api.minecraftservices.com/launcher/login";

// MS token exchange properties
const AUTH_CLIENT_ID: &str = "00000000402B5328";
const AUTH_SCOPE: &str = "service::user.auth.xboxlive.com::MBI_SSL";

// Xbox relying parties
const AUTH_RP_XBOX: &str = "http://auth.xboxlive.com";
const AUTH_RP_GAME: &str = "rp://api.minecraftservices.com/";

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

mod data {
    use std::collections::HashMap;

    use serde::{Deserialize, Serialize};

    #[derive(Debug, Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct AuthCodeExchangeResponse {
        pub token_type: String,
        pub expires_in: u64,
        pub scope: String,
        pub access_token: String,
        pub refresh_token: String,
        pub user_id: String,
        pub foci: String,
    }

    #[derive(Debug, Serialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct AuthXboxLiveTokenRequestProperties {
        pub auth_method: &'static str,
        pub site_name: &'static str,
        pub rps_ticket: String,
    }

    #[derive(Debug, Serialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct AuthXboxSecureTokenRequestProperties {
        pub sandbox_id: &'static str,
        pub user_tokens: [String; 1],
    }

    #[derive(Debug, Serialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct AuthXboxTokenRequest<T> {
        pub properties: T,
        pub relying_party: &'static str,
        pub token_type: &'static str,
    }

    #[derive(Debug, Deserialize)]
    #[serde(deny_unknown_fields, rename_all = "PascalCase")]
    pub struct AuthXboxTokenResponse {
        pub issue_instant: String,
        pub not_after: String,
        pub token: String,
        pub display_claims: HashMap<String, Vec<HashMap<String, String>>>,
    }

    #[derive(Debug, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct AuthGameTokenRequest {
        pub platform: &'static str,
        pub xtoken: String,
    }

    #[derive(Debug, Deserialize)]
    pub struct AuthGameTokenResponse {
        pub access_token: String,
        pub expires_in: i64,
    }
}

pub struct MicrosoftAuthenticationService;

#[async_trait]
impl AuthenticationService<String> for MicrosoftAuthenticationService {
    async fn authenticate(client: &Client, code: String) -> Result<Account> {
        let (access_token, refresh_token) = exchange(client, code).await?;
        let (token, expires) = authenticate(client, access_token.clone()).await?;

        let profile = get_profile(client, &token).await;
        let has_profile = profile.is_ok();
        let (id, username) = profile
            .map(|p| (p.id, p.username))
            .unwrap_or_else(|_| (Uuid::new_v4().to_string(), "Player".to_string()));

        Ok(Account {
            id,
            username,
            has_profile,
            token,
            expires: Some(expires),
            credentials: AccountCredentials::Microsoft {
                access: access_token,
                refresh: refresh_token,
            },
        })
    }

    async fn refresh(client: &Client, account: Account) -> Result<Account> {
        // Get a new access token and refresh token.
        let refresh_token = match account.credentials {
            AccountCredentials::Microsoft { refresh, .. } => refresh,
            AccountCredentials::Offline => bail!("Wrong account type"),
        };

        let (access_token, refresh_token) = refresh(client, refresh_token).await?;
        let (token, expires) = authenticate(client, access_token.clone()).await?;

        let profile = get_profile(client, &token).await;
        let has_profile = profile.is_ok();
        let (id, username) = profile
            .map(|p| (p.id, p.username))
            .unwrap_or_else(|_| (account.id, account.username));

        Ok(Account {
            id,
            username,
            has_profile,
            token,
            expires: Some(expires),
            credentials: AccountCredentials::Microsoft {
                access: access_token,
                refresh: refresh_token,
            },
        })
    }
}

/// Exchanges the code received from the Microsoft login webpage to an access and refresh token.
async fn exchange(client: &Client, code: String) -> Result<(String, String)> {
    let params = [
        ("client_id", AUTH_CLIENT_ID),
        ("code", &code),
        ("grant_type", "authorization_code"),
        ("redirect_uri", AUTH_REDIRECT_URL),
        ("scope", AUTH_SCOPE),
    ];

    let request = client.post(AUTH_MS_TOKEN_URL).form(&params).build()?;
    let data = try_request(client, request).await?;
    let data: data::AuthCodeExchangeResponse = data.json().await?;
    Ok((data.access_token, data.refresh_token))
}

/// Refreshes an expired Microsoft token for another one.
async fn refresh(client: &Client, refresh_token: String) -> Result<(String, String)> {
    let params = [
        ("client_id", AUTH_CLIENT_ID),
        ("grant_type", "refresh_token"),
        ("refresh_token", &refresh_token),
        ("scope", AUTH_SCOPE),
    ];

    let request = client.post(AUTH_MS_TOKEN_URL).form(&params).build()?;
    let data = try_request(client, request).await?;
    let data: data::AuthCodeExchangeResponse = data.json().await?;
    Ok((data.access_token, data.refresh_token))
}

/// MS -> XBL -> XSTS -> Game Token (and when it expires)
async fn authenticate(client: &Client, token: String) -> Result<(String, DateTime<Utc>)> {
    // MS -> XBL
    let body = data::AuthXboxTokenRequest {
        properties: data::AuthXboxLiveTokenRequestProperties {
            auth_method: "RPS",
            site_name: "user.auth.xboxlive.com",
            rps_ticket: token,
        },
        relying_party: AUTH_RP_XBOX,
        token_type: "JWT",
    };

    let request = client.post(AUTH_XBL_TOKEN_URL).json(&body).build()?;
    let data = try_request(client, request).await?;
    let data: data::AuthXboxTokenResponse = data.json().await?;
    let xbl = data.token;

    // XBL -> XSTS
    let body = data::AuthXboxTokenRequest {
        properties: data::AuthXboxSecureTokenRequestProperties {
            sandbox_id: "RETAIL",
            user_tokens: [xbl],
        },
        relying_party: AUTH_RP_GAME,
        token_type: "JWT",
    };

    let request = client.post(AUTH_XSTS_TOKEN_URL).json(&body).build()?;
    let data = try_request(client, request).await?;
    let data: data::AuthXboxTokenResponse = data.json().await?;

    let xsts = data.token;
    let uhs = data
        .display_claims
        .get("xui")
        .ok_or_else(|| anyhow!("No XUI in display_claims"))?
        .first()
        .ok_or_else(|| anyhow!("No data in XUI display_claims"))?
        .get("uhs")
        .ok_or_else(|| anyhow!("No UHS"))?;

    // XSTS -> Game
    let body = data::AuthGameTokenRequest {
        platform: "PC_LAUNCHER",
        xtoken: format!("XBL3.0 x={};{}", uhs, xsts),
    };

    let request = client.post(AUTH_GAME_TOKEN_URL).json(&body).build()?;
    let data = try_request(client, request).await?;
    let data: data::AuthGameTokenResponse = data.json().await?;

    let expires = Utc::now() + Duration::seconds(data.expires_in);
    Ok((data.access_token, expires))
}
