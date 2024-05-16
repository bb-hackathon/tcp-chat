// use futures::StreamExt;
// use tcp_chat::auth::AuthenticatedRequest;
// use tcp_chat::proto::{chat_client::ChatClient, registry_client::RegistryClient, UserCredentials};
// use tonic::Request;

mod common;

// #[tokio::test]
// async fn server_events() {
//     tokio::spawn(common::start_test_server());
//     let mut registry_client = RegistryClient::connect("http://localhost:9001")
//         .await
//         .unwrap();

//     let (_, auth_pair) = common::register_with_credentials(
//         UserCredentials {
//             username: "username".into(),
//             password: "pwd".into(),
//         },
//         &mut registry_client,
//     )
//     .await;

//     let mut chat_client = ChatClient::connect("http://localhost:9001").await.unwrap();
//     let mut request = Request::new(());
//     request.add_auth_pair(auth_pair).unwrap();

//     let server_event_stream = chat_client
//         .server_events(request)
//         .await
//         .unwrap()
//         .into_inner();

//     let tmp = server_event_stream.take(10).collect::<Vec<_>>().await;
//     assert_eq!(tmp.len(), 10);
// }
