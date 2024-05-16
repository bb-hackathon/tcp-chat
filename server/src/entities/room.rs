use diesel::prelude::*;
use uuid::Uuid;

#[derive(Queryable, Identifiable, Selectable, Insertable, Debug, Clone)]
#[diesel(table_name = crate::entities::schema::rooms)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(primary_key(uuid))]
pub struct Room {
    uuid: Uuid,
    name: String,
}

impl Room {
    pub fn new<T: Into<String>>(name: T) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            name: name.into(),
        }
    }

    #[must_use]
    pub const fn uuid(&self) -> &Uuid {
        &self.uuid
    }

    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }
}
