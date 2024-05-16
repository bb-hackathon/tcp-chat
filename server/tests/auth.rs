#![allow(clippy::significant_drop_tightening)]

mod common;

use common::start_test_server;
use tcp_chat::auth::{AuthenticatedRequest, Authenticator};
use tcp_chat::entities::AuthToken;
use tcp_chat::proto::authentication_tester_client::AuthenticationTesterClient;
use tcp_chat::proto::registry_client::RegistryClient;
use tcp_chat::proto::UserCredentials;
use tonic::Request;
use uuid::Uuid;

#[tokio::test]
async fn authentication() {
    tokio::spawn(start_test_server());

    let mut registry_client = RegistryClient::connect("http://localhost:9001")
        .await
        .unwrap();

    let (_, auth_pair) = common::register_with_credentials(
        UserCredentials {
            username: "user_1".into(),
            password: "pass_1".into(),
        },
        &mut registry_client,
    )
    .await;

    let mut request = Request::new(());
    request.add_auth_pair(auth_pair).unwrap();

    let mut auth_client = AuthenticationTesterClient::connect("http://localhost:9001")
        .await
        .unwrap();
    assert!(auth_client.test_authentication(request).await.is_ok());
}

#[tokio::test]
async fn authentication_fail_no_token() {
    tokio::spawn(start_test_server());

    let mut registry_client = RegistryClient::connect("http://localhost:9001")
        .await
        .unwrap();

    let (_, auth_pair) = common::register_with_credentials(
        UserCredentials {
            username: "user_1".into(),
            password: "pass_1".into(),
        },
        &mut registry_client,
    )
    .await;

    let user_uuid: Uuid = auth_pair.user_uuid.unwrap_or_default().try_into().unwrap();
    let user_uuid = user_uuid.to_string().parse().unwrap();
    let mut request = Request::new(());
    request
        .metadata_mut()
        .insert(Authenticator::USER_UUID_KEY, user_uuid);

    let mut auth_client = AuthenticationTesterClient::connect("http://localhost:9001")
        .await
        .unwrap();
    assert!(auth_client.test_authentication(request).await.is_err());
}

#[tokio::test]
async fn authentication_fail_no_uuid() {
    tokio::spawn(start_test_server());

    let mut registry_client = RegistryClient::connect("http://localhost:9001")
        .await
        .unwrap();

    let (_, auth_pair) = common::register_with_credentials(
        UserCredentials {
            username: "user_1".into(),
            password: "pass_1".into(),
        },
        &mut registry_client,
    )
    .await;

    let auth_token: AuthToken = auth_pair.token.unwrap_or_default().try_into().unwrap();
    let auth_token = auth_token.to_string().parse().unwrap();
    let mut request = Request::new(());
    request
        .metadata_mut()
        .insert(Authenticator::AUTH_TOKEN_KEY, auth_token);

    let mut auth_client = AuthenticationTesterClient::connect("http://localhost:9001")
        .await
        .unwrap();
    assert!(auth_client.test_authentication(request).await.is_err());
}
