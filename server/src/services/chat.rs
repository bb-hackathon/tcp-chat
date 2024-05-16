use crate::persistence::ConnectionPool;
use crate::proto::{self, ServersideUserEvent};
use crate::proto::{ClientsideMessage, ClientsideRoom, ServersideRoomEvent, UserUuidLookupRequest};
use std::pin::Pin;
use tokio_stream::Stream;
use tonic::{Request, Response, Status};

#[derive(Debug, Clone)]
pub struct Chat {
    _connection_pool: ConnectionPool,
}

impl Chat {
    pub fn new(connection_pool: ConnectionPool) -> Self {
        Self {
            _connection_pool: connection_pool,
        }
    }
}

type RPCStream<T> = Pin<Box<dyn Stream<Item = Result<T, Status>> + Send + 'static>>;

#[tonic::async_trait]
impl proto::chat_server::Chat for Chat {
    #[tracing::instrument(skip(self))]
    #[allow(clippy::significant_drop_tightening)]
    async fn lookup_user(
        &self,
        _request: Request<UserUuidLookupRequest>,
    ) -> Result<Response<proto::User>, Status> {
        // let lookup_request = request.into_inner();
        // let user_repo = self.user_repo.lock().await;
        // let user = user_repo
        //     .iter()
        //     .find(|user| user.username == lookup_request.username);

        // match user {
        //     Some(user) => {
        //         let username = &user.username;
        //         let uuid = &user.uuid;
        //         tracing::debug!(message = "Successful user lookup", ?username, ?uuid);
        //         Ok(Response::new(proto::User::from(user.clone())))
        //     }
        //     None => {
        //         tracing::debug!(message = "Unsuccessful user lookup", ?lookup_request);
        //         Err(Status::not_found("No user with such username"))
        //     }
        // }

        unimplemented!()
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
