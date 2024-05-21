package main

import (
	"context"
	"log"

	"google.golang.org/grpc"
	"google.golang.org/grpc/metadata"

	"bb-hackathon/tcp-chat.git/proto"
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

	messageReq := &proto.ClientsideMessage{
		RoomUuid: &proto.UUID{Uuid: "36537ef0-ee3e-44fa-aaa1-6547e466dd4a"},
		Text:     "Hello, BOBR!",
	}

	_, err = client.SendMessage(ctx, messageReq)
	if err != nil {
		log.Fatalf("SendMessage failed: %v", err)
	} else {
		log.Println("Message sent successfully")
	}
}
