mod options;

use crate::options::Options;
use clap::Parser;
use options::Action;
use promkit::preset::{password::Password, readline::Readline};
use std::panic;
use tcp_chat::proto::registry_client::RegistryClient;
use tcp_chat::proto::UserCredentials;

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
        Action::Login => {
            let auth_pair = registry
                .login_as_user(credentials)
                .await
                .unwrap()
                .into_inner();
            crossterm::terminal::disable_raw_mode().unwrap();
            println!("{auth_pair:?}");
        }
    }
}
