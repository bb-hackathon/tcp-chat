#![allow(clippy::significant_drop_tightening)]

use tcp_chat::proto::{registry_client::RegistryClient, UserCredentials};
use tcp_chat::TCPChat;

async fn start_test_server() {
    TCPChat::preflight();
    let chat = TCPChat::default();
    chat.run().await;
}

#[tokio::test]
async fn register() {
    tokio::spawn(start_test_server());
    let mut client = RegistryClient::connect("http://localhost:9001")
        .await
        .unwrap();

    let credentials = UserCredentials {
        username: "user_1".into(),
        password: "pass_1".into(),
    };
    assert!(client.register_new_user(credentials).await.is_ok());
}

#[tokio::test]
async fn login() {
    tokio::spawn(start_test_server());
    let mut client = RegistryClient::connect("http://localhost:9001")
        .await
        .unwrap();

    let credentials = UserCredentials {
        username: "user_1".into(),
        password: "pass_1".into(),
    };
    assert!(client.register_new_user(credentials.clone()).await.is_ok());

    let auth_pair = client
        .login_as_user(credentials)
        .await
        .unwrap()
        .into_inner();
    assert!(auth_pair.user_uuid.is_some());
    assert!(auth_pair.token.is_some());
}
