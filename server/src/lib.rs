#![deny(clippy::unwrap_used)]

pub mod auth;
pub mod channel;
pub mod entities;
pub mod persistence;
pub mod services;

use crate::auth::Authenticator;
use crate::persistence::create_persistence_pool;
use crate::proto::chat_server::ChatServer;
use crate::proto::registry_server::RegistryServer;
use crate::services::{chat::Chat, registry::Registry};
use std::env;
use tonic::transport::{Identity, Server, ServerTlsConfig};
use tracing_subscriber::fmt;

const CERT: &str = include_str!("../../tls/server.pem");
const KEY: &str = include_str!("../../tls/server.key");

#[derive(Debug, Default)]
pub struct TCPChat {}

impl TCPChat {
    pub fn preflight() {
        let color_eyre = color_eyre::install().is_ok();
        fmt::Subscriber::builder()
            .with_env_filter("tcp_chat=trace")
            .pretty()
            .init();
        tracing::debug!(message = "Tracing setup hook finished", %color_eyre);
    }

    #[allow(clippy::missing_panics_doc)]
    pub async fn run(&self) {
        let port = env::var("SERVER_PORT").expect("$SERVER_PORT should be set");
        let addr = format!("0.0.0.0:{port}")
            .parse()
            .expect("Invalid gRPC listen address");

        // Set up needed external resources and an authenticator.
        let persistence_pool = create_persistence_pool();
        let interceptor = Authenticator::new(persistence_pool.clone());

        // Set up gRPC services.
        let chat = Chat::new(persistence_pool.clone())
            .await
            .expect("Could not initialize a chat instance");
        let chat = ChatServer::with_interceptor(chat, interceptor.clone());
        let registry = Registry::with_persistence_pool(persistence_pool.clone());
        let registry = RegistryServer::new(registry);

        let identity = Identity::from_pem(CERT, KEY);

        tracing::info!(message = "Starting server", ?addr);
        Server::builder()
            .tls_config(ServerTlsConfig::new().identity(identity))
            .expect("The TLS key or certificate is invalid!")
            .trace_fn(|_| tracing::info_span!("server"))
            .add_service(registry)
            .add_service(chat)
            .serve(addr)
            .await
            .expect("The server should've finished successfully");
    }
}

pub mod proto {
    // HACK: The generated code produces some clippy warnings, which
    // are by nature impossible to fix for me, so just silence them.
    #![allow(clippy::pedantic, clippy::nursery, clippy::unwrap_used)]
    tonic::include_proto!("tcp_chat");
}
