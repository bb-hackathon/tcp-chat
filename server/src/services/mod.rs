pub mod auth_tester;
pub mod chat;
pub mod registry;

pub fn acquire_connection_error<E: std::error::Error>(error: E) -> String {
    format!("Error when acquiring a database connection: {error}")
}

pub fn acquire_connection_error_status<E: std::error::Error>(error: E) -> tonic::Status {
    let msg = format!("Error when acquiring a database connection: {error}");
    tonic::Status::internal(msg)
}
