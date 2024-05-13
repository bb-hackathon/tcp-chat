use tcp_chat::proto::room_manager_client::RoomManagerClient;
use tcp_chat::proto::RoomCreationRequest;
use tcp_chat::TCPChat;
use tokio_stream::StreamExt;
use tonic::transport::Channel;
use uuid::Uuid;

async fn start_test_server() {
    TCPChat::preflight();
    let chat = TCPChat::default();
    chat.run().await;
}

async fn create_rooms(
    count: usize,
    client: &mut RoomManagerClient<Channel>,
) -> Vec<(String, Vec<tcp_chat::proto::Uuid>)> {
    let mut pairs = vec![];
    for _ in 0..count {
        let visible_name = String::from("Private chat room");
        let user_uuids = vec![Uuid::new_v4().into(), Uuid::new_v4().into()];
        client
            .create_room(RoomCreationRequest {
                visible_name: visible_name.clone(),
                user_uuids: user_uuids.clone(),
            })
            .await
            .unwrap();
        pairs.push((visible_name, user_uuids));
    }

    pairs
}

#[tokio::test]
async fn create_single_room() {
    tokio::spawn(start_test_server());
    let mut client = RoomManagerClient::connect("http://localhost:9001")
        .await
        .unwrap();

    let pairs = create_rooms(1, &mut client).await;
    let (visible_name, user_uuids) = pairs.first().unwrap();

    let room_stream = client.get_rooms(()).await.unwrap().into_inner();
    let rooms: Vec<_> = room_stream.map(|room| room.unwrap()).collect().await;
    assert_eq!(rooms.len(), 1);

    let room = rooms.first().unwrap();
    assert_eq!(room.visible_name, visible_name.clone());
    assert_eq!(&room.user_uuids, user_uuids);
}

#[tokio::test]
async fn create_multiple_rooms() {
    tokio::spawn(start_test_server());
    let mut client = RoomManagerClient::connect("http://localhost:9001")
        .await
        .unwrap();

    const COUNT: usize = 16;

    let pairs = create_rooms(COUNT, &mut client).await;

    let room_stream = client.get_rooms(()).await.unwrap().into_inner();
    let rooms: Vec<_> = room_stream.map(|room| room.unwrap()).collect().await;
    assert_eq!(rooms.len(), COUNT);

    for n in 0..COUNT {
        let room = rooms.get(n).unwrap();
        let pair = pairs.get(n).unwrap();
        assert_eq!(room.visible_name, pair.0.clone());
        assert_eq!(room.user_uuids, pair.1.clone());
    }
}
