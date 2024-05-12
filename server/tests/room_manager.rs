use tcp_chat::proto::room_manager_client::RoomManagerClient;
use tcp_chat::proto::{Room, User, Uuid};
use tcp_chat::TCPChat;

async fn start_test_server() {
    TCPChat::preflight();
    let chat = TCPChat::default();
    chat.run().await;
}

fn mkroom<T: ToString>(uuid: T) -> Room {
    Room {
        uuid: Some(Uuid {
            uuid: uuid.to_string(),
        }),
        users: vec![
            User {
                uuid: Some(Uuid { uuid: "1".into() }),
                nickname: "user_1".into(),
            },
            User {
                uuid: Some(Uuid { uuid: "2".into() }),
                nickname: "user_2".into(),
            },
            User {
                uuid: Some(Uuid { uuid: "3".into() }),
                nickname: "user_3".into(),
            },
        ],
    }
}

#[tokio::test]
async fn create_room() {
    tokio::spawn(start_test_server());
    let mut client = RoomManagerClient::connect("http://localhost:9001")
        .await
        .unwrap();

    let rooms = vec![mkroom(1), mkroom(2), mkroom(3)];
    let room_count = rooms.len();

    for r in rooms {
        client.create(r).await.unwrap();
    }

    assert_eq!(
        client.get_room_count(()).await.unwrap().into_inner().count,
        room_count as u32
    );
}
