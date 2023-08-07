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

use reqwest::{Client, Request, Response};
use tokio::sync::{mpsc, oneshot};
use tokio::time::{interval, Interval};
use tracing::{debug, error, instrument, trace, warn};

use crate::Error;

/// User-agent to be used for outgoing requests.
const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/115.0.0.0 Safari/537.36";

type Result<T> = core::result::Result<T, Error>;

/// Shorthand for the data received in a NetQueue job.
pub type NetQueueJob = (Request, oneshot::Sender<reqwest::Result<Response>>);

/// A queue of network requests. The main method of this struct is [NetQueue::run], which listens
/// for requests to be sent, picks them up from the queue with a rate limit, sends them, then sends
/// the response back.
pub struct NetQueue {
    /// The [Client] of the request queue.
    client: Client,
    /// An interval to rate-limit outgoing requests. One request will be processed every tick.
    interval: Interval,
    /// The receiving channel of the request queue.
    rx: mpsc::Receiver<NetQueueJob>,
}

impl NetQueue {
    /// Creates a new NetQueue that will listen for incoming jobs on the supplied receiver.
    pub fn new(rx: mpsc::Receiver<NetQueueJob>) -> Self {
        Self {
            client: Client::builder().user_agent(USER_AGENT).build().unwrap(),
            interval: interval(Duration::from_secs(1)),
            rx,
        }
    }

    /// Waits for the next job and processes it. Returns `true` if a job was processed, `false` if
    /// the channel is closed. The `cancel` argument is a oneshot receiver that, upon being sent a
    /// value, will cause the queue to be shut down.
    #[instrument(name = "NetQueue", skip_all)]
    pub async fn run(&mut self, mut cancel: oneshot::Receiver<()>) {
        trace!("Running request queue.");
        loop {
            // Wait for ratelimit
            self.interval.tick().await;

            // Check if we've been told to shutdown
            match cancel.try_recv() {
                Ok(()) | Err(oneshot::error::TryRecvError::Closed) => {
                    debug!("Shutdown signal received, shutting down.");
                    return;
                }
                _ => (),
            };

            // Get the next request, or if all transmitters have been dropped, shut down.
            let Some((request, tx)) = self.rx.recv().await else {
                debug!("Queue receive channel closed, shutting down.");
                return;
            };

            // Execute the actual request.
            let result = self.client.execute(request).await;
            let result = result.and_then(Response::error_for_status);

            // Try to send back response, warn in logs if failed.
            if let Err(result) = tx.send(result) {
                warn!("Failed to send back response.");
                match result {
                    Ok(resp) => warn!("Response was Ok: {} {}", resp.status(), resp.url()),
                    Err(err) => warn!("Response was Err: {err}"),
                }
            };
        }
    }
}

/// A NetQueueClient is a simple wrapper around a NetQueue channel to allow for easy sending of
/// requests, mimicking a standard reqwest [Client].
pub struct NetQueueClient(mpsc::Sender<NetQueueJob>);

impl NetQueueClient {
    /// Creates a new NetQueueClient around the specified sender.
    pub fn new(tx: mpsc::Sender<NetQueueJob>) -> Self {
        Self(tx)
    }

    /// Executes a single request (no retry logic). Analogue of [Client::execute].
    #[instrument(name = "NetQueueClient", skip_all)]
    pub async fn execute(&self, request: Request) -> Result<Response> {
        let (tx, rx) = oneshot::channel();

        debug!("--> {} {}", request.method(), request.url());

        // Send request to queue for processing
        if let Err(err) = self.0.send((request, tx)).await {
            warn!("Failed to send request to queue: {err}");
            return Err(Error::QueueShutDown);
        };

        // Wait for queue to send back result
        let Ok(result) = rx.await else {
            warn!("Failed to get response from queue");
            return Err(Error::QueueShutDown);
        };

        // Some pretty logging based on the outcome
        match result {
            Ok(response) => {
                debug!("<-- {} {}", response.status(), response.url());
                Ok(response)
            }
            Err(err) => {
                error!("[!] {err}");
                Err(Error::from(err))
            }
        }
    }
}

/// Creates a new queue and spawns it as a background task, returning the `tx` handle.
pub async fn spawn_queue(cancel: oneshot::Receiver<()>) -> mpsc::Sender<NetQueueJob> {
    trace!("Spawning off-thread NetQueue.");

    // These are the channels used to interact with the background task.
    let (tx, rx) = mpsc::channel(20);

    // Create and spawn the queue.
    let mut queue = NetQueue::new(rx);
    tokio::spawn(async move {
        queue.run(cancel).await;
    });

    // Return the (only!) channel to communicate with the queue. (It can be cloned though)
    tx
}
