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

use reqwest::{Request, Response};
use tokio::sync::Mutex;
use tokio::task::JoinHandle;
use tokio::time;
use tracing::{debug, instrument};

use crate::{queue, Error};

type Result<T> = core::result::Result<T, Error>;

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
    #[instrument(name = "net::Client")]
    pub async fn new() -> Self {
        let (queue, handle) = queue::spawn_queue().await;
        Self {
            queue: Mutex::new(Some(queue)),
            handle: Mutex::new(Some(handle)),
        }
    }

    /// Shuts down the client's queue. Requests after this call will return [Error::QueueShutDown].
    /// Returns Some(()) on success, None if the queue has already been shut down. Panics if the
    /// queue thread panicked.
    pub async fn destroy(&self) -> Option<()> {
        // First wait for both queue and handle to be in our possession
        let mut queue = self.queue.lock().await;
        let mut handle = self.handle.lock().await;

        // Then replace both of them with nothing (dropping them both).
        queue.take()?; // Drop the QueueClient first, this will cause queue to shut down.
        handle.take()?.await.expect("queue panicked"); // Wait for queue to shut down.

        Some(())
    }

    /// Executes a request with retry logic.
    #[instrument(name = "net::Client", skip_all)]
    pub async fn execute(&self, request: Request) -> Result<Response> {
        let queue = self.queue.lock().await;
        let queue = queue.as_ref().ok_or(Error::QueueShutDown)?;
        let mut err = None;

        for attempt in 0..MAX_ATTEMPTS {
            // Delay will be: 0 seconds on 1st attempt, 2 on 2nd, 8 on 3rd
            let delay = attempt.pow(2) * 2;
            debug!("Attempt {}/{MAX_ATTEMPTS}. Waiting {delay}s.", attempt + 1);
            time::sleep(Duration::from_secs(delay)).await;

            // Clone the request (otherwise it would be consumed on each iteration) and send it to
            // the queue using QueueClient.
            let request = request.try_clone().ok_or(Error::RequestCloneFail)?;
            let response = queue.execute(request).await;

            // Retry on network errors, bail on other errors
            match response {
                Ok(response) => return Ok(response),
                Err(Error::Network(error)) => err = Some(error),
                Err(error) => return Err(error),
            };
        }

        Err(Error::RequestAttemptsExhausted(
            MAX_ATTEMPTS,
            err.unwrap(), // this is safe because otherwise the function would bail early
        ))
    }
}
