package sendmessage

import (
	"context"
	"log"
	"sync"

	proto "bb-hackathon/tcp-chat.git/proto"

	"google.golang.org/grpc"
	"google.golang.org/grpc/metadata"
)

var (
	UserUUID    string
	AuthToken   string
	authDataMux sync.Mutex
)

func setUserAuthData(uuid, token string) {
	authDataMux.Lock()
	defer authDataMux.Unlock()
	UserUUID = uuid
	AuthToken = token
}

func getUserAuthData() (string, string) {
	authDataMux.Lock()
	defer authDataMux.Unlock()
	return UserUUID, AuthToken
}

func Register(username string, password string) {
	conn, err := grpc.Dial("luna:9001", grpc.WithInsecure())
	if err != nil {
		log.Fatalf("did not connect: %v", err)
	}
	defer conn.Close()

	client := proto.NewRegistryClient(conn)

	ctx := context.Background()
	userCredentials := &proto.UserCredentials{
		Username: username,
		Password: password,
	}

	_, err = client.RegisterNewUser(ctx, userCredentials)
	if err != nil {
		log.Fatalf("could not register new user: %v", err)
	}

	log.Printf("Registered new user successfully")
}

func Login(username, password string) {
	conn, err := grpc.Dial("luna:9001", grpc.WithInsecure())
	if err != nil {
		log.Fatalf("did not connect: %v", err)
	}
	defer conn.Close()

	client := proto.NewRegistryClient(conn)

	ctx := context.Background()
	userCredentials := &proto.UserCredentials{
		Username: username,
		Password: password,
	}

	response, err := client.LoginAsUser(ctx, userCredentials)
	if err != nil {
		log.Fatalf("could not login: %v", err)
	}

	setUserAuthData(response.UserUuid.Uuid, response.Token.Token)

	log.Printf("Logged in successfully")
}

func SendMessage(t string, room string) {
	userUUID, authToken := getUserAuthData()

	conn, err := grpc.Dial("luna:9001", grpc.WithInsecure())
	if err != nil {
		log.Fatalf("Failed to connect: %v", err)
	}
	defer conn.Close()

	client := proto.NewChatClient(conn)

	md := metadata.Pairs(
		"user_uuid", userUUID,
		"auth_token", authToken,
	)

	ctx := metadata.NewOutgoingContext(context.Background(), md)
	log.Println(t)
	messageReq := &proto.ClientsideMessage{
		RoomUuid: &proto.UUID{Uuid: room},
		Text:     t,
	}

	_, err = client.SendMessage(ctx, messageReq)
	if err != nil {
		log.Fatalf("SendMessage failed: %v", err)
	} else {
		log.Printf("Message sent successfully: %s", t)
	}
}
