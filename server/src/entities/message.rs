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

#[cfg(test)]
#[expect(clippy::unwrap_used, reason = "Test module")]
mod tests {
    use super::Message;
    use crate::auth::Authenticator;
    use crate::proto::{self, ClientsideMessage, ServersideMessage};
    use rstest::rstest;

    /// Clients get their own messages echoed back in orded to be displayed,
    /// So each message essentially goes though being a [`ClientsideMessage`],
    /// Then just a [`Message`], and then a [`ServersideMessage`]. This test
    /// ensures no data is lost during these conversions.
    #[rstest]
    #[case::normal("message_text")]
    #[case::empty("")]
    fn echo(#[case] text: String) {
        let uuid = dbg!(uuid::Uuid::new_v4());
        let clientside_message = ClientsideMessage {
            text: text.clone(),
            room_uuid: Some(proto::Uuid { uuid: uuid.into() }),
        };

        let mut request = tonic::Request::new(clientside_message);
        request.metadata_mut().insert(
            Authenticator::USER_UUID_KEY,
            uuid.to_string().parse().unwrap(),
        );

        let message: Message = request.try_into().unwrap();
        assert_eq!(message.sender_uuid, uuid);
        assert_eq!(message.room_uuid, uuid);
        assert_eq!(message.text, text);
        let timestamp = dbg!(message.timestamp);
        let message_uuid = dbg!(message.uuid);

        let serverside_message = ServersideMessage::from(message);
        assert_eq!(
            serverside_message,
            ServersideMessage {
                uuid: Some(message_uuid.into()),
                sender_uuid: Some(uuid.into()),
                room_uuid: Some(uuid.into()),
                timestamp: Some(timestamp.into()),
                text,
            }
        );
    }
}
