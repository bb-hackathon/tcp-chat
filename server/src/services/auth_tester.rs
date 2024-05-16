use crate::proto;
use tonic::{Request, Response, Status};

#[derive(Debug, Default)]
pub struct AuthenticationTester {}

impl AuthenticationTester {
    #[must_use]
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
