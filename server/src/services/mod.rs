pub mod chat;
pub mod registry;

use std::error::Error;

pub fn acquire_connection_error_status<E: Error>(error: E) -> tonic::Status {
    let message = "Could not acquire a connection";
    tracing::error!(message = message, ?error);
    tonic::Status::internal(message)
}
