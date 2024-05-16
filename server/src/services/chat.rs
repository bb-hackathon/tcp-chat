use crate::entities::User;
use crate::persistence::ConnectionPool;
use crate::proto::{self, ServersideUserEvent};
use crate::proto::{ClientsideMessage, ClientsideRoom, ServersideRoomEvent, UserUuidLookupRequest};
use std::pin::Pin;
use tokio_stream::Stream;
use tonic::{Request, Response, Status};

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
        request: Request<UserUuidLookupRequest>,
    ) -> Result<Response<proto::User>, Status> {
        let lookup_request = request.get_ref();

        let mut connection = self
            .connection_pool
            .get()
            .map_err(|_| Status::internal("Could not acquire a database connection"))?;

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
    async fn send_message(
        &self,
        _request: Request<ClientsideMessage>,
    ) -> Result<Response<()>, Status> {
        unimplemented!()
    }

    #[tracing::instrument(skip(self))]
    async fn create_room(&self, _request: Request<ClientsideRoom>) -> Result<Response<()>, Status> {
        unimplemented!()
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
