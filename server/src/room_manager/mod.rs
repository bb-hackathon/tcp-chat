use crate::proto::room_manager_server::RoomManager as Manager;
use crate::proto::{Message, Room, RoomCount, Uuid};
use color_eyre::eyre::Result;
use std::pin::Pin;
use tokio::sync::Mutex;
use tokio_stream::Stream;
use tonic::{Request, Response, Streaming};
use tracing::instrument;

#[derive(Debug, Default)]
pub struct RoomManager {
    rooms: Mutex<Vec<Room>>,
}

type RPCResponse<T> = Result<Response<T>, tonic::Status>;
type MessageStream = Pin<Box<dyn Stream<Item = Result<Message, tonic::Status>> + Send>>;

#[tonic::async_trait]
impl Manager for RoomManager {
    type ReceiveStream = MessageStream;

    #[instrument(skip(self))]
    async fn create(&self, room: Request<Room>) -> RPCResponse<Uuid> {
        let room = room.into_inner();
        let uuid = room.uuid.clone().unwrap_or_default().uuid;
        tracing::info!(message = "Creating room", ?uuid);
        self.rooms.lock().await.push(room);
        Ok(Response::new(Uuid { uuid }))
    }

    #[instrument(skip(self))]
    async fn delete(&self, uuid: Request<Uuid>) -> RPCResponse<()> {
        let uuid = uuid.into_inner();
        let mut rooms = self.rooms.lock().await;
        if let Some(index) = rooms
            .iter()
            .position(|room| room.uuid.as_ref() == Some(&uuid))
        {
            rooms.swap_remove(index);
            drop(rooms);
            tracing::info!(message = "Deleting room", ?uuid);
            Ok(Response::new(()))
        } else {
            tracing::warn!(message = "No such room", ?uuid);
            Err(tonic::Status::not_found(uuid.uuid))
        }
    }

    #[instrument(skip(self))]
    async fn get_room_count(&self, _request: Request<()>) -> RPCResponse<RoomCount> {
        Ok(Response::new(RoomCount {
            count: self.rooms.lock().await.len() as u32,
        }))
    }

    #[allow(unused_variables)]
    #[instrument]
    async fn add_user(&self, uuid: Request<Uuid>) -> RPCResponse<()> {
        todo!("Handle user addition")
    }

    #[allow(unused_variables)]
    #[instrument]
    async fn kick_user(&self, uuid: Request<Uuid>) -> RPCResponse<()> {
        todo!("Handle user kickouts")
    }

    #[allow(unused_variables)]
    #[instrument]
    async fn send(&self, message: Request<Streaming<Message>>) -> RPCResponse<()> {
        todo!("Handle message sending")
    }

    #[allow(unused_variables)]
    #[instrument]
    async fn receive(&self, uuid: Request<Uuid>) -> RPCResponse<MessageStream> {
        todo!("Handle message receiving")
    }
}
