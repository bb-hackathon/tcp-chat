package main

import (
	pb "bb-hackathon/tcp-chat.git/proto"
	form "bb-hackathon/tcp-chat.git/src/form"
	"log"
	"net/http"

	"google.golang.org/grpc"
)

var client pb.RoomManagerClient

func main() {
	// Инициализируем gRPC-клиент
	conn, err := grpc.Dial("localhost:50051", grpc.WithInsecure())
	if err != nil {
		log.Fatalf("did not connect: %v", err)
	}
	defer conn.Close()

	client = pb.NewRoomManagerClient(conn)

	// Настройка обработчиков HTTP
	http.HandleFunc("/", form.IndexHandler)
	http.HandleFunc("/create", form.CreateHandler(client))

	log.Println("Сервер запущен на http://localhost:8081")
	log.Fatal(http.ListenAndServe(":8081", nil))
}
