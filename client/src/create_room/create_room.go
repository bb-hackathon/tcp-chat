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
		"user_uuid", "d9890202-a129-4044-8736-e7770fcae7f5",
		"auth_token", "f94ee0212842c6434f019eb42bfcdc63",
	))

	roomReq := &proto.ClientsideRoom{
		Name: "Room2",
		Members: []*proto.UUID{
			&proto.UUID{Uuid: "d9890202-a129-4044-8736-e7770fcae7f5"},
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
