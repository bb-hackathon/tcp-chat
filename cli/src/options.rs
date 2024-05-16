use clap::{Parser, ValueEnum};

#[derive(Debug, Clone, Parser)]
pub struct Options {
    #[arg(short, long)]
    action: Action,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum Action {
    Register,
    Login,
    LookupUser,
    CreatePrivateRoom,
    SendMessage,
}

impl Options {
    pub fn action(&self) -> &Action {
        &self.action
    }
}
