use crate::entities::relations::RoomUser;
use crate::{persistence::Connection, proto::ClientsideRoom};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use redis::{aio::MultiplexedConnection, AsyncCommands};
use uuid::Uuid;

#[derive(Queryable, Identifiable, Selectable, Insertable, Debug, Clone)]
#[diesel(table_name = crate::entities::schema::rooms)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(primary_key(uuid))]
pub struct Room {
    pub uuid: Uuid,
    pub name: String,
}

impl Room {
    pub fn new<T: Into<String>>(name: T) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            name: name.into(),
        }
    }

    #[tracing::instrument(skip_all)]
    pub async fn from_room_with_members(
        clientside_room: ClientsideRoom,
        db_connection: &mut PooledConnection<ConnectionManager<Connection>>,
        cache_connection: &mut MultiplexedConnection,
    ) -> Result<Uuid, tonic::Status> {
        let user_uuids = clientside_room
            .members
            .into_iter()
            .map(Uuid::try_from)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|err| {
                let err = err.to_string();
                let msg = format!("Some member UUIDs could not be converted: {err}");
                tonic::Status::invalid_argument(msg)
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
                .execute(db_connection)
                .map_err(|err| {
                    let err = err.to_string();
                    let msg = format!("Could not save the room in the database: {err}");
                    tonic::Status::internal(msg)
                })?;

            let _ = insert_into(rooms_users)
                .values(&members)
                .execute(db_connection)
                .map_err(|err| {
                    let err = err.to_string();
                    let msg = format!("Could not save the room's members: {err}");
                    tonic::Status::internal(msg)
                })?;

            tracing::info!(message = "Created new room", members = ?user_uuids, uuid = ?room.uuid);
        }

        // Update the membership cache.
        for user_uuid in user_uuids.into_iter() {
            let _: () = cache_connection
                .rpush(user_uuid, room.uuid)
                .await
                .map_err(|err| {
                    let msg = "Could not update membership cache";
                    tracing::error!(message = msg, ?err);
                    tonic::Status::internal(msg)
                })?;
        }

        tracing::info!(message = "Updated membership cache", room = ?room.uuid);

        Ok(room.uuid)
    }
}
