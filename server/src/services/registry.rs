use crate::entities::schema::users::dsl::*;
use crate::entities::User;
use crate::persistence::ConnectionPool;
use crate::proto::{self, AuthPair, UserCredentials};
use rand_chacha::ChaCha20Rng;
use rand_core::{OsRng, RngCore, SeedableRng};
use std::sync::Arc;
use tokio::sync::Mutex;
use tonic::{Request, Response, Status};

#[derive(Debug)]
pub struct Registry {
    connection_pool: ConnectionPool,
    rng: Arc<Mutex<ChaCha20Rng>>,
}

impl Registry {
    pub fn with_connection_pool(connection_pool: ConnectionPool) -> Self {
        let rng = ChaCha20Rng::seed_from_u64(OsRng.next_u64());
        Self {
            connection_pool,
            rng: Arc::new(Mutex::new(rng)),
        }
    }
}

#[tonic::async_trait]
impl proto::registry_server::Registry for Registry {
    #[tracing::instrument(skip(self, request))]
    async fn register_new_user(
        &self,
        request: Request<UserCredentials>,
    ) -> Result<Response<()>, Status> {
        let mut connection = self
            .connection_pool
            .get()
            .map_err(|_| Status::internal("Database pool error"))?;

        // Import some traits and methods to interact with the ORM.
        use diesel::query_dsl::methods::{FilterDsl, SelectDsl};
        use diesel::{ExpressionMethods, OptionalExtension, RunQueryDsl, SelectableHelper};

        let credentials = request.into_inner();
        let duplicate_user = users
            .filter(username.eq(&credentials.username))
            .select(User::as_select())
            .first(&mut connection)
            .optional()
            .map_err(|err| Status::internal(err.to_string()))?;

        match duplicate_user {
            // No duplicate usernames found, registering a new account.
            None => {
                let mut rng = self.rng.lock().await;
                let user = User::new(credentials.username.clone(), credentials.password, &mut rng);
                let _ = diesel::insert_into(users)
                    .values(&user)
                    .execute(&mut connection)
                    .map_err(|err| Status::internal(err.to_string()))?;
                tracing::info!(message = "Registered new user", username = ?credentials.username);
                Ok(Response::new(()))
            }

            // A user with a matching username was found, refuse to register.
            Some(_) => {
                const MESSAGE: &str = "Such user already exists";
                tracing::warn!(message = MESSAGE, username = ?credentials.username);
                Err(Status::already_exists(MESSAGE))
            }
        }
    }

    #[tracing::instrument(skip(self, request))]
    async fn login_as_user(
        &self,
        request: Request<UserCredentials>,
    ) -> Result<Response<AuthPair>, Status> {
        let mut connection = self
            .connection_pool
            .get()
            .map_err(|_| Status::internal("Database pool error"))?;

        // Import some traits and methods to interact with the ORM.
        use diesel::query_dsl::methods::{FilterDsl, SelectDsl};
        use diesel::{ExpressionMethods, OptionalExtension, RunQueryDsl, SelectableHelper};

        let credentials = request.into_inner();
        let candidate_user = users
            .filter(username.eq(&credentials.username))
            .filter(password.eq(&credentials.password))
            .select(User::as_select())
            .first(&mut connection)
            .optional()
            .map_err(|err| Status::internal(err.to_string()))?;

        match candidate_user {
            // A an account with matching credentials exist, returns its UUID and token.
            Some(user) => {
                tracing::debug!(message = "Login successful", username = ?credentials.username);
                Ok(Response::new(user.auth_pair()))
            }

            // No matching username+password pair was found, reject.
            None => {
                const MESSAGE: &str = "Invalid username or password";
                tracing::warn!(message = MESSAGE, username = ?credentials.username);
                Err(Status::unauthenticated(MESSAGE))
            }
        }
    }
}
