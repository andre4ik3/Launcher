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

use std::{future::Future, sync::Arc};

use futures_util::StreamExt;
use reqwest::{IntoUrl, Method, Request, Response};
use serde::Serialize;
use tokio::io::{AsyncWrite, AsyncWriteExt};
use tokio::sync::Mutex;
use tokio::task::JoinHandle;
use tokio::time;

use super::{Error, header::{self, HeaderValue}, queue, Result};

/// Maximum attempts for the client to make a request.
const MAX_ATTEMPTS: u64 = 3;

/// The main public interface of the networking module. Each client has an associated [queue::Queue]
/// that runs in the background to process requests.
pub struct Client {
    queue: Mutex<Option<queue::QueueClient>>,
    handle: Mutex<Option<JoinHandle<()>>>,
}

impl Client {
    /// Creates a new client. A background request queue will be spawned to process requests.
    #[tracing::instrument(name = "net::Client")]
    pub async fn new() -> Self {
        let (queue, handle) = queue::spawn().await;
        Self {
            queue: Mutex::new(Some(queue)),
            handle: Mutex::new(Some(handle)),
        }
    }

    /// Shuts down the client's queue. Requests after this call will return [Error::QueueShutDown].
    /// Returns Some(()) on success, None if the queue has already been shut down. Panics if the
    /// queue thread panicked.
    ///
    /// It is the application's job to call this method when shutting down to fully clean up any
    /// remaining resources.
    pub async fn destroy(&self) -> Option<()> {
        // First wait for both queue and handle to be in our possession
        let mut queue = self.queue.lock().await;
        let mut handle = self.handle.lock().await;

        // Then replace both of them with nothing (dropping them both).
        queue.take()?; // Drop the QueueClient first, this will cause queue to shut down.
        handle.take()?.await.expect("queue panicked"); // Wait for queue to shut down.

        Some(())
    }

    /// Attempts to run a closure 3 times. The closure is expected to return a type of [Result]
    /// (that is, the alias type with [Error] as the error). If a network error ([Error::Network])
    /// is encountered, the given closure is run again. On any other error (e.g.
    /// [Error::QueueShutDown]), all attempts are abandoned and the function returns immediately.
    /// Upon exhaustion of all attempts, [Error::RequestAttemptsExhausted] is returned, containing
    /// the number of attempts tried as well as the last error that occurred within the closure.
    async fn attempt<T, Fut>(mut func: impl FnMut() -> Fut) -> Result<T>
        where
            Fut: Future<Output=Result<T>>,
    {
        let mut last_error = None;

        for attempt in 0..MAX_ATTEMPTS {
            // Delay will be: 0 seconds on 1st attempt, 2 on 2nd, 8 on 3rd
            let delay = attempt.pow(2) * 2;
            tracing::debug!("Attempt {}/{MAX_ATTEMPTS}. Waiting {delay}s.", attempt + 1);
            time::sleep(time::Duration::from_secs(delay)).await;

            // Run the function
            let result = func().await;
            match result {
                Ok(data) => return Ok(data),
                Err(Error::Network(error)) => last_error = Some(Error::from(error)),
                Err(Error::Io(error)) => last_error = Some(Error::from(error)),
                Err(error) => return Err(error),
            };
        }

        Err(Error::RequestAttemptsExhausted(
            MAX_ATTEMPTS,
            Box::new(last_error.unwrap()), // this is safe because otherwise the function would bail early
        ))
    }

    /// Executes a request with retry logic.
    #[tracing::instrument(name = "net::Client::execute", skip_all)]
    pub async fn execute(&self, request: Request) -> Result<Response> {
        let queue = self.queue.lock().await;
        let queue = queue.as_ref().ok_or(Error::QueueShutDown)?;

        Client::attempt(|| async {
            let request = request.try_clone().ok_or(Error::RequestCloneFail)?;
            queue.execute(request).await
        })
            .await
    }

    /// Attempts to download a file to a destination with retry logic and interrupted download
    /// resuming. The destination can be anything that implements [AsyncWrite] and [Unpin].
    #[tracing::instrument(name = "net::Client::download", skip_all)]
    pub async fn download(&self, url: impl IntoUrl, dest: impl AsyncWrite + Unpin) -> Result<()> {
        let queue = self.queue.lock().await;
        let queue = queue.as_ref().ok_or(Error::QueueShutDown)?;

        let request = Request::new(Method::GET, url.into_url()?);

        // these variables are mutable from inside the closure below (they persist between attempts)
        let dest = Arc::new(Mutex::new(dest));
        let length = Arc::new(Mutex::new(0));

        Client::attempt(|| async {
            let mut request = request.try_clone().ok_or(Error::RequestCloneFail)?;
            let mut dest = dest.lock().await;
            let mut length = length.lock().await;

            // Set the range header (download resuming if an attempt fails)
            request
                .headers_mut()
                .insert(header::RANGE, format!("bytes={length}-").parse().unwrap());

            let response = queue.execute(request).await?;
            let skip_length = if response.status() != 206 { *length } else { 0 };
            let mut stream = response.bytes_stream().skip(skip_length);

            while let Some(bytes) = stream.next().await {
                // this will bail on network error
                let bytes = bytes?;
                tracing::trace!("Received chunk of {} bytes", bytes.len());
                *length += dest.write(bytes.as_ref()).await?;
            }

            tracing::debug!("{length} bytes transferred");
            dest.flush().await?;
            Ok(())
        })
            .await
    }

    /// Shorthand for creating a POST request with a form body and using it with [Client::execute].
    pub async fn post_form(&self, url: impl IntoUrl, body: &impl Serialize) -> Result<Response> {
        let mut request = Request::new(Method::POST, url.into_url()?);
        request.headers_mut().insert(header::CONTENT_TYPE, HeaderValue::from_static("application/x-www-form-urlencoded"));
        *request.body_mut() = Some(serde_urlencoded::to_string(body)?.into());
        self.execute(request).await
    }

    /// Shorthand for creating a POST request with a JSON body and using it with [Client::execute].
    pub async fn post_json(&self, url: impl IntoUrl, body: &impl Serialize) -> Result<Response> {
        let mut request = Request::new(Method::POST, url.into_url()?);
        request.headers_mut().insert(header::CONTENT_TYPE, HeaderValue::from_static("application/json"));
        *request.body_mut() = Some(serde_json::to_string(body)?.into());
        self.execute(request).await
    }

    /// Shorthand for creating a GET request and using it with [Client::execute].
    pub async fn get(&self, url: impl IntoUrl) -> Result<Response> {
        let request = Request::new(Method::GET, url.into_url()?);
        self.execute(request).await
    }
}
