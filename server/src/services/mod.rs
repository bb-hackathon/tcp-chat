pub mod auth_tester;
pub mod chat;
pub mod registry;

pub fn acquire_connection_error_status<E: std::error::Error>(error: E) -> tonic::Status {
    let message = "Could not acquire a database connection";
    tracing::error!(message = message, ?error);
    tonic::Status::internal(message)
}
