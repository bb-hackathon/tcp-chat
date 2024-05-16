use tcp_chat::persistence::create_connection_pool;

#[test]
fn establish_connection_pool() {
    let pool = create_connection_pool();
    assert!(pool.is_ok());
}
