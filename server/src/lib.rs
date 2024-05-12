pub mod room_manager;

use crate::proto::room_manager_server::RoomManagerServer;
use crate::room_manager::RoomManager;
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

    pub async fn run(&self) {
        let addr = Self::ADDR.parse().unwrap();
        let room_manager = RoomManagerServer::new(RoomManager::default());
        tracing::info!(message = "Starting gRPC server", ?addr);
        Server::builder()
            .trace_fn(|_| tracing::info_span!("tcpchat_server"))
            .add_service(room_manager)
            .serve(addr)
            .await
            .unwrap();
    }
}

pub mod proto {
    // HACK: The generated code produces some clippy warnings, which
    // are by nature impossible to fix for me, so just silence them.
    #![allow(clippy::pedantic, clippy::nursery)]
    tonic::include_proto!("tcpchat");
}
