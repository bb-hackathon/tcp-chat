use color_eyre::owo_colors::OwoColorize;
use promkit::preset::{listbox::Listbox, password::Password, readline::Readline};
use std::panic;
use std::str::FromStr;
use tcp_chat::auth::AuthenticatedRequest;
use tcp_chat::proto::chat_client::ChatClient;
use tcp_chat::proto::registry_client::RegistryClient;
use tcp_chat::proto::serverside_room_event::Event;
use tcp_chat::proto::{self, AuthPair, ClientsideMessage, ServersideMessage, UserCredentials};
use tokio_stream::StreamExt;
use tonic::transport::Channel;
use uuid::Uuid;

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
        .title("Password")
        .prompt()
        .unwrap()
        .run()
        .unwrap();

    let mut registry = RegistryClient::connect("http://localhost:9001")
        .await
        .unwrap();

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
        .connect()
        .await
        .unwrap();
    let mut chat = ChatClient::with_interceptor(chat, move |mut request: tonic::Request<()>| {
        request.add_auth_pair(auth_pair.clone()).unwrap();
        Ok(request)
    });

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

    let mut message_stream = chat
        .subscribe_to_room(proto::Uuid::from(chosen_room))
        .await
        .unwrap()
        .into_inner();

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
            let messages = chat
                .list_messages(Into::<proto::Uuid>::into(chosen_room))
                .await
                .unwrap()
                .into_inner()
                .messages;

            for msg in messages.into_iter() {
                print_message(msg);
            }

            while let Ok(event) = message_stream.next().await.unwrap() {
                match event.event.unwrap() {
                    Event::NewMessage(msg) => print_message(msg),
                }
            }
        }

        _ => unreachable!(),
    }
}

fn print_message(msg: ServersideMessage) {
    println!(
        "{} | {}: {}",
        msg.timestamp.unwrap().blue(),
        msg.sender_uuid.unwrap().uuid.green(),
        msg.text
    );
}
