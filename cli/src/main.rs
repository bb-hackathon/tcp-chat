mod options;

use crate::options::Options;
use clap::Parser;
use options::Action;
use promkit::preset::{password::Password, readline::Readline};
use std::panic;
use tcp_chat::auth::AuthenticatedRequest;
use tcp_chat::proto::chat_client::ChatClient;
use tcp_chat::proto::registry_client::RegistryClient;
use tcp_chat::proto::user_lookup_request::Identifier;
use tcp_chat::proto::{self, ClientsideMessage, RoomWithUserCreationRequest};
use tcp_chat::proto::{UserCredentials, UserLookupRequest};
use tokio_stream::StreamExt;
use tonic::transport::Channel;
use tonic::Request;
use uuid::{uuid, Uuid};

const ROOM_UUID: Uuid = uuid!("b37490a2-4781-44ad-9c22-d25f1a58f228");

#[tokio::main]
async fn main() {
    let _ = color_eyre::install();
    let eyre_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic_info| {
        crossterm::terminal::disable_raw_mode().unwrap();
        eyre_hook(panic_info);
    }));

    let options = Options::parse();

    let mut registry = RegistryClient::connect("http://localhost:9001")
        .await
        .unwrap();

    let mut username_prompt = Readline::default()
        .title("Enter a username:")
        .validator(
            |username| !username.is_empty(),
            |_| "Username must not be empty".to_string(),
        )
        .prompt()
        .unwrap();
    let username = username_prompt.run().unwrap();

    let mut password_prompt = Password::default()
        .title("Enter your password:")
        .validator(
            |password| !password.is_empty(),
            |_| "Password must not be empty".to_string(),
        )
        .prompt()
        .unwrap();
    let password = password_prompt.run().unwrap();
    let credentials = UserCredentials { username, password };

    match options.action() {
        Action::Register => {
            registry
                .register_new_user(credentials.clone())
                .await
                .unwrap();
        }
        action => {
            let auth_pair = registry
                .login_as_user(credentials)
                .await
                .unwrap()
                .into_inner();
            let chat = Channel::from_static("http://localhost:9001")
                .connect()
                .await
                .unwrap();
            let mut chat = ChatClient::with_interceptor(chat, move |mut req: Request<()>| {
                req.add_auth_pair(auth_pair.clone()).unwrap();
                Ok(req)
            });

            match action {
                Action::Login => println!("Successfully authenticated!"),
                Action::LookupUser => {
                    let user = chat
                        .lookup_user(UserLookupRequest {
                            identifier: Some(Identifier::Username(username_prompt.run().unwrap())),
                        })
                        .await
                        .unwrap()
                        .into_inner();
                    println!("{user:?}");
                }
                Action::CreatePrivateRoom => {
                    let interlocutor = chat
                        .lookup_user(UserLookupRequest {
                            identifier: Some(Identifier::Username(username_prompt.run().unwrap())),
                        })
                        .await
                        .unwrap()
                        .into_inner();
                    let room_uuid = chat
                        .create_room_with_user(RoomWithUserCreationRequest {
                            user_uuid: interlocutor.uuid.clone(),
                        })
                        .await
                        .unwrap()
                        .into_inner();
                    println!(
                        "Created private room with {interlocutor:?}, room UUID: {room_uuid:?}"
                    );
                }
                Action::SendMessage => {
                    let mut message_prompt = Readline::default()
                        .title("Enter the message:")
                        .validator(
                            |username| !username.is_empty(),
                            |_| "Username must not be empty".to_string(),
                        )
                        .prompt()
                        .unwrap();
                    chat.send_message(ClientsideMessage {
                        room_uuid: Some(ROOM_UUID.into()),
                        text: message_prompt.run().unwrap(),
                    })
                    .await
                    .unwrap();

                    println!("Message sent!");
                }
                Action::Subscribe => {
                    let mut message_stream = chat
                        .subscribe_to_room(proto::Uuid::from(ROOM_UUID))
                        .await
                        .unwrap()
                        .into_inner();

                    while let Some(event) = message_stream.next().await {
                        println!("{event:?}");
                    }
                }
                Action::Register => unreachable!(),
            }
        }
    }
}
