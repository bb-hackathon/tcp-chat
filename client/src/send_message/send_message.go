package sendmessage

import (
	"context"
	"crypto/tls"
	"crypto/x509"
	"fmt"
	"log"
	"os"
	"sync"

	proto "bb-hackathon/tcp-chat.git/proto"

	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials"
	"google.golang.org/grpc/metadata"
	"google.golang.org/protobuf/types/known/emptypb"
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
	address := "luna:9001"
	caCert := "./ca.pem"

	b, err := os.ReadFile(caCert)
	if err != nil {
		log.Fatalf("error reading %s: %v", caCert, err)
	}
	pool := x509.NewCertPool()
	pool.AppendCertsFromPEM(b)

	tc := credentials.NewTLS(&tls.Config{
		RootCAs:            pool,
		InsecureSkipVerify: true,
	})

	conn, err := grpc.Dial(address,
		grpc.WithTransportCredentials(tc),
	)
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
	address := "luna:9001"
	caCert := "./ca.pem"

	b, err := os.ReadFile(caCert)
	if err != nil {
		log.Fatalf("error reading %s: %v", caCert, err)
	}
	pool := x509.NewCertPool()
	pool.AppendCertsFromPEM(b)

	tc := credentials.NewTLS(&tls.Config{
		RootCAs:            pool,
		InsecureSkipVerify: true,
	})

	conn, err := grpc.Dial(address,
		grpc.WithTransportCredentials(tc),
	)
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

	fmt.Println(response.UserUuid)

	setUserAuthData(response.UserUuid.Uuid, response.Token.Token)

	log.Printf("Logged in successfully")
}

func SendMessage(t string, room string) {
	userUUID, authToken := getUserAuthData()

	address := "luna:9001"
	caCert := "./ca.pem"

	b, err := os.ReadFile(caCert)
	if err != nil {
		log.Fatalf("error reading %s: %v", caCert, err)
	}
	pool := x509.NewCertPool()
	pool.AppendCertsFromPEM(b)

	tc := credentials.NewTLS(&tls.Config{
		RootCAs:            pool,
		InsecureSkipVerify: true,
	})

	conn, err := grpc.Dial(address,
		grpc.WithTransportCredentials(tc),
	)
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

func ReceiveMessage(room string) {
	userUUID, authToken := getUserAuthData()

	address := "luna:9001"
	caCert := "./ca.pem"

	b, err := os.ReadFile(caCert)
	if err != nil {
		log.Fatalf("error reading %s: %v", caCert, err)
	}
	pool := x509.NewCertPool()
	pool.AppendCertsFromPEM(b)

	tc := credentials.NewTLS(&tls.Config{
		RootCAs:            pool,
		InsecureSkipVerify: true,
	})

	conn, err := grpc.Dial(address,
		grpc.WithTransportCredentials(tc),
	)
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

	roomUUID := room
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

func SubscribeToUser() {
	UserUUID, AuthToken := getUserAuthData()

	address := "luna:9001"
	caCert := "./ca.pem"

	b, err := os.ReadFile(caCert)
	if err != nil {
		log.Fatalf("error reading %s: %v", caCert, err)
	}
	pool := x509.NewCertPool()
	pool.AppendCertsFromPEM(b)

	tc := credentials.NewTLS(&tls.Config{
		RootCAs:            pool,
		InsecureSkipVerify: true,
	})

	conn, err := grpc.Dial(address,
		grpc.WithTransportCredentials(tc),
	)
	if err != nil {
		log.Fatalf("Failed to connect: %v", err)
	}
	defer conn.Close()

	client := proto.NewChatClient(conn)

	md := metadata.Pairs(
		"user_uuid", UserUUID,
		"auth_token", AuthToken,
	)

	ctx := metadata.NewOutgoingContext(context.Background(), md)

	stream, err := client.SubscribeToUser(ctx, &emptypb.Empty{})
	if err != nil {
		log.Fatalf("SubscribeToUser failed: %v", err)
	}

	for {
		event, err := stream.Recv()
		if err != nil {
			log.Fatalf("Error receiving event: %v", err)
		}

		log.Printf("Received user event: %v", event)
	}
}

func CreateRoom(uuids []string, room string) {
	UserUUID, AuthToken := getUserAuthData()

	address := "luna:9001"
	caCert := "./ca.pem"

	b, err := os.ReadFile(caCert)
	if err != nil {
		log.Fatalf("error reading %s: %v", caCert, err)
	}
	pool := x509.NewCertPool()
	pool.AppendCertsFromPEM(b)

	tc := credentials.NewTLS(&tls.Config{
		RootCAs:            pool,
		InsecureSkipVerify: true,
	})

	conn, err := grpc.Dial(address,
		grpc.WithTransportCredentials(tc),
	)
	if err != nil {
		log.Fatalf("did not connect: %v", err)
	}
	defer conn.Close()

	client := proto.NewChatClient(conn)

	ctx := metadata.NewOutgoingContext(context.Background(), metadata.Pairs(
		"user_uuid", UserUUID,
		"auth_token", AuthToken,
	))

	roomReq := &proto.ClientsideRoom{
		Name:    room,
		Members: []*proto.UUID{},
	}

	currentUserUUID := &proto.UUID{Uuid: UserUUID}
	roomReq.Members = append(roomReq.Members, currentUserUUID)

	for _, uuid := range uuids {
		roomReq.Members = append(roomReq.Members, &proto.UUID{Uuid: uuid})
	}

	response, err := client.CreateRoom(ctx, roomReq)
	fmt.Println(response.Uuid)
	if err != nil {
		log.Fatalf("CreateRoom failed: %v", err)
	} else {
		log.Println("Room created successfully")
		fmt.Println(response)
	}
}

func ListRooms() map[string]string {
	UserUUID, AuthToken := getUserAuthData()

	address := "luna:9001"
	caCert := "./ca.pem"

	b, err := os.ReadFile(caCert)
	if err != nil {
		log.Fatalf("error reading %s: %v", caCert, err)
	}
	pool := x509.NewCertPool()
	pool.AppendCertsFromPEM(b)

	tc := credentials.NewTLS(&tls.Config{
		RootCAs:            pool,
		InsecureSkipVerify: true,
	})

	conn, err := grpc.Dial(address,
		grpc.WithTransportCredentials(tc),
	)
	if err != nil {
		log.Fatalf("Failed to connect: %v", err)
	}
	defer conn.Close()

	client := proto.NewChatClient(conn)

	md := metadata.Pairs(
		"user_uuid", UserUUID,
		"auth_token", AuthToken,
	)

	ctx := metadata.NewOutgoingContext(context.Background(), md)

	response, err := client.ListRooms(ctx, &emptypb.Empty{})
	if err != nil {
		log.Fatalf("Error calling ListRooms: %v", err)
	}
	var result = map[string]string{}
	log.Println("Rooms:")
	for _, room := range response.GetRooms() {
		log.Printf("Room UUID: %s, Room Name: %s", room.GetUuid(), room.GetName())
		result[room.Uuid.Uuid] = room.Name
	}

	return result
}

func ListMessages(room string) {
	UserUUID, AuthToken := getUserAuthData()

	address := "luna:9001"
	caCert := "./ca.pem"

	b, err := os.ReadFile(caCert)
	if err != nil {
		log.Fatalf("error reading %s: %v", caCert, err)
	}
	pool := x509.NewCertPool()
	pool.AppendCertsFromPEM(b)

	tc := credentials.NewTLS(&tls.Config{
		RootCAs:            pool,
		InsecureSkipVerify: true,
	})

	conn, err := grpc.Dial(address,
		grpc.WithTransportCredentials(tc),
	)
	if err != nil {
		log.Fatalf("Failed to connect: %v", err)
	}
	defer conn.Close()

	client := proto.NewChatClient(conn)

	md := metadata.Pairs(
		"user_uuid", UserUUID,
		"auth_token", AuthToken,
	)

	ctx := metadata.NewOutgoingContext(context.Background(), md)

	roomUUID := room

	listMessagesRequest := &proto.UUID{Uuid: roomUUID}
	listMessagesResponse, err := client.ListMessages(ctx, listMessagesRequest)
	if err != nil {
		log.Fatalf("Failed to list messages: %v", err)
	}

	for _, message := range listMessagesResponse.Messages {
		fmt.Printf("Message ID: %s\n", message.Text)
		fmt.Println("-----------------------")
	}
}

func LookUpUser(user string) string {
	UserUUID, AuthToken := getUserAuthData()

	address := "luna:9001"
	caCert := "./ca.pem"

	b, err := os.ReadFile(caCert)
	if err != nil {
		log.Fatalf("error reading %s: %v", caCert, err)
	}
	pool := x509.NewCertPool()
	pool.AppendCertsFromPEM(b)

	tc := credentials.NewTLS(&tls.Config{
		RootCAs:            pool,
		InsecureSkipVerify: true,
	})

	conn, err := grpc.Dial(address,
		grpc.WithTransportCredentials(tc),
	)
	if err != nil {
		log.Fatalf("Failed to connect: %v", err)
	}
	defer conn.Close()

	client := proto.NewChatClient(conn)

	md := metadata.Pairs(
		"user_uuid", UserUUID,
		"auth_token", AuthToken,
	)

	ctx := metadata.NewOutgoingContext(context.Background(), md)

	userIdentifier := user

	lookupUserRequest := &proto.UserLookupRequest{
		Identifier: &proto.UserLookupRequest_Username{
			Username: userIdentifier,
		},
	}
	lookupUserResponse, err := client.LookupUser(ctx, lookupUserRequest)
	if err != nil {
		log.Fatalf("Failed to lookup user: %v", err)
	}

	fmt.Printf("Username: %s\n", lookupUserResponse.Username)
	fmt.Printf("User ID: %s\n", lookupUserResponse.Uuid)
	return lookupUserResponse.Uuid.Uuid
}
