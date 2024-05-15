#![allow(unused)]
#![allow(clippy::missing_panics_doc)]

use tcp_chat::proto::{registry_client::RegistryClient, AuthPair, UserCredentials};
use tcp_chat::TCPChat;
use tonic::transport::Channel;

pub async fn start_test_server() {
    TCPChat::preflight();
    let chat = TCPChat::default();
    chat.run().await;
}

pub async fn register_with_credentials(
    credentials: UserCredentials,
    registry_client: &mut RegistryClient<Channel>,
) -> (UserCredentials, AuthPair) {
    registry_client
        .register_new_user(credentials.clone())
        .await
        .unwrap();
    let auth_pair = registry_client
        .login_as_user(credentials.clone())
        .await
        .unwrap()
        .into_inner();

    (credentials, auth_pair)
}
