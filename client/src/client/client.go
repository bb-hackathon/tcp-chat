package main

import (
	"context"
	"log"

	pb "bb-hackathon/tcp-chat.git/proto" // Замените на путь к сгенерированному пакету

	"google.golang.org/grpc"
)

func main() {
	conn, err := grpc.Dial("localhost:50051", grpc.WithInsecure())
	if err != nil {
		log.Fatalf("did not connect: %v", err)
	}
	defer conn.Close()

	client := pb.NewRoomManagerClient(conn)

	ctx := context.Background()
	room := &pb.Room{
		Uuid: &pb.UUID{Uuid: "some-id"},
		Users: []*pb.User{
			{
				Uuid:     &pb.UUID{Uuid: "user-uuid-1"},
				Nickname: "nickname1",
			},
			{
				Uuid:     &pb.UUID{Uuid: "user-uuid-2"},
				Nickname: "nickname2",
			},
		},
	}

	response, err := client.Create(ctx, room)
	if err != nil {
		log.Fatalf("could not create room: %v", err)
	}
	log.Printf("Created room with UUID: %s", response.Uuid) // Исправлено здесь
}
