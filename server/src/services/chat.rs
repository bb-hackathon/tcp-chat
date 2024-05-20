use crate::auth::AuthenticatedRequest;
use crate::entities::{Message, Room, RoomUser, User};
use crate::proto::user_lookup_request::Identifier;
use crate::proto::{ClientsideMessage, ClientsideRoom, ServersideRoomEvent, ServersideUserEvent};
use crate::proto::{RoomWithUserCreationRequest, UserLookupRequest};
use crate::services::acquire_connection_error_status;
use crate::{persistence, proto};
use redis::{AsyncCommands, Client, RedisResult};
use std::{env, pin::Pin};
use tokio::sync::{broadcast, mpsc};
use tokio_stream::wrappers::ReceiverStream;
use tokio_stream::Stream;
use tonic::{Request, Response, Status};
use uuid::Uuid;

#[derive(Debug)]
pub struct Chat {
    // Connections to external services.
    persistence_pool: persistence::ConnectionPool,
    cache_client: redis::Client,

    // Message passing channel.
    message_sender: broadcast::Sender<Message>,
    // message_receiver: broadcast::Receiver<Message>,
}

impl Chat {
    pub async fn new(persistence_pool: persistence::ConnectionPool) -> RedisResult<Self> {
        let cache_client = Client::open(env::var("KV_URL").expect("Could not read $KV_URL"))?;
        let mut cache = cache_client.get_multiplexed_async_connection().await?;

        // Flush the membership cache.
        let _: () = redis::cmd("FLUSHALL")
            .query_async(&mut cache)
            .await
            .map_err(|error| {
                tracing::error!(message = "Could not flush the membership cache", ?error);
                error
            })?;

        // Acquire a connection to the database.
        let mut db = persistence_pool
            .get()
            .expect("Couldn't acquire a database connection");

        use crate::entities::schema::rooms_users::dsl::*;
        use crate::entities::schema::users::dsl::*;
        use diesel::prelude::*;

        // Get a list of registered users.
        let user_uuids = users
            .select(uuid)
            .load::<Uuid>(&mut db)
            .unwrap_or_else(|err| {
                tracing::error!(message = "Could not get a list of users from the DB", ?err);
                vec![]
            });

        // Set up the membership cache.
        for user in user_uuids.iter() {
            tracing::debug!(message = "Setting up membership cache", ?user);
            let rooms: Vec<Uuid> = rooms_users
                .filter(user_uuid.eq(user))
                .select(room_uuid)
                .load::<Uuid>(&mut db)
                .unwrap_or_default();

            for room in rooms.iter() {
                cache.rpush(user, room).await?;
            }
        }

        let (tx, _) = broadcast::channel::<Message>(16);

        Ok(Self {
            persistence_pool,
            cache_client,
            message_sender: tx,
            // message_receiver: rx,
        })
    }
}

type RPCStream<T> = Pin<Box<dyn Stream<Item = Result<T, Status>> + Send + 'static>>;

#[tonic::async_trait]
impl proto::chat_server::Chat for Chat {
    #[tracing::instrument(skip_all)]
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
            .persistence_pool
            .get()
            .map_err(acquire_connection_error_status)?;

        // Import some traits and methods to interact with the ORM.
        use crate::entities::schema::users::dsl::*;
        use diesel::prelude::*;

        let found_user: Option<User> = match identifier.clone() {
            Identifier::Uuid(proto_uuid) => {
                let proto_uuid = Uuid::try_from(proto_uuid)
                    .map_err(|_| Status::invalid_argument("The provided UUID is invalid"))?;
                users
                    .filter(uuid.eq::<Uuid>(proto_uuid))
                    .select(User::as_select())
                    .first(&mut connection)
            }
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

    #[tracing::instrument(skip_all)]
    async fn send_message(
        &self,
        request: Request<ClientsideMessage>,
    ) -> Result<Response<()>, Status> {
        let message = Message::try_from(request)?;

        let mut conn = self
            .persistence_pool
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

        tracing::info!(message = "New message", sender = ?message.sender_uuid, room = ?message.room_uuid);

        // Store the message in the database and mirror it to all receivers.
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

            match self.message_sender.send(message) {
                Ok(recv_count) => tracing::trace!(message = "Broadcasting message", ?recv_count),
                Err(err) => tracing::error!(message = "Could not broadcast message", ?err),
            }
        }

        Ok(Response::new(()))
    }

    #[tracing::instrument(skip_all)]
    async fn create_room(
        &self,
        request: Request<ClientsideRoom>,
    ) -> Result<Response<proto::Uuid>, Status> {
        let mut db = self
            .persistence_pool
            .get()
            .map_err(acquire_connection_error_status)?;
        let mut cache = self
            .cache_client
            .get_multiplexed_async_connection()
            .await
            .map_err(acquire_connection_error_status)?;
        let room = request.into_inner();
        let room_uuid = Room::from_room_with_members(room, &mut db, &mut cache).await?;

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

        let mut db = self
            .persistence_pool
            .get()
            .map_err(acquire_connection_error_status)?;

        let mut cache = self
            .cache_client
            .get_multiplexed_async_connection()
            .await
            .map_err(acquire_connection_error_status)?;

        // Import some traits and methods to interact with the ORM.
        use crate::entities::schema::users::dsl::*;
        use diesel::prelude::*;

        let interlocutor = users
            .find(possible_interlocutor_uuid)
            .select(User::as_select())
            .first(&mut db)
            .optional()
            .map_err(|err| Status::internal(err.to_string()))?
            .ok_or(Status::internal("No such user"))?;

        let originator = users
            .find(originator_uuid)
            .select(User::as_select())
            .first(&mut db)
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
            &mut db,
            &mut cache,
        )
        .await?;

        Ok(Response::new(private_room_uuid.into()))
    }

    type SubscribeToRoomStream = ReceiverStream<Result<ServersideRoomEvent, Status>>;

    #[tracing::instrument(skip(self))]
    async fn subscribe_to_room(
        &self,
        request: Request<proto::Uuid>,
    ) -> Result<Response<Self::SubscribeToRoomStream>, Status> {
        let subscriber: Uuid = request
            .get_originator()
            .expect("The authenticator should not let anonymous requests through");

        let subscribed_room: Uuid = request
            .into_inner()
            .try_into()
            .map_err(|_| Status::invalid_argument("Invalid room UUID"))?;

        let mut cache_connection = self
            .cache_client
            .get_multiplexed_async_connection()
            .await
            .map_err(|err| {
                let msg = "Couldn't acquire a cache connection";
                tracing::error!(message = msg, ?err);
                Status::internal(msg)
            })?;

        let subscriber_rooms: Vec<Uuid> = cache_connection.lrange(subscriber, 0, -1).await.unwrap();

        if !subscriber_rooms.contains(&subscribed_room) {
            return Err(Status::permission_denied(
                "You are not a member of this room",
            ));
        }

        let (tx, rx) = mpsc::channel(4);

        let mut message_rx = self.message_sender.subscribe();
        tokio::spawn(async move {
            while let Ok(msg) = message_rx.recv().await {
                let message_room = msg.room_uuid;
                let subscriber_rooms: Vec<Uuid> =
                    cache_connection.lrange(subscriber, 0, -1).await.unwrap();

                if subscriber_rooms.contains(&message_room) {
                    use proto::serverside_room_event::Event;
                    let new_message = Event::NewMessage(msg.into());
                    let event = ServersideRoomEvent {
                        room_uuid: Some(subscribed_room.into()),
                        event: Some(new_message),
                    };
                    tx.send(Ok(event)).await.unwrap();
                }
            }
        });

        Ok(Response::new(ReceiverStream::new(rx)))
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
