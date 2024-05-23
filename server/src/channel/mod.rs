//! # DisconnectChannel
//!
//! `DisconnectChannel` is a specialized wrapper around a multi-producer, single-consumer (mpsc) channel,
//! designed specifically for scenarios where detecting disconnections is crucial. It extends the functionality
//! of standard channels by integrating a one-shot channel mechanism to signal disconnection events.
//!
//! ## Purpose
//!
//! The primary goal of `DisconnectChannel` is to facilitate graceful handling of disconnections in asynchronous
//! applications, particularly those involving gRPC communication. By monitoring the underlying mpsc channel
//! for disconnections, it allows for clean shutdowns and error handling mechanisms to be triggered upon
//! disconnection, improving the robustness and reliability of networked services.
//!
//! ## How It Works
//!
//! Internally, `DisconnectChannel` maintains a reference to both the original mpsc receiver and a one-shot sender.
//! When the `DisconnectChannel` instance is dropped, indicating a disconnection event, it sends a single message
//! through the one-shot channel. This mechanism serves as a notification system for other components of the
//! application to react accordingly to the disconnection.
//!
//! ## Example usage
//!
//! ```rust
//! use futures::Stream;
//! use std::task::{Context, Poll};
//! use std::{ops::Deref, pin::Pin};
//! use tokio::sync::{mpsc, oneshot};
//!
//! // Define a new DisconnectChannel.
//! let (grpc_rx, disconnect_tx): (mpsc::Receiver<String>, oneshot::Sender<()>) =
//! mpsc::channel::<String>(10);
//! let disconnect_channel = DisconnectChannel { grpc_rx, disconnect_tx };
//!
//! // Use the DisconnectChannel as a stream.
//! async fn process_messages(disconnect_channel: DisconnectChannel<String>) {
//!     while let Some(message) = disconnect_channel.next().await {
//!         println!("Received message: {}", message);
//!     }
//! }
//!
//! // Upon dropping disconnect_channel, the one-shot channel will be triggered.
//! ```
//!
//! ## Further Reading
//!
//! For more insights and discussions on implementing disconnection detection in Rust asynchronous
//! frameworks, refer to the following GitHub issue:
//! - [Tonic Issue #377](https://github.com/hyperium/tonic/issues/377)

use futures::Stream;
use std::task::{Context, Poll};
use std::{ops::Deref, pin::Pin};
use tokio::sync::{mpsc, oneshot};

pub struct DisconnectChannel<T> {
    pub(crate) disconnect_tx: Option<oneshot::Sender<()>>,
    pub(crate) grpc_rx: mpsc::Receiver<T>,
}

impl<T> Stream for DisconnectChannel<T> {
    type Item = T;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        Pin::new(&mut self.grpc_rx).poll_recv(cx)
    }
}

impl<T> Deref for DisconnectChannel<T> {
    type Target = mpsc::Receiver<T>;

    fn deref(&self) -> &Self::Target {
        &self.grpc_rx
    }
}

impl<T> Drop for DisconnectChannel<T> {
    fn drop(&mut self) {
        if let Some(drop_signal) = self.disconnect_tx.take() {
            let _ = drop_signal.send(());
        }
    }
}
