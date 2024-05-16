use crate::entities::token::AuthToken;
use crate::entities::user::Repo;
use crate::proto::AuthPair;
use std::str::FromStr;
use tokio::runtime::Handle;
use tonic::{service::Interceptor, Request, Status};
use uuid::Uuid;

#[allow(unused, clippy::missing_errors_doc)]
pub trait AuthenticatedRequest {
    type Error;

    fn add_auth_pair(&mut self, auth_pair: AuthPair) -> Result<(), Self::Error>;
    fn get_auth_pair(&self) -> Result<AuthPair, Self::Error>;
}

#[derive(thiserror::Error, Debug, Clone, Copy)]
#[allow(clippy::module_name_repetitions)]
pub enum AuthMetadataError {
    #[error("Could not add AuthPair to request metadata")]
    InsertionError,
    #[error("Could not retrieve an AuthPair from request metadata")]
    RetrievalError,
}

impl<T> AuthenticatedRequest for Request<T> {
    type Error = AuthMetadataError;

    fn add_auth_pair(&mut self, auth_pair: AuthPair) -> Result<(), Self::Error> {
        let error = AuthMetadataError::InsertionError;
        let user_uuid: Uuid = auth_pair
            .user_uuid
            .and_then(|uuid| uuid.try_into().ok())
            .ok_or(error)?;
        let user_uuid = user_uuid.to_string().parse().map_err(|_| error)?;
        let auth_token: AuthToken = auth_pair
            .token
            .and_then(|token| token.try_into().ok())
            .ok_or(error)?;
        let auth_token = auth_token.to_string().parse().map_err(|_| error)?;
        let metadata = self.metadata_mut();
        metadata.insert(Authenticator::USER_UUID_KEY, user_uuid);
        metadata.insert(Authenticator::AUTH_TOKEN_KEY, auth_token);
        Ok(())
    }

    fn get_auth_pair(&self) -> Result<AuthPair, Self::Error> {
        let error = AuthMetadataError::RetrievalError;
        let metadata = self.metadata();
        let user_uuid = metadata
            .get(Authenticator::USER_UUID_KEY)
            .and_then(|m| m.to_str().ok())
            .and_then(|m| Uuid::from_str(m).ok());
        let auth_token = metadata
            .get(Authenticator::AUTH_TOKEN_KEY)
            .and_then(|m| m.to_str().ok())
            .and_then(|m| AuthToken::from_str(m).ok());

        match (user_uuid, auth_token) {
            (Some(user_uuid), Some(auth_token)) => Ok(AuthPair {
                user_uuid: Some(user_uuid.into()),
                token: Some(auth_token.into()),
            }),
            _ => Err(error),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Authenticator {
    user_repo: Repo,
}

impl Authenticator {
    pub fn new(user_repo: Repo) -> Self {
        Self { user_repo }
    }

    pub const USER_UUID_KEY: &'static str = "user_uuid";
    pub const AUTH_TOKEN_KEY: &'static str = "auth_token";
}

impl Interceptor for Authenticator {
    #[allow(clippy::significant_drop_tightening)]
    fn call(&mut self, request: Request<()>) -> Result<Request<()>, Status> {
        let auth_pair = request.get_auth_pair().map_err(|_| unauthenticated())?;

        // HACK: This is a horrible, but also the only possible way to .await
        // an asynchronous tokio::sync::Mutex inside of a non-async function.
        //
        // See https://stackoverflow.com/questions/66035290 for information.
        let _ = Handle::current().enter();
        let user_repo = futures::executor::block_on(self.user_repo.lock());
        let matching_user = user_repo.iter().find(|user| {
            Some(user.uuid.into()) == auth_pair.user_uuid
                && Some(user.auth_token.as_str())
                    == auth_pair.token.as_ref().map(|t| t.to_string()).as_deref()
        });

        match matching_user {
            Some(user) => {
                let username = &user.username;
                tracing::debug!(message = "Authenticated request", ?username);
                Ok(request)
            }
            None => Err(unauthenticated()),
        }
    }
}

fn unauthenticated() -> Status {
    tracing::warn!(message = "Interceptor caught an unauthenticated request!");
    Status::unauthenticated("The UUID+token pair was invalid or not provided in request metadata")
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::AuthenticatedRequest;
    use crate::{entities::User, proto::AuthPair};
    use rand_chacha::ChaCha20Rng;
    use rand_core::{OsRng, RngCore, SeedableRng};
    use tonic::Request;

    #[test]
    fn authenticated_request_roundtrip() {
        let mut rng = ChaCha20Rng::seed_from_u64(OsRng.next_u64());
        let user = User::new("user_1".into(), "pass_1".into(), &mut rng);
        let auth_pair = AuthPair {
            user_uuid: Some(user.uuid.into()),
            token: Some(user.proto_token()),
        };

        let mut request = Request::new(());
        assert!(request.add_auth_pair(auth_pair.clone()).is_ok());
        assert_eq!(request.get_auth_pair().unwrap(), auth_pair);
    }
}
