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

//! Launcher Networking Module
//! ==========================
//!
//! This module contains all network-related functionality. This crate is the only one that depends
//! on the [reqwest] crate, providing safe wrappers around it that add a request queue, retry logic,
//! download resuming, and some generally nice utilities.

use std::io;

pub use reqwest::{
    Body, Certificate, dns, header, Identity, Method, NoProxy, Proxy, Request, Response, StatusCode,
};
use thiserror::Error;

pub use client::Client;

mod client;
mod queue;

#[derive(Debug, Error)]
pub enum Error {
    #[error("network error: {0}")]
    Network(#[from] reqwest::Error),
    #[error("io error: {0}")]
    Io(#[from] io::Error),
    #[error("request failed after {0} attempts, last error: {1}")]
    RequestAttemptsExhausted(u64, Box<Error>),
    #[error("request cannot be cloned (required for retrying)")]
    RequestCloneFail,
    #[error("queue has been shut down")]
    QueueShutDown,
    #[error("failed to serialize form body: {0}")]
    FormSerializationFailure(#[from] serde_urlencoded::ser::Error),
    #[error("failed to serialize json body: {0}")]
    JsonSerializationFailure(#[from] serde_json::Error),
}

pub type Result<T> = core::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_URL: &str = "https://httpbingo.org/base64/dGVzdGluZyBzdWNjZXNzZnVsCg==";
    const TEST_RESPONSE: &str = "testing successful\n";

    #[tokio::test]
    async fn client() -> Result<()> {
        let client = Client::new().await;

        // Try a basic request
        let res = client.get(TEST_URL).await?;
        assert_eq!(res.status(), StatusCode::OK);
        assert_eq!(res.text().await?, TEST_RESPONSE);

        // Try the same thing but using download method
        let mut buf = Vec::<u8>::new();
        client.download(TEST_URL, &mut buf).await?;

        assert_eq!(buf.len(), TEST_RESPONSE.len());
        assert_eq!(String::from_utf8(buf), Ok(TEST_RESPONSE.to_string()));

        // Check that the client can destroy properly
        assert_eq!(client.destroy().await, Some(()));
        assert_eq!(client.destroy().await, None); // destroying twice should be a no-op

        // Check that trying to run requests now returns an error
        let Err(Error::QueueShutDown) = client.get(TEST_URL).await else {
            panic!("Client::execute returned with the wrong error (or no error at all!)");
        };

        let Err(Error::QueueShutDown) = client.download(TEST_URL, &mut Vec::new()).await else {
            panic!("Client::download returned with the wrong error (or no error at all!)");
        };

        Ok(())
    }

    #[tokio::test]
    async fn queue() -> Result<()> {
        let (queue, handle) = queue::spawn().await;

        // Try a basic request
        let req = Request::new(Method::GET, TEST_URL.parse().unwrap());
        let res = queue.execute(req).await?;
        assert_eq!(res.text().await?, TEST_RESPONSE);

        // Manually join the queue
        drop(queue);
        handle.await.expect("queue thread panicked!");
        Ok(())
    }
}
