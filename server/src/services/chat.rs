use crate::auth::AuthenticatedRequest;
use crate::entities::{Message, Room, RoomUser, User};
use crate::persistence::ConnectionPool;
use crate::proto::user_lookup_request::Identifier;
use crate::proto::{self, RoomWithUserCreationRequest, UserLookupRequest};
use crate::proto::{ClientsideMessage, ClientsideRoom, ServersideRoomEvent, ServersideUserEvent};
use crate::services::acquire_connection_error_status;
use std::pin::Pin;
use tokio_stream::Stream;
use tonic::{Request, Response, Status};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Chat {
    connection_pool: ConnectionPool,
}

impl Chat {
    pub fn new(connection_pool: ConnectionPool) -> Self {
        Self { connection_pool }
    }
}

type RPCStream<T> = Pin<Box<dyn Stream<Item = Result<T, Status>> + Send + 'static>>;

#[tonic::async_trait]
impl proto::chat_server::Chat for Chat {
    #[tracing::instrument(skip(self))]
    async fn lookup_user(
        &self,
        request: Request<UserLookupRequest>,
    ) -> Result<Response<proto::User>, Status> {
        let identifier: Identifier =
            request
                .into_inner()
                .identifier
                .ok_or(Status::invalid_argument(
                    "Can't lookup user without an identifier",
                ))?;

        let mut connection = self
            .connection_pool
            .get()
            .map_err(acquire_connection_error_status)?;

        // Import some traits and methods to interact with the ORM.
        use crate::entities::schema::users::dsl::*;
        use diesel::prelude::*;

        let found_user: Option<User> = match identifier.clone() {
            Identifier::Uuid(proto_uuid) => users
                .filter(uuid.eq::<Uuid>(Uuid::try_from(proto_uuid).unwrap()))
                .select(User::as_select())
                .first(&mut connection),
            Identifier::Username(proto_uname) => users
                .filter(username.eq(proto_uname))
                .select(User::as_select())
                .first(&mut connection),
        }
        .optional()
        .map_err(|err| Status::internal(err.to_string()))?;

        match found_user {
            Some(user) => {
                tracing::debug!(message = "Successful user lookup", username = ?user.username, uuid = ?user.uuid);
                Ok(Response::new(proto::User::from(user.clone())))
            }
            None => {
                tracing::debug!(message = "Unsuccessful user lookup", ?identifier);
                Err(Status::not_found("No user with such username"))
            }
        }
    }

    #[tracing::instrument(skip(self))]
    async fn send_message(
        &self,
        request: Request<ClientsideMessage>,
    ) -> Result<Response<()>, Status> {
        let message = Message::try_from(request)?;

        let mut conn = self
            .connection_pool
            .get()
            .map_err(acquire_connection_error_status)?;

        // Ensure the user isn't sending a message to a room he's not a member of.
        {
            use crate::entities::schema::rooms_users::dsl::*;
            use diesel::prelude::*;
            let _membership: RoomUser = rooms_users
                .find((message.room_uuid, message.sender_uuid))
                .select(RoomUser::as_select())
                .first(&mut conn)
                .map_err(|_| {
                    let msg = "User tried to send a message to a room he's not a member of";
                    tracing::warn!(message = msg, ?message);
                    Status::permission_denied("You're not a member of this room!")
                })?;
        }

        // Store the message in the database and TODO: mirror it to all subscribers.
        {
            use crate::entities::schema::messages::dsl::*;
            use diesel::prelude::*;
            let _ = diesel::insert_into(messages)
                .values(&message)
                .execute(&mut conn)
                .map_err(|err| {
                    tracing::error!(message = "Could not store message!", ?err);
                    Status::internal("Could not send the message due to an internal error")
                })?;
        }

        tracing::info!(message = "New message", sender = ?message.sender_uuid, room = ?message.room_uuid);

        Ok(Response::new(()))
    }

    #[tracing::instrument(skip(self))]
    async fn create_room(
        &self,
        request: Request<ClientsideRoom>,
    ) -> Result<Response<proto::Uuid>, Status> {
        let mut connection = self
            .connection_pool
            .get()
            .map_err(acquire_connection_error_status)?;
        let room = request.into_inner();
        let room_uuid = Room::from_room_with_members(room, &mut connection)?;

        Ok(Response::new(room_uuid.into()))
    }

    #[tracing::instrument(skip(self))]
    async fn create_room_with_user(
        &self,
        request: Request<RoomWithUserCreationRequest>,
    ) -> Result<Response<proto::Uuid>, Status> {
        let originator_uuid = request
            .get_originator()
            .map_err(|err| Status::internal(err.to_string()))?;
        let possible_interlocutor_uuid = request
            .into_inner()
            .user_uuid
            .and_then(|u| Uuid::try_from(u).ok())
            .ok_or(Status::invalid_argument("Invalid interlocutor UUID"))?;

        let mut connection = self
            .connection_pool
            .get()
            .map_err(acquire_connection_error_status)?;

        // Import some traits and methods to interact with the ORM.
        use crate::entities::schema::users::dsl::*;
        use diesel::prelude::*;

        let interlocutor = users
            .find(possible_interlocutor_uuid)
            .select(User::as_select())
            .first(&mut connection)
            .optional()
            .map_err(|err| Status::internal(err.to_string()))?
            .ok_or(Status::internal("No such user"))?;

        let originator = users
            .find(originator_uuid)
            .select(User::as_select())
            .first(&mut connection)
            .optional()
            .map_err(|err| Status::internal(err.to_string()))?
            .ok_or(Status::internal("No such user"))?;

        let room_name = format!(
            "Private chat between {} and {}",
            originator.username, interlocutor.username
        );
        let private_room_uuid = Room::from_room_with_members(
            ClientsideRoom {
                name: room_name,
                members: vec![interlocutor.uuid.into(), originator_uuid.into()],
            },
            &mut connection,
        )?;

        Ok(Response::new(private_room_uuid.into()))
    }

    type SubscribeToRoomStream = RPCStream<ServersideRoomEvent>;

    #[tracing::instrument(skip(self))]
    async fn subscribe_to_room(
        &self,
        _request: Request<proto::Uuid>,
    ) -> Result<Response<Self::SubscribeToRoomStream>, Status> {
        unimplemented!()
    }

    type SubscribeToUserStream = RPCStream<ServersideUserEvent>;

    #[tracing::instrument(skip(self))]
    async fn subscribe_to_user(
        &self,
        _request: Request<()>,
    ) -> Result<Response<Self::SubscribeToUserStream>, Status> {
        unimplemented!()
    }
}
