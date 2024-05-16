use crate::{persistence::Connection, proto::ClientsideRoom};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use uuid::Uuid;

use super::RoomUser;

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

    pub fn from_room_with_members(
        clientside_room: ClientsideRoom,
        db_connection: &mut PooledConnection<ConnectionManager<Connection>>,
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
            .into_iter()
            .map(|user_uuid| RoomUser {
                room_uuid,
                user_uuid,
            })
            .collect();

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
        }

        Ok(room.uuid)
    }
}
