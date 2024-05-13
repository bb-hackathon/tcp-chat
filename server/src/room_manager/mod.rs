#![allow(clippy::single_match_else)]
#![allow(clippy::significant_drop_in_scrutinee)]

use crate::proto::result::Type;
use crate::proto::room_manager_server::RoomManager as Manager;
use crate::proto::Result as ProtobufResult;
use crate::proto::Uuid as ProtobufUuid;
use crate::proto::{Room, RoomCreationRequest, RoomDeletionRequest};
use color_eyre::eyre::Result;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};
use tracing::instrument;
use uuid::Uuid;

#[derive(Debug, Default)]
pub struct RoomManager {
    rooms: Arc<Mutex<Vec<Room>>>,
}

type RPCResponse<T> = Result<Response<T>, Status>;

#[tonic::async_trait]
impl Manager for RoomManager {
    #[instrument(skip(self))]
    async fn create_room(
        &self,
        request: Request<RoomCreationRequest>,
    ) -> RPCResponse<ProtobufUuid> {
        let room = request.into_inner();
        let visible_name = room.visible_name.clone();
        let uuid = Uuid::new_v4();

        tracing::info!(message = "Creating room", ?visible_name);
        self.rooms.lock().await.push(Room {
            uuid: Some(uuid.into()),
            visible_name,
            user_uuids: room.user_uuids,
        });

        Ok(Response::new(ProtobufUuid {
            uuid: uuid.to_string(),
        }))
    }

    #[instrument(skip(self))]
    async fn delete_room(
        &self,
        request: Request<RoomDeletionRequest>,
    ) -> RPCResponse<ProtobufResult> {
        let proto_uuid = request.into_inner().room_uuid.unwrap_or_default();
        let mut rooms = self.rooms.lock().await;
        match rooms
            .iter()
            .position(|room| room.uuid.as_ref() == Some(&proto_uuid))
        {
            Some(index) => {
                rooms.swap_remove(index);
                drop(rooms);
                tracing::info!(message = "Deleting room", ?proto_uuid);
                Ok(Response::new(ProtobufResult {
                    r#type: Some(Type::Ok(proto_uuid.uuid)),
                }))
            }
            None => {
                tracing::warn!(message = "No such room", ?proto_uuid);
                Err(Status::not_found(proto_uuid.uuid))
            }
        }
    }

    type GetRoomsStream = ReceiverStream<Result<Room, Status>>;

    #[instrument(skip(self, _request))]
    async fn get_rooms(&self, _request: Request<()>) -> RPCResponse<Self::GetRoomsStream> {
        let (tx, rx) = tokio::sync::mpsc::channel(128);
        let rooms = self.rooms.clone();

        tokio::spawn(async move {
            for room in rooms.lock().await.iter() {
                tx.send(Ok(room.clone())).await.unwrap();
            }
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }
}
