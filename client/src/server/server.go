package main

import (
	"context"
	"log"
	"net"

	pb "bb-hackathon/tcp-chat.git/proto" // Замените на путь к сгенерированному пакету

	"google.golang.org/grpc"
)

type server struct {
	pb.UnimplementedRoomManagerServer
}

func (s *server) Create(ctx context.Context, in *pb.Room) (*pb.UUID, error) {
	log.Printf("Creating room with UUID: %s", in.GetUuid().GetUuid())

	return in.GetUuid(), nil
}

func main() {
	lis, err := net.Listen("tcp", ":50051") // Замените на нужный порт
	if err != nil {
		log.Fatalf("failed to listen: %v", err)
	}
	s := grpc.NewServer()
	pb.RegisterRoomManagerServer(s, &server{})

	log.Printf("Server listening at %v", lis.Addr())
	if err := s.Serve(lis); err != nil {
		log.Fatalf("failed to serve: %v", err)
	}
}
