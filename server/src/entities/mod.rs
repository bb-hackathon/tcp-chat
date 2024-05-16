pub mod schema;

pub mod message;
pub mod relations;
pub mod room;
pub mod user;

pub use message::Message;
pub use relations::RoomUser;
pub use room::Room;
pub use user::User;

#[derive(thiserror::Error, Debug)]
pub enum ConversionError {
    #[error("The protobuf entity is missing a required field")]
    MissingField,
}
