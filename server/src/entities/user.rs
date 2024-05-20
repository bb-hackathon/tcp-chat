use crate::entities::token::AuthToken;
use crate::proto::{self, AuthPair};
use diesel::prelude::*;
use rand_chacha::ChaCha20Rng;
use std::str::FromStr;
use uuid::Uuid;

#[derive(Queryable, Selectable, Insertable, Identifiable, Clone, Debug)]
#[diesel(table_name = crate::entities::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(primary_key(uuid))]
pub struct User {
    pub uuid: Uuid,
    pub username: String,
    pub password: String,
    pub auth_token: String,
}

impl User {
    pub fn new(username: String, password: String, rng: &mut ChaCha20Rng) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            username,
            password,
            auth_token: AuthToken::new(rng).to_string(),
        }
    }

    pub fn auth_token(&self) -> AuthToken {
        AuthToken::from_str(&self.auth_token).expect("The database should only store valid UUIDs")
    }

    pub fn proto_token(&self) -> proto::AuthToken {
        self.auth_token().into()
    }

    pub fn auth_pair(&self) -> AuthPair {
        AuthPair {
            user_uuid: Some(self.uuid.into()),
            token: Some(self.proto_token()),
        }
    }
}

impl From<User> for proto::User {
    fn from(user: User) -> Self {
        Self {
            uuid: Some(user.uuid.into()),
            username: user.username,
        }
    }
}
