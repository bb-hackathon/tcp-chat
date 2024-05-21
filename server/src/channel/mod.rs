//! # A wrapper around a [`mpsc`] channel that detects disconnects.
//!
//! Implements the [`Deref`] trait (`Target = mpsc::Receiver<T>`), and uses a [`oneshot`]
//! channel to send a single message back when the whole thing gets dropped.
//!
//! **Source & further reading:** <https://github.com/hyperium/tonic/issues/377>

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
