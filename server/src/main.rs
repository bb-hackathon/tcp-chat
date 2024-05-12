use tcp_chat::TCPChat;

#[tokio::main]
async fn main() {
    TCPChat::preflight();
    let chat = TCPChat::default();
    chat.run().await;
}
