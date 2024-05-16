use super::{Room, User};
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Queryable, Identifiable, Selectable, Insertable, Associations, Debug, Clone)]
#[diesel(table_name = crate::entities::schema::rooms_users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(Room, foreign_key = room_uuid))]
#[diesel(belongs_to(User, foreign_key = user_uuid))]
#[diesel(primary_key(room_uuid, user_uuid))]
pub struct RoomUser {
    pub room_uuid: Uuid,
    pub user_uuid: Uuid,
}
