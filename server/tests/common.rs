use tcp_chat::TCPChat;

#[allow(unused)]
pub async fn start_test_server() {
    TCPChat::preflight();
    let chat = TCPChat::default();
    chat.run().await;
}
