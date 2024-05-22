package main

import (
	"context"
	"fmt"
	"log"

	proto "bb-hackathon/tcp-chat.git/proto"

	"google.golang.org/grpc"
	"google.golang.org/grpc/metadata"
)

func main() {
	conn, err := grpc.Dial("luna:9001", grpc.WithInsecure())
	if err != nil {
		log.Fatalf("did not connect: %v", err)
	}
	defer conn.Close()

	client := proto.NewChatClient(conn)

	ctx := metadata.NewOutgoingContext(context.Background(), metadata.Pairs(
		"user_uuid", "17c4ab93-0b16-4233-9936-9f1c89ab96d7",
		"auth_token", "70ea0d70360f118b4de4dd7d2b40c543",
	))

	roomReq := &proto.ClientsideRoom{
		Name: "Room0",
		Members: []*proto.UUID{
			&proto.UUID{Uuid: "17c4ab93-0b16-4233-9936-9f1c89ab96d7"},
		},
	}

	response, err := client.CreateRoom(ctx, roomReq)
	if err != nil {
		log.Fatalf("CreateRoom failed: %v", err)
	} else {
		log.Println("Room created successfully")
		fmt.Println(response)
	}
}
