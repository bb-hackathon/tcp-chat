package main

import (
	"context"
	"log"

	"google.golang.org/grpc"
	"google.golang.org/grpc/metadata"

	proto "bb-hackathon/tcp-chat.git/proto"
)

func main() {
	conn, err := grpc.Dial("luna:9001", grpc.WithInsecure())
	if err != nil {
		log.Fatalf("Failed to connect: %v", err)
	}
	defer conn.Close()

	client := proto.NewChatClient(conn)

	md := metadata.Pairs(
		"user_uuid", "1e99a43a-9111-4a38-8c91-adeba3666729",
		"auth_token", "970112691aef145c6f35f18e9586864d",
	)

	ctx := metadata.NewOutgoingContext(context.Background(), md)

	roomUUID := "0cc2800c-21b5-4959-a307-40e8ce6767a8"
	stream, err := client.SubscribeToRoom(ctx, &proto.UUID{Uuid: roomUUID})
	if err != nil {
		log.Fatalf("SubscribeToRoom failed: %v", err)
	}

	for {
		event, err := stream.Recv()
		if err != nil {
			log.Fatalf("Error receiving event: %v", err)
		}
		log.Printf("Received event: %v", event)
	}
}
