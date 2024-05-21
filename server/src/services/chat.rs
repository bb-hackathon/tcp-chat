use crate::auth::AuthenticatedRequest;
use crate::channel::DisconnectChannel;
use crate::entities::{Message, Room, RoomUser, User};
use crate::proto::serverside_user_event::Event;
use crate::proto::user_lookup_request::Identifier;
use crate::proto::{ClientsideMessage, ClientsideRoom, MessageList, RoomList};
use crate::proto::{RoomWithUserCreationRequest, UserLookupRequest};
use crate::proto::{ServersideMessage, ServersideRoom, ServersideRoomEvent, ServersideUserEvent};
use crate::{channel, persistence, proto};
use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::PgConnection;
use redis::aio::MultiplexedConnection;
use redis::{AsyncCommands, Client, RedisResult};
use std::env;
use tokio::sync::{broadcast, mpsc, oneshot};
use tokio_util::sync::CancellationToken;
use tonic::{Request, Response, Status};
use tracing::instrument;
use uuid::Uuid;

#[derive(Debug)]
pub struct Chat {
    // Connections to external services.
    persistence_pool: persistence::ConnectionPool,
    cache_client: redis::Client,

    // Message passing channel.
    message_tx: broadcast::Sender<Message>,
    user_event_tx: broadcast::Sender<ServersideUserEvent>,
}

#[tonic::async_trait]
impl proto::chat_server::Chat for Chat {
    #[instrument(skip_all)]
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

        let mut connection = self.acquire_database_connection().await?;

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

    #[instrument(skip_all)]
    async fn lookup_room(
        &self,
        request: Request<proto::Uuid>,
    ) -> Result<Response<ServersideRoom>, Status> {
        let _originator: Uuid = request
            .get_originator_uuid()
            .expect("The authenticator should not let anonymous requests through");

        let requested_room: Uuid = request
            .into_inner()
            .try_into()
            .map_err(|_| Status::invalid_argument("Invalid room UUID"))?;

        let mut db = self.acquire_database_connection().await?;

        use crate::entities::schema::rooms::dsl::*;
        use diesel::prelude::*;

        let db_room: Room = rooms
            .find(requested_room)
            .select(Room::as_select())
            .first(&mut db)
            .map_err(|error| {
                let msg = "Couldn't fetch rooms from database";
                tracing::error!(message = msg, ?error);
                Status::internal(msg)
            })?;

        let members: Vec<proto::Uuid> = db_room
            .get_members(&mut db)
            .await
            .into_iter()
            .map(|u| u.into())
            .collect();

        let serverside_room = ServersideRoom {
            uuid: Some(db_room.uuid.into()),
            name: db_room.name,
            members,
        };

        Ok(Response::new(serverside_room))
    }

    #[instrument(skip_all)]
    async fn list_rooms(&self, request: Request<()>) -> Result<Response<RoomList>, Status> {
        let originator = request
            .get_originator_uuid()
            .expect("The authenticator should not let anonymous requests through");

        let mut db = self.acquire_database_connection().await?;
        let mut cache = self.acquire_cache_connection().await?;

        let room_uuids: Vec<Uuid> = cache
            .lrange(originator, 0, -1)
            .await
            .unwrap_or_else(|error| {
                tracing::error!(message = "Couldn't get membership from cache", ?error);
                vec![]
            });

        use crate::entities::schema::rooms::dsl::*;
        use diesel::prelude::*;

        let db_rooms: Vec<Room> = rooms
            .filter(uuid.eq_any(room_uuids))
            .load::<Room>(&mut db)
            .map_err(|error| {
                let msg = "Couldn't load rooms from the database";
                tracing::error!(message = msg, ?error);
                Status::internal(msg)
            })?;

        let serverside_rooms_future: Vec<_> = db_rooms
            .into_iter()
            .map(|db_room| async move {
                let mut db = self
                    .acquire_database_connection()
                    .await
                    .expect("Couldn't acquire a database connection");

                let members: Vec<proto::Uuid> = db_room
                    .get_members(&mut db)
                    .await
                    .iter()
                    .map(|u| proto::Uuid::from(*u))
                    .collect();

                ServersideRoom {
                    uuid: Some(db_room.uuid.into()),
                    name: db_room.name,
                    members,
                }
            })
            .collect();

        let serverside_rooms = futures::future::join_all(serverside_rooms_future).await;

        Ok(Response::new(RoomList {
            rooms: serverside_rooms,
        }))
    }

    #[instrument(skip_all)]
    async fn list_messages(
        &self,
        request: Request<proto::Uuid>,
    ) -> Result<Response<MessageList>, Status> {
        let originator_uuid = request
            .get_originator_uuid()
            .expect("The authenticator should not let anonymous requests through");

        let requested_room_uuid: Uuid = request
            .into_inner()
            .try_into()
            .map_err(|_| Status::invalid_argument("Invalid room UUID"))?;

        // Ensure the user is a member of the rooms he's fetching messages from.
        if !self
            .check_room_membership(&originator_uuid, &requested_room_uuid)
            .await?
        {
            tracing::warn!(
                message = "User tried to fetch messages from a room he's not a member of",
                user = ?originator_uuid,
                room = ?requested_room_uuid
            );
            return Err(Status::permission_denied(
                "You are not a member of this room",
            ));
        }

        let mut db = self.acquire_database_connection().await?;

        use crate::entities::schema::messages::dsl::*;
        use diesel::prelude::*;

        let room_messages: Vec<Message> = messages
            .filter(room_uuid.eq(requested_room_uuid))
            .load::<Message>(&mut db)
            .map_err(|error| {
                let msg = "Couldn't fetch messages from database";
                tracing::error!(message = msg, ?error);
                Status::internal(msg)
            })?;

        let serverside_messages: Vec<ServersideMessage> =
            room_messages.into_iter().map(|m| m.into()).collect();

        Ok(Response::new(MessageList {
            messages: serverside_messages,
        }))
    }

    #[instrument(skip_all)]
    async fn send_message(
        &self,
        request: Request<ClientsideMessage>,
    ) -> Result<Response<()>, Status> {
        let message = Message::try_from(request)?;

        // Ensure the user isn't sending a message to a room he's not a member of.
        if !self
            .check_room_membership(&message.sender_uuid, &message.room_uuid)
            .await?
        {
            tracing::warn!(
                message = "User tried to send a message to a room he's not a member of",
                user = ?&message.sender_uuid,
                room = ?&message.room_uuid
            );
            return Err(Status::permission_denied(
                "You're not a member of this room",
            ));
        }

        tracing::info!(message = "New message", sender = ?message.sender_uuid, room = ?message.room_uuid);

        // Store the message in the database and mirror it to all receivers.
        {
            use crate::entities::schema::messages::dsl::*;
            use diesel::prelude::*;

            let mut conn = self.acquire_database_connection().await?;
            let _ = diesel::insert_into(messages)
                .values(&message)
                .execute(&mut conn)
                .map_err(|err| {
                    tracing::error!(message = "Could not store message!", ?err);
                    Status::internal("Could not send the message due to an internal error")
                })?;

            match self.message_tx.send(message) {
                Ok(recv_count) => tracing::trace!(message = "Broadcasting message", ?recv_count),
                Err(err) => tracing::error!(message = "Could not broadcast message", ?err),
            }
        }

        Ok(Response::new(()))
    }

    #[instrument(skip_all)]
    async fn create_room(
        &self,
        request: Request<ClientsideRoom>,
    ) -> Result<Response<proto::Uuid>, Status> {
        let room = request.into_inner();
        let room_uuid = self.create_room(room).await?;

        Ok(Response::new(room_uuid.into()))
    }

    #[instrument(skip_all)]
    async fn create_room_with_user(
        &self,
        request: Request<RoomWithUserCreationRequest>,
    ) -> Result<Response<proto::Uuid>, Status> {
        let originator_uuid = request
            .get_originator_uuid()
            .map_err(|err| Status::internal(err.to_string()))?;

        let possible_interlocutor_uuid = request
            .into_inner()
            .user_uuid
            .and_then(|u| Uuid::try_from(u).ok())
            .ok_or(Status::invalid_argument("Invalid interlocutor UUID"))?;

        let mut db = self.acquire_database_connection().await?;

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
        let private_room_uuid = self
            .create_room(ClientsideRoom {
                name: room_name,
                members: vec![interlocutor.uuid.into(), originator_uuid.into()],
            })
            .await?;

        Ok(Response::new(private_room_uuid.into()))
    }

    type SubscribeToRoomStream = DisconnectChannel<Result<ServersideRoomEvent, Status>>;

    #[instrument(skip_all)]
    async fn subscribe_to_room(
        &self,
        request: Request<proto::Uuid>,
    ) -> Result<Response<Self::SubscribeToRoomStream>, Status> {
        let subscriber: Uuid = request
            .get_originator_uuid()
            .expect("The authenticator should not let anonymous requests through");

        let subscribed_room: Uuid = request.into_inner().try_into().map_err(|error| {
            let msg = "The room UUID is invalid";
            tracing::trace!(message = msg, ?error);
            Status::invalid_argument(msg)
        })?;

        // Ensure the user is a member of the room he's subscribing to.
        if !self
            .check_room_membership(&subscriber, &subscribed_room)
            .await?
        {
            tracing::warn!(
                message = "User tried to subscribe to a room he's not a member of",
                ?subscriber,
                room = ?subscribed_room
            );
            return Err(Status::permission_denied(
                "You are not a member of this room",
            ));
        }

        // The 'streamer' thread (see below) needs a cache connection.
        let mut cache = self.acquire_cache_connection().await?;

        // NOTE: Read this.
        //
        // There are a total of 3 channels involved in this whole streaming thing:
        // - An internal `broadcast` channel that transfers messages from `SendMessage` RPC calls;
        // - A `DisconnectChannel`, which holds another 2 channels inside:
        //   - A `mpsc` Tokio channel, which performs gRPC streaming;
        //   - A `oneshot` Tokio channel, which fires when the client disconnects.

        let (grpc_tx, grpc_rx) = mpsc::channel(4);
        let (disconnect_tx, disconnect_rx) = oneshot::channel();
        let disconnect_channel = channel::DisconnectChannel {
            disconnect_tx: Some(disconnect_tx),
            grpc_rx,
        };

        let mut message_rx = self.message_tx.subscribe();
        tracing::info!(message = "New room subscriber", room = ?subscribed_room);

        // The logic for the streaming thread, extracted into a variable to help rustfmt.
        let streaming_closure = async move {
            while let Ok(msg) = message_rx.recv().await {
                let message_room = msg.room_uuid;
                let subscriber_rooms: Vec<Uuid> = cache
                    .lrange(subscriber, 0, -1)
                    .await
                    .unwrap_or_else(|error| {
                        tracing::error!(
                            message = "Could not retrieve membership from cache",
                            ?subscriber,
                            ?error
                        );
                        vec![]
                    });

                // Check that the user is a member of the room and that he's subscribed to the rooms the message is from.
                if subscriber_rooms.contains(&message_room) && subscribed_room == message_room {
                    use proto::serverside_room_event::Event;
                    let new_message = Event::NewMessage(msg.into());
                    let event = ServersideRoomEvent {
                        room_uuid: Some(subscribed_room.into()),
                        event: Some(new_message),
                    };

                    let send_result = grpc_tx.send(Ok(event)).await;
                    if send_result.is_err() {
                        tracing::warn!(
                            message = "A message was sent, but nobody is subscribed to the channel"
                        )
                    }
                }
            }
        };

        // The 'canceller' thread will cancel this token when the client disconnects.
        let token = CancellationToken::new();
        let token_clone = token.clone();

        // This is the 'canceller' thread.
        //
        // This task will cancel the token when the client disconnects, which will shutdown
        // the streaming thread (see below) and cause the broadcast::Receiver to drop.
        tokio::spawn(async move {
            let _ = disconnect_rx.await;
            tracing::debug!(message = "Client disconnected, stopping message streaming");
            token.cancel();
        });

        // This is the 'streamer' thread.
        //
        // This thread will receive all messages sent via the `SendMessage` RPC call, and
        // mirror them to all subsribers. Without a canceller thread, a cancellation token
        // and a hacky DisconnectChannel, this thread would never terminate, meaning there
        // would soon be a thousand of hanging broadcast::Receivers with no real client.
        tokio::spawn(async move {
            tokio::select! {
                _ = token_clone.cancelled() => {}
                _ = streaming_closure => {}
            }
        });

        Ok(Response::new(disconnect_channel))
    }

    type SubscribeToUserStream = DisconnectChannel<Result<ServersideUserEvent, Status>>;

    #[instrument(skip_all)]
    async fn subscribe_to_user(
        &self,
        request: Request<()>,
    ) -> Result<Response<Self::SubscribeToUserStream>, Status> {
        let user_uuid: Uuid = request
            .get_originator_uuid()
            .expect("The authenticator should not let anonymous requests through");

        let (grpc_tx, grpc_rx) = mpsc::channel(4);
        let (disconnect_tx, disconnect_rx) = oneshot::channel();
        let disconnect_channel = DisconnectChannel {
            disconnect_tx: Some(disconnect_tx),
            grpc_rx,
        };

        let mut user_event_rx = self.user_event_tx.subscribe();
        let streaming_closure = async move {
            while let Ok(event) = user_event_rx.recv().await {
                if event
                    .user_uuid
                    .clone()
                    .is_some_and(|event_user_uuid| user_uuid == event_user_uuid)
                {
                    let send_result = grpc_tx.send(Ok(event)).await;
                    if send_result.is_err() {
                        tracing::trace!(message = "A user event occurred, but nobody is subscribed")
                    }
                }
            }
        };

        let token = CancellationToken::new();
        let token_clone = token.clone();

        // Spawn the "canceller" thread.
        tokio::spawn(async move {
            let _ = disconnect_rx.await;
            tracing::debug!(message = "Client disconnected, stopping user event streaming");
            token.cancel();
        });

        // Spawn the "streamer" thread.
        tokio::spawn(async move {
            tokio::select! {
                _ = token_clone.cancelled() => {}
                _ = streaming_closure => {}
            }
        });

        Ok(Response::new(disconnect_channel))
    }
}

impl Chat {
    const INTERNAL_CHANNEL_CAPACITY: usize = 16;

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

        let (message_tx, _) = broadcast::channel(Self::INTERNAL_CHANNEL_CAPACITY);
        let (user_event_tx, _) = broadcast::channel(Self::INTERNAL_CHANNEL_CAPACITY);

        Ok(Self {
            persistence_pool,
            cache_client,
            message_tx,
            user_event_tx,
        })
    }

    #[instrument(skip_all)]
    async fn acquire_database_connection(
        &self,
    ) -> Result<PooledConnection<ConnectionManager<PgConnection>>, Status> {
        let db_connection = self.persistence_pool.get().map_err(|error| {
            let msg = "Couldn't acquire a database connection";
            tracing::error!(message = msg, ?error);
            Status::internal(msg)
        })?;

        Ok(db_connection)
    }

    #[instrument(skip_all)]
    async fn acquire_cache_connection(&self) -> Result<MultiplexedConnection, Status> {
        let cache_connection = self
            .cache_client
            .get_multiplexed_async_connection()
            .await
            .map_err(|error| {
                let msg = "Couldn't acquire a cache connection";
                tracing::error!(message = msg, ?error);
                Status::internal(msg)
            })?;

        Ok(cache_connection)
    }

    #[instrument]
    async fn check_room_membership(&self, user: &Uuid, room: &Uuid) -> Result<bool, Status> {
        let mut cache = self.acquire_cache_connection().await?;
        let allowed_rooms: Vec<Uuid> = cache.lrange(user, 0, -1).await.unwrap_or_else(|error| {
            tracing::error!(message = "Couldn't get membership from cache", ?error);
            vec![]
        });

        Ok(allowed_rooms.contains(room))
    }

    #[instrument(skip_all)]
    async fn create_room(&self, clientside_room: ClientsideRoom) -> Result<Uuid, Status> {
        let mut db_connection = self.acquire_database_connection().await?;
        let mut cache_connection = self.acquire_cache_connection().await?;

        let user_uuids: Vec<Uuid> = clientside_room
            .members
            .into_iter()
            .map(Uuid::try_from)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|error| {
                let error = error.to_string();
                let message = format!("Some member UUIDs could not be converted: {error}");
                Status::invalid_argument(message)
            })?;

        let room = Room::new(clientside_room.name);
        let room_uuid = room.uuid;
        let members: Vec<RoomUser> = user_uuids
            .iter()
            .map(|user_uuid| RoomUser {
                room_uuid,
                user_uuid: *user_uuid,
            })
            .collect();

        // Store the room and members in the database.
        {
            use crate::entities::schema::rooms::dsl::*;
            use crate::entities::schema::rooms_users::dsl::*;
            use diesel::{insert_into, RunQueryDsl};

            let _ = insert_into(rooms)
                .values(&room)
                .execute(&mut db_connection)
                .map_err(|error| {
                    let error = error.to_string();
                    let message = format!("Could not save the room in the database: {error}");
                    Status::internal(message)
                })?;

            let _ = insert_into(rooms_users)
                .values(&members)
                .execute(&mut db_connection)
                .map_err(|error| {
                    let error = error.to_string();
                    let message = format!("Could not save the room's members: {error}");
                    Status::internal(message)
                })?;

            tracing::info!(message = "Created new room", members = ?user_uuids, uuid = ?room.uuid);
        }

        // Update the membership cache.
        for user_uuid in user_uuids.into_iter() {
            let _: () = cache_connection
                .rpush(user_uuid, room.uuid)
                .await
                .map_err(|error| {
                    let message = "Could not update membership cache";
                    tracing::error!(message = message, ?error);
                    Status::internal(message)
                })?;

            let event = ServersideUserEvent {
                user_uuid: Some(user_uuid.into()),
                event: Some(Event::AddedToRoom(room.uuid.into())),
            };

            match self.user_event_tx.send(event) {
                Ok(recv_count) => tracing::trace!(message = "Broadcasting user event", ?recv_count),
                Err(error) => tracing::error!(message = "Could not broadcast user event", ?error),
            }
        }

        tracing::info!(message = "Updated membership cache", room = ?room.uuid);

        Ok(room.uuid)
    }
}
