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
		"user_uuid", "d9890202-a129-4044-8736-e7770fcae7f5",
		"auth_token", "f94ee0212842c6434f019eb42bfcdc63",
	)

	ctx := metadata.NewOutgoingContext(context.Background(), md)

	roomUUID := "36537ef0-ee3e-44fa-aaa1-6547e466dd4a"
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
