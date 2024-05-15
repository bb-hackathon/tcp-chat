use crate::entities::User;
use crate::proto::{self, AuthPair, UserCredentials};
use rand_chacha::ChaCha20Rng;
use rand_core::{OsRng, RngCore, SeedableRng};
use std::sync::Arc;
use tokio::sync::Mutex;
use tonic::{Request, Response, Status};

#[derive(Debug)]
pub struct Registry {
    user_repo: Arc<Mutex<Vec<User>>>,
    rng: Arc<Mutex<ChaCha20Rng>>,
}

impl Registry {
    #[must_use]
    pub fn get_user_repo(&self) -> Arc<Mutex<Vec<User>>> {
        Arc::clone(&self.user_repo)
    }
}

#[tonic::async_trait]
impl proto::registry_server::Registry for Registry {
    #[tracing::instrument(skip(self))]
    async fn register_new_user(
        &self,
        request: Request<UserCredentials>,
    ) -> Result<Response<()>, Status> {
        let credentials = request.into_inner();
        let username = credentials.username;
        let password = credentials.password;
        let mut user_repo = self.user_repo.lock().await;
        let duplicate_user = user_repo.iter().find(|user| user.username() == username);

        match duplicate_user {
            // No duplicate usernames found, registering a new account.
            None => {
                user_repo.push(User::new(
                    username.clone(),
                    password,
                    &mut *self.rng.lock().await,
                ));
                drop(user_repo);
                tracing::info!(message = "Created new user", ?username);
                Ok(Response::new(()))
            }

            // A user with a matching username was found, refuse to register.
            Some(_) => {
                tracing::warn!(message = "Refusing to register duplicate user", ?username);
                Err(Status::already_exists("Such user already exists"))
            }
        }
    }

    #[tracing::instrument(skip(self))]
    #[allow(clippy::significant_drop_tightening)]
    async fn login_as_user(
        &self,
        request: Request<UserCredentials>,
    ) -> Result<Response<AuthPair>, Status> {
        let credentials = request.into_inner();
        let username = credentials.username;
        let password = credentials.password;
        let user_repo = self.user_repo.lock().await;
        let matching_user = user_repo
            .iter()
            .find(|user| user.username() == username && user.password() == password);

        match matching_user {
            // A an account with matching credentials exist, returns its UUID and token.
            Some(user) => {
                tracing::info!(message = "Authentication successful", ?username);
                Ok(Response::new(AuthPair {
                    user_uuid: Some((*user.uuid()).into()),
                    token: Some(user.auth_token().clone().into()),
                }))
            }

            // No matching username+password pair was found, reject.
            None => {
                let msg = "Invalid credentials";
                tracing::warn!(message = msg, ?username);
                Err(Status::unauthenticated(msg))
            }
        }
    }
}

impl Default for Registry {
    fn default() -> Self {
        let rng = ChaCha20Rng::seed_from_u64(OsRng.next_u64());
        Self {
            user_repo: Arc::new(Mutex::new(vec![])),
            rng: Arc::new(Mutex::new(rng)),
        }
    }
}
