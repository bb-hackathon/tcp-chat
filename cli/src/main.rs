use promkit::preset::{password::Password, readline::Readline};
use std::panic;
use tcp_chat::auth::AuthenticatedRequest;
use tcp_chat::proto::chat_client::ChatClient;
use tcp_chat::proto::registry_client::RegistryClient;
use tcp_chat::proto::{UserCredentials, UserUuidLookupRequest};
use tonic::Request;

#[tokio::main]
async fn main() {
    let _ = color_eyre::install();
    let eyre_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic_info| {
        crossterm::terminal::disable_raw_mode().unwrap();
        eyre_hook(panic_info);
    }));

    let mut registry = RegistryClient::connect("http://localhost:9001")
        .await
        .unwrap();

    let mut username_prompt = Readline::default()
        .title("Enter a username:")
        .prompt()
        .unwrap();
    let username = username_prompt.run().unwrap();

    let mut password_prompt = Password::default()
        .title("Enter your password:")
        .validator(
            |text| 4 < text.len() && text.len() < 10,
            |text| format!("Length must be over 4 and within 10 but got {}", text.len()),
        )
        .prompt()
        .unwrap();
    let password = password_prompt.run().unwrap();

    let credentials = UserCredentials {
        username: username.clone(),
        password,
    };
    registry
        .register_new_user(credentials.clone())
        .await
        .unwrap();

    crossterm::terminal::disable_raw_mode().unwrap();

    let auth_pair = registry
        .login_as_user(credentials)
        .await
        .unwrap()
        .into_inner();
    dbg!(&auth_pair);

    let mut chat = ChatClient::connect("http://localhost:9001").await.unwrap();

    let mut req = Request::new(UserUuidLookupRequest { username });
    req.add_auth_pair(auth_pair.clone()).unwrap();
    let result = chat.lookup_user(req).await;
    let _ = dbg!(result);
}
