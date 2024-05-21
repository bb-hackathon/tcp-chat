use crate::persistence::Connection;
use crate::proto::ServersideRoom;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use std::fmt;
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

    pub async fn get_members(
        &self,
        db_connection: &mut PooledConnection<ConnectionManager<Connection>>,
    ) -> Vec<Uuid> {
        use crate::entities::schema::rooms_users::dsl::*;
        use diesel::prelude::*;

        let members: Vec<Uuid> = rooms_users
            .filter(room_uuid.eq(self.uuid))
            .select(user_uuid)
            .load(db_connection)
            .unwrap_or_else(|error| {
                tracing::error!(message = "Couldn't fetch membership from database", ?error);
                vec![]
            });

        members
    }
}

impl fmt::Display for ServersideRoom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} ({})",
            self.name,
            self.uuid.clone().unwrap_or_default().uuid
        )?;
        Ok(())
    }
}
