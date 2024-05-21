package main

import (
	"context"
	"log"

	proto "bb-hackathon/tcp-chat.git/proto"

	"google.golang.org/grpc"
)

func main() {
	conn, err := grpc.Dial("luna:9001", grpc.WithInsecure())
	if err != nil {
		log.Fatalf("did not connect: %v", err)
	}
	defer conn.Close()

	client := proto.NewRegistryClient(conn)

	ctx := context.Background()
	userCredentials := &proto.UserCredentials{
		Username: "user2",
		Password: "password",
	}

	_, err = client.RegisterNewUser(ctx, userCredentials)
	if err != nil {
		log.Fatalf("could not register new user: %v", err)
	}

	log.Printf("Registered new user successfully")
}
