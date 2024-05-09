use crate::proto::greeter_server::{Greeter, GreeterServer};
use crate::proto::{HelloRequest, HelloResponse};
use const_format::formatcp;
use tonic::{transport::Server, Request, Response};

/// A ZST that implements the `gRPC` service.
///
/// An instance of this type is passed to the tonic with the meaning:
/// "I implement this `gRPC` service and can handle it, let me handle
/// all requests that are related to that service"
#[derive(Debug, Default)]
pub struct TCPChat {}

// NOTE: Here is where we teach the handler how to
// handle each of the `RPC` requests it may receive.
#[tonic::async_trait]
impl Greeter for TCPChat {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloResponse>, tonic::Status> {
        dbg!(request);
        Ok(Response::new(HelloResponse {
            message: "hey!".into(),
        }))
    }
}

/// The address of our `gRPC` service.
const ADDR: &str = formatcp!("0.0.0.0:{}", env!("SERVER_RPC_PORT"));

#[tokio::main]
async fn main() {
    let _ = color_eyre::install();
    let addr = ADDR.parse().unwrap();
    println!("gRPC running on {addr}");
    let greeter = TCPChat::default();
    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await
        .unwrap();
}

pub mod proto {
    // HACK: The generated code produces some clippy warnings, which
    // are by nature impossible to fix for me, so just silence them.
    #![allow(clippy::pedantic, clippy::nursery)]
    tonic::include_proto!("tcpchat");
}
