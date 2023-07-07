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

use std::time::Duration;

use anyhow::{anyhow, bail, Result};
use reqwest::{Client, Request, Response};
use tokio::time::sleep;

/// Maximum amount of attempts to run for a request.
const MAX_ATTEMPTS: u8 = 3;

/// Delay between requests in seconds. For every attempt, it is multiplied by itself.
const DELAY: u64 = 2;

enum RequestResult {
    /// Successful response (2XX code).
    Success(Response),
    /// Response that failed due to the server (5XX code).
    ServerError(Response),
    /// Response that failed due to a client error (4XX code).
    ClientError(Response),
    /// Response that failed due to a network error.
    NetworkError(reqwest::Error),
}

/// Actually tries to execute a request. Returns `Some(Response)` if the response is either
/// successful or failed with a recoverable error. Returns `None` on unrecoverable error.
async fn run_request(client: &Client, request: Request) -> RequestResult {
    match client.execute(request).await {
        Ok(resp) => {
            let status = resp.status();
            if status.is_success() {
                RequestResult::Success(resp)
            } else if status.is_server_error() {
                RequestResult::ServerError(resp)
            } else {
                RequestResult::ClientError(resp)
            }
        }
        Err(err) => RequestResult::NetworkError(err),
    }
}

/// Tries to run a request with built-in retry logic.
pub async fn try_request(client: &Client, request: Request) -> Result<Response> {
    for attempt in 0..MAX_ATTEMPTS {
        let request = request
            .try_clone()
            .ok_or_else(|| anyhow!("Failed to clone request"))?;

        let delay = DELAY.pow(attempt as u32);
        let result = run_request(client, request).await;
        match result {
            RequestResult::Success(resp) => return Ok(resp),
            RequestResult::ClientError(resp) => bail!("Request error: {}", resp.status()),
            _ => sleep(Duration::from_secs(delay)).await,
        }
    }

    bail!(
        "Giving up on {} after {MAX_ATTEMPTS} attempts",
        request.url().as_str()
    )
}
