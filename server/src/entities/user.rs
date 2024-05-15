use crate::token::AuthToken;
use rand_chacha::ChaCha20Rng;
use uuid::Uuid;

#[derive(Debug)]
pub struct User {
    username: String,
    password: String,
    uuid: Uuid,
    auth_token: AuthToken,
}

impl User {
    pub fn new(username: String, password: String, rng: &mut ChaCha20Rng) -> Self {
        Self {
            username,
            password,
            uuid: Uuid::new_v4(),
            auth_token: AuthToken::new(rng),
        }
    }

    #[must_use]
    pub fn username(&self) -> &str {
        &self.username
    }

    #[must_use]
    pub fn password(&self) -> &str {
        &self.password
    }

    #[must_use]
    pub const fn uuid(&self) -> &Uuid {
        &self.uuid
    }

    #[must_use]
    pub const fn auth_token(&self) -> &AuthToken {
        &self.auth_token
    }
}
