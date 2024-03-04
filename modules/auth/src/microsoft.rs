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

use async_trait::async_trait;
use chrono::{DateTime, Duration, Utc};
use url::Url;
use uuid::Uuid;

use data::auth::{Account, AccountCredentials};
use data::web::microsoft::*;
use fetch::mojang::get_profile;
use net::{Client, Request};

use super::{AuthenticationMethod, Error, Result};

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

pub struct MicrosoftAuthenticationService;

#[async_trait]
impl AuthenticationMethod for MicrosoftAuthenticationService {
    type Credentials = String;

    #[tracing::instrument(skip_all)]
    async fn authenticate(client: &Client, credentials: Self::Credentials) -> Result<Account> {
        tracing::debug!("Beginning authentication of Microsoft account.");

        let (access_token, refresh_token) = exchange(client, credentials).await?;
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

    #[tracing::instrument(skip_all)]
    async fn refresh(client: &Client, account: Account) -> Result<Account> {
        tracing::debug!("Beginning refresh of Microsoft account {}.", account.username);
        
        // Get a new access token and refresh token.
        let refresh_token = match account.credentials {
            AccountCredentials::Microsoft { refresh, .. } => refresh,
            AccountCredentials::Offline => return Err(Error::WrongAccountType("microsoft", "offline")),
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

    let data: AuthCodeExchangeResponse = client.post_form(AUTH_MS_TOKEN_URL, &params).await?.json().await.map_err(net::Error::from)?;
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

    let data: AuthCodeExchangeResponse = client.post_form(AUTH_MS_TOKEN_URL, &params).await?.json().await.map_err(net::Error::from)?;
    Ok((data.access_token, data.refresh_token))
}

/// MS -> XBL -> XSTS -> Game Token (and when it expires)
async fn authenticate(client: &Client, token: String) -> Result<(String, DateTime<Utc>)> {
    // === MS -> XBL ===
    let body = AuthXboxTokenRequest {
        properties: AuthXboxLiveTokenRequestProperties {
            auth_method: "RPS",
            site_name: "user.auth.xboxlive.com",
            rps_ticket: token,
        },
        relying_party: AUTH_RP_XBOX,
        token_type: "JWT",
    };

    let data: AuthXboxTokenResponse = client.post_json(AUTH_XBL_TOKEN_URL, &body).await?.json().await.map_err(net::Error::from)?;
    let xbl = data.token;

    // === XBL -> XSTS ===
    let body = AuthXboxTokenRequest {
        properties: AuthXboxSecureTokenRequestProperties {
            sandbox_id: "RETAIL",
            user_tokens: [xbl],
        },
        relying_party: AUTH_RP_GAME,
        token_type: "JWT",
    };

    let data: AuthXboxTokenResponse = client.post_json(AUTH_XSTS_TOKEN_URL, &body).await?.json().await.map_err(net::Error::from)?;
    let xsts = data.token;
    let uhs = data
        .display_claims
        .get("xui")
        .and_then(|x| x.first())
        .and_then(|x| x.get("uhs"))
        .ok_or(Error::DecodingError)?;

    // === XSTS -> Game ===
    let body = AuthGameTokenRequest {
        platform: "PC_LAUNCHER",
        xtoken: format!("XBL3.0 x={};{}", uhs, xsts),
    };

    let data: AuthGameTokenResponse = client.post_json(AUTH_GAME_TOKEN_URL, &body).await?.json().await.map_err(net::Error::from)?;
    let expires = Utc::now() + Duration::seconds(data.expires_in);
    Ok((data.access_token, expires))
}
