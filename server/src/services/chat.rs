use crate::auth::AuthenticatedRequest;
use crate::entities::{Message, Room, User};
use crate::persistence::ConnectionPool;
use crate::proto::{self, RoomWithUserCreationRequest};
use crate::proto::{ClientsideMessage, ClientsideRoom, ServersideRoomEvent, ServersideUserEvent};
use crate::proto::{UserUuidLookupRequest, UsernameLookupRequest};
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
    async fn lookup_user_uuid(
        &self,
        request: Request<UserUuidLookupRequest>,
    ) -> Result<Response<proto::User>, Status> {
        let lookup_request = request.get_ref();

        let mut connection = self
            .connection_pool
            .get()
            .map_err(acquire_connection_error_status)?;

        // Import some traits and methods to interact with the ORM.
        use crate::entities::schema::users::dsl::*;
        use diesel::query_dsl::methods::{FilterDsl, SelectDsl};
        use diesel::{ExpressionMethods, OptionalExtension, RunQueryDsl, SelectableHelper};

        let user_with_matching_username = users
            .filter(username.eq(&lookup_request.username))
            .select(User::as_select())
            .first(&mut connection)
            .optional()
            .map_err(|err| Status::internal(err.to_string()))?;

        match user_with_matching_username {
            Some(user) => {
                tracing::debug!(message = "Successful user lookup", username = ?lookup_request.username, uuid = ?user.uuid);
                Ok(Response::new(proto::User::from(user.clone())))
            }
            None => {
                tracing::debug!(message = "Unsuccessful user lookup", username = ?lookup_request.username);
                Err(Status::not_found("No user with such username"))
            }
        }
    }

    #[tracing::instrument(skip(self))]
    async fn lookup_username(
        &self,
        request: Request<UsernameLookupRequest>,
    ) -> Result<Response<proto::User>, Status> {
        let lookup_request = request.get_ref();
        let lookup_uuid = lookup_request
            .uuid
            .clone()
            .and_then(|u| Uuid::try_from(u).ok())
            .ok_or(Status::invalid_argument("Invalid UUID"))?;

        let mut connection = self
            .connection_pool
            .get()
            .map_err(acquire_connection_error_status)?;

        // Import some traits and methods to interact with the ORM.
        use crate::entities::schema::users::dsl::*;
        use diesel::query_dsl::methods::{FilterDsl, SelectDsl};
        use diesel::{ExpressionMethods, OptionalExtension, RunQueryDsl, SelectableHelper};

        let user_with_matching_uuid = users
            .filter(uuid.eq(lookup_uuid))
            .select(User::as_select())
            .first(&mut connection)
            .optional()
            .map_err(|err| Status::internal(err.to_string()))?;

        match user_with_matching_uuid {
            Some(user) => {
                tracing::debug!(message = "Successful user lookup", username = ?user.username, uuid = ?user.uuid);
                Ok(Response::new(proto::User::from(user.clone())))
            }
            None => {
                tracing::debug!(message = "Unsuccessful user lookup", uuid = ?lookup_uuid);
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

        // Import some traits and methods to interact with the ORM.
        use crate::entities::schema::messages::dsl::*;
        use diesel::RunQueryDsl;
        let _ = diesel::insert_into(messages)
            .values(&message)
            .execute(&mut conn)
            .map_err(|err| Status::internal(err.to_string()))?;

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
            .ok_or(Status::invalid_argument(""))?;

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

        let private_room_uuid = Room::from_room_with_members(
            ClientsideRoom {
                name: format!("Private chat between {} and ...", interlocutor.username),
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
