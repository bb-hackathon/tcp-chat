use crate::entities::User;
use crate::proto::AuthPair;
use crate::services::acquire_connection_error_status;
use crate::{entities::token::AuthToken, persistence::ConnectionPool};
use std::str::FromStr;
use tonic::{service::Interceptor, Request, Status};
use uuid::Uuid;

#[allow(clippy::missing_errors_doc)]
pub trait AuthenticatedRequest {
    type Error;

    fn add_auth_pair(&mut self, auth_pair: AuthPair) -> Result<(), Self::Error>;
    fn get_auth_pair(&self) -> Result<AuthPair, Self::Error>;
    fn get_originator_uuid(&self) -> Result<Uuid, Self::Error>;
    fn get_token(&self) -> Result<AuthToken, Self::Error>;
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
        match (self.get_originator_uuid(), self.get_token()) {
            (Ok(user_uuid), Ok(auth_token)) => Ok(AuthPair {
                user_uuid: Some(user_uuid.into()),
                token: Some(auth_token.into()),
            }),
            _ => Err(AuthMetadataError::RetrievalError),
        }
    }

    fn get_originator_uuid(&self) -> Result<Uuid, Self::Error> {
        self.metadata()
            .get(Authenticator::USER_UUID_KEY)
            .and_then(|m| m.to_str().ok())
            .and_then(|m| Uuid::from_str(m).ok())
            .ok_or(AuthMetadataError::RetrievalError)
    }

    fn get_token(&self) -> Result<AuthToken, Self::Error> {
        self.metadata()
            .get(Authenticator::AUTH_TOKEN_KEY)
            .and_then(|m| m.to_str().ok())
            .and_then(|m| AuthToken::from_str(m).ok())
            .ok_or(AuthMetadataError::RetrievalError)
    }
}

#[derive(Debug, Clone)]
pub struct Authenticator {
    persistence_pool: ConnectionPool,
}

impl Authenticator {
    pub fn new(persistence_pool: ConnectionPool) -> Self {
        Self { persistence_pool }
    }

    pub const USER_UUID_KEY: &'static str = "user_uuid";
    pub const AUTH_TOKEN_KEY: &'static str = "auth_token";
}

impl Interceptor for Authenticator {
    fn call(&mut self, mut request: Request<()>) -> Result<Request<()>, Status> {
        let auth_pair: AuthPair = request.get_auth_pair().map_err(|_| unauthenticated())?;
        let user_uuid: Uuid = auth_pair
            .user_uuid
            .and_then(|u| u.try_into().ok())
            .ok_or_else(unauthenticated)?;
        let proto_auth_token: String = auth_pair
            .token
            .map(|t| t.to_string())
            .ok_or_else(unauthenticated)?;

        // WARN: Wipe out the AuthToken from the request's metadata,
        // so it doesn't accidently appear anywhere else (i.e. logs).
        let _ = request.metadata_mut().remove(Self::AUTH_TOKEN_KEY);

        let mut connection = self
            .persistence_pool
            .get()
            .map_err(acquire_connection_error_status)?;

        // Import some traits and methods to interact with the ORM.
        use crate::entities::schema::users::dsl::*;
        use diesel::query_dsl::methods::{FilterDsl, SelectDsl};
        use diesel::{ExpressionMethods, OptionalExtension, RunQueryDsl, SelectableHelper};

        let user_with_matching_credentials = users
            .filter(uuid.eq(user_uuid))
            .filter(auth_token.eq(proto_auth_token))
            .select(User::as_select())
            .first(&mut connection)
            .optional()
            .map_err(|err| Status::internal(err.to_string()))?;

        match user_with_matching_credentials {
            Some(user) => {
                tracing::trace!(message = "Authenticated request", username = ?user.username);
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
    fn auth_pair_roundtrip() {
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
