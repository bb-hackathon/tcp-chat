#![deny(clippy::unwrap_used)]

pub mod auth;
pub mod entities;
pub mod persistence;
pub mod services;

use crate::auth::Authenticator;
use crate::proto::authentication_tester_server::AuthenticationTesterServer;
use crate::proto::chat_server::ChatServer;
use crate::proto::registry_server::RegistryServer;
use crate::services::{auth_tester::AuthenticationTester, chat::Chat, registry::Registry};
use const_format::formatcp;
use tonic::transport::Server;
use tracing_subscriber::fmt;

#[derive(Debug, Default)]
pub struct TCPChat {}

impl TCPChat {
    const ADDR: &'static str = formatcp!("0.0.0.0:{}", env!("SERVER_RPC_PORT"));

    pub fn preflight() {
        let color_eyre = color_eyre::install().is_ok();
        fmt::Subscriber::builder()
            .with_env_filter("tcp_chat=trace")
            .without_time()
            .pretty()
            .init();
        tracing::debug!(message = "Tracing setup hook finished", %color_eyre);
    }

    #[allow(clippy::missing_panics_doc)]
    pub async fn run(&self) {
        let addr = Self::ADDR
            .parse()
            .expect("Server listen address can't be invalid");

        let registry = Registry::default();
        let user_repo = registry.get_user_repo();
        let registry_service = RegistryServer::new(registry);
        let auth_interceptor = Authenticator::new(user_repo.clone());
        let auth_tester_service = AuthenticationTesterServer::with_interceptor(
            AuthenticationTester::new(),
            auth_interceptor.clone(),
        );
        let chat_service =
            ChatServer::with_interceptor(Chat::new(user_repo.clone()), auth_interceptor);

        tracing::info!(message = "Starting gRPC chat", ?addr);
        Server::builder()
            .trace_fn(|_| tracing::info_span!("tcp_chat"))
            .add_service(registry_service)
            .add_service(chat_service)
            .add_service(auth_tester_service)
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
