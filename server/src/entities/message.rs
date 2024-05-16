use super::{ConversionError, Room, User};
use crate::auth::Authenticator;
use crate::proto::{ClientsideMessage, ServersideMessage};
use diesel::prelude::*;
use std::{fmt, str::FromStr, time::SystemTime};
use tonic::{Request, Status};
use uuid::Uuid;

#[derive(Queryable, Identifiable, Selectable, Insertable, Debug, Clone, Associations)]
#[diesel(table_name = crate::entities::schema::messages)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(User, foreign_key = sender_uuid))]
#[diesel(belongs_to(Room, foreign_key = room_uuid))]
#[diesel(primary_key(uuid))]
pub struct Message {
    pub uuid: Uuid,
    pub sender_uuid: Uuid,
    pub room_uuid: Uuid,
    pub text: String,
    pub timestamp: SystemTime,
}

impl Message {
    pub fn new<T: Into<String>>(text: T, sender_uuid: Uuid, room_uuid: Uuid) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            sender_uuid,
            room_uuid,
            text: text.into(),
            timestamp: SystemTime::now(),
        }
    }

    pub fn from_clientside_message(
        msg: ClientsideMessage,
        sender_uuid: Uuid,
    ) -> Result<Self, ConversionError> {
        Ok(Self {
            uuid: Uuid::new_v4(),
            sender_uuid,
            room_uuid: msg
                .room_uuid
                .and_then(|u| u.try_into().ok())
                .ok_or(ConversionError::MissingField)?,
            text: msg.text,
            timestamp: SystemTime::now(),
        })
    }
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.sender_uuid, self.text)?;
        Ok(())
    }
}

impl TryFrom<Request<ClientsideMessage>> for Message {
    type Error = Status;

    fn try_from(request: Request<ClientsideMessage>) -> Result<Self, Self::Error> {
        let invalid_uuid_msg = "The sender's UUID is invalid or missing from request's metadata";
        let user_uuid: Uuid = request
            .metadata()
            .get(Authenticator::USER_UUID_KEY)
            .and_then(|mv| mv.to_str().ok())
            .and_then(|s| Uuid::from_str(s).ok())
            .ok_or(Status::invalid_argument(invalid_uuid_msg))?;

        let msg = request.into_inner();
        Message::from_clientside_message(msg, user_uuid)
            .map_err(|err| Status::invalid_argument(err.to_string()))
    }
}

impl From<Message> for ServersideMessage {
    fn from(msg: Message) -> Self {
        Self {
            uuid: Some(msg.uuid.into()),
            sender_uuid: Some(msg.sender_uuid.into()),
            room_uuid: Some(msg.room_uuid.into()),
            text: msg.text,
            timestamp: Some(msg.timestamp.into()),
        }
    }
}
