mod entities;
pub mod registry;
mod token;
mod uuid;

use crate::{proto::registry_server::RegistryServer, registry::Registry};
use const_format::formatcp;
use tonic::transport::Server;
use tracing_subscriber::fmt;

#[derive(Debug, Default)]
pub struct TCPChat {}

impl TCPChat {
    const ADDR: &'static str = formatcp!("0.0.0.0:{}", env!("SERVER_RPC_PORT"));

    pub fn preflight() {
        let color_eyre = color_eyre::install().is_ok();
        fmt::Subscriber::builder().without_time().pretty().init();
        tracing::debug!(message = "Tracing setup hook finished", %color_eyre);
    }

    #[allow(clippy::missing_panics_doc)]
    pub async fn run(&self) {
        let addr = Self::ADDR.parse().unwrap();
        tracing::info!(message = "Starting gRPC chat", ?addr);
        Server::builder()
            .trace_fn(|_| tracing::info_span!("tcp_chat"))
            .add_service(RegistryServer::new(Registry::default()))
            .serve(addr)
            .await
            .unwrap();
    }
}

pub mod proto {
    // HACK: The generated code produces some clippy warnings, which
    // are by nature impossible to fix for me, so just silence them.
    #![allow(clippy::pedantic, clippy::nursery)]
    tonic::include_proto!("tcp_chat");
}
