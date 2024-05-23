use color_eyre::owo_colors::OwoColorize;
use promkit::preset::{listbox::Listbox, password::Password, readline::Readline};
use std::{panic, str::FromStr};
use tcp_chat::proto::{chat_client::ChatClient, registry_client::RegistryClient};
use tcp_chat::proto::{serverside_room_event::Event, user_lookup_request::Identifier};
use tcp_chat::proto::{AuthPair, ClientsideMessage, ServersideMessage};
use tcp_chat::proto::{RoomWithUserCreationRequest, UserCredentials, UserLookupRequest};
use tcp_chat::{auth::AuthenticatedRequest, proto};
use tokio_stream::StreamExt;
use tonic::service::interceptor::InterceptedService;
use tonic::transport::{Certificate, ClientTlsConfig};
use tonic::{transport::Channel, Request, Status};
use uuid::Uuid;

const CERT: &str = include_str!("../../tls/ca.pem");

#[tokio::main]
async fn main() {
    let _ = color_eyre::install();
    let eyre_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic_info| {
        crossterm::terminal::disable_raw_mode().unwrap();
        eyre_hook(panic_info);
    }));

    let auth_strategy = Listbox::new(["Login", "Register"])
        .title("Would you like to log in or register?")
        .prompt()
        .unwrap()
        .run()
        .unwrap();

    let username = Readline::default()
        .title("Username:")
        .prompt()
        .unwrap()
        .run()
        .unwrap();

    let password = Password::default()
        .title("Password:")
        .prompt()
        .unwrap()
        .run()
        .unwrap();

    let channel = Channel::from_static("https://localhost:9001")
        .tls_config(
            ClientTlsConfig::new()
                .ca_certificate(Certificate::from_pem(CERT))
                .domain_name("example.com"),
        )
        .unwrap()
        .connect()
        .await
        .unwrap();

    let mut registry = RegistryClient::new(channel);

    match auth_strategy.as_str() {
        "Login" => {
            let auth_pair = registry
                .login_as_user(UserCredentials { username, password })
                .await
                .unwrap()
                .into_inner();

            list_rooms(auth_pair).await;
        }

        "Register" => {
            registry
                .register_new_user(UserCredentials { username, password })
                .await
                .unwrap();
        }

        _ => unreachable!(),
    }
}

async fn list_rooms(auth_pair: AuthPair) {
    let chat = Channel::from_static("https://localhost:9001")
        .tls_config(
            ClientTlsConfig::new()
                .ca_certificate(Certificate::from_pem(CERT))
                .domain_name("example.com"),
        )
        .unwrap()
        .connect()
        .await
        .unwrap();
    let mut chat = ChatClient::with_interceptor(chat, move |mut request: Request<()>| {
        request.add_auth_pair(auth_pair.clone()).unwrap();
        Ok(request)
    });

    let room_strategy = Listbox::new(["Focus existing room", "Create new private room"])
        .title("What would you like to do?")
        .prompt()
        .unwrap()
        .run()
        .unwrap();

    match room_strategy.as_str() {
        "Focus existing room" => existing_room(chat).await,

        "Create new private room" => {
            let interlocutor = Readline::default()
                .title("Who would you like to chat with?")
                .prompt()
                .unwrap()
                .run()
                .unwrap();

            let interlocutor_uuid = chat
                .lookup_user(UserLookupRequest {
                    identifier: Some(Identifier::Username(interlocutor.clone())),
                })
                .await
                .unwrap()
                .into_inner()
                .uuid
                .unwrap();

            let _ = chat
                .create_room_with_user(RoomWithUserCreationRequest {
                    user_uuid: Some(interlocutor_uuid),
                })
                .await
                .unwrap();

            println!("Created new private room with {}.", interlocutor.purple());

            existing_room(chat).await;
        }
        _ => unreachable!(),
    }
}

async fn existing_room(
    mut chat: ChatClient<
        InterceptedService<Channel, impl Fn(Request<()>) -> Result<Request<()>, Status>>,
    >,
) {
    let rooms = chat.list_rooms(()).await.unwrap().into_inner().rooms;
    let chosen_room = &Listbox::new(&rooms)
        .title("Which room would you like to focus?")
        .prompt()
        .unwrap()
        .run()
        .unwrap();
    let chosen_room = chosen_room
        .split(' ')
        .last()
        .unwrap()
        .trim_matches(|c| c == '(' || c == ')');
    let chosen_room = Uuid::from_str(chosen_room).unwrap();

    let room_action = Listbox::new(["Send new messages", "Listen to messages"])
        .title("Would you like to send new messages or listen to incoming ones?")
        .prompt()
        .unwrap()
        .run()
        .unwrap();

    match room_action.as_str() {
        "Send new messages" => loop {
            let text = Readline::default()
                .title("Message text:")
                .prompt()
                .unwrap()
                .run()
                .unwrap();

            let _ = chat
                .send_message(ClientsideMessage {
                    room_uuid: Some(chosen_room.into()),
                    text,
                })
                .await
                .unwrap();

            println!("{}", "Message sent!".bright_black());
        },

        "Listen to messages" => {
            // Print older, in-database messages.
            let messages = chat
                .list_messages(Into::<proto::Uuid>::into(chosen_room))
                .await
                .unwrap()
                .into_inner()
                .messages;

            for msg in messages.into_iter() {
                print_message(&msg);
            }

            // Print live messages.
            let mut message_stream = chat
                .subscribe_to_room(proto::Uuid::from(chosen_room))
                .await
                .unwrap()
                .into_inner();

            'message_listener: while let Ok(event) = message_stream.next().await.unwrap() {
                match event.event.unwrap() {
                    Event::NewMessage(msg) => {
                        print_message(&msg);
                        if msg.text.as_str() == "exit" {
                            break 'message_listener;
                        }
                    }
                }
            }
        }

        _ => unreachable!(),
    }
}

fn print_message(msg: &ServersideMessage) {
    println!(
        "{} | {}: {}",
        msg.timestamp.clone().unwrap().blue(),
        msg.sender_uuid.clone().unwrap().uuid.green(),
        msg.text
    );
}
