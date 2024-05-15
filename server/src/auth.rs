use crate::{entities::User, proto};
use std::sync::Arc;
use tokio::{runtime::Handle, sync::Mutex};
use tonic::{service::Interceptor, Request, Response, Status};

type UserRepo = Arc<Mutex<Vec<User>>>;

#[derive(Debug, Default)]
pub struct AuthenticationTester {}

impl AuthenticationTester {
    pub const fn new() -> Self {
        Self {}
    }
}

#[tonic::async_trait]
impl proto::authentication_tester_server::AuthenticationTester for AuthenticationTester {
    #[tracing::instrument(skip(self))]
    async fn test_authentication(&self, _request: Request<()>) -> Result<Response<()>, Status> {
        Ok(Response::new(()))
    }
}

fn unauthenticated() -> Status {
    tracing::warn!(message = "Unauthenticated");
    Status::unauthenticated("The UUID+token pair was invalid or not provided in request metadata")
}

#[derive(Debug, Clone)]
pub struct Authenticator {
    user_repo: UserRepo,
}

impl Authenticator {
    pub fn new(user_repo: UserRepo) -> Self {
        Self { user_repo }
    }
}

impl Interceptor for Authenticator {
    #[allow(clippy::significant_drop_tightening)]
    fn call(&mut self, request: Request<()>) -> Result<Request<()>, Status> {
        let metadata = request.metadata();
        let user_uuid_str = metadata
            .get("user_uuid")
            .ok_or_else(unauthenticated)?
            .to_str()
            .map_err(|_| unauthenticated())?;
        let auth_token_str = metadata
            .get("auth_token")
            .ok_or_else(unauthenticated)?
            .to_str()
            .map_err(|_| unauthenticated())?;

        // HACK: This is a horrible, but also the only possible way to .await
        // an asynchronous tokio::sync::Mutex inside of a non-async function.
        //
        // See https://stackoverflow.com/questions/66035290 for information.
        let _ = Handle::current().enter();
        let user_repo = futures::executor::block_on(self.user_repo.lock());

        let matching_user = user_repo.iter().find(|user| {
            user.uuid().to_string() == user_uuid_str
                && user.auth_token().to_string() == auth_token_str
        });

        match matching_user {
            Some(user) => {
                let username = user.username();
                tracing::info!(message = "Authenticated", ?username);
                Ok(request)
            }
            None => Err(unauthenticated()),
        }
    }
}
