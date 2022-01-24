package main

import (
	"context"
	pb "github.com/example/gen"
	grpc "google.golang.org/grpc"
	"log"

	"net"
)

func main() {
	lis, _ := net.Listen("tcp", "8080")
	s := grpc.NewServer()

	pb.RegisterProductInfoServer(s)
	if err := s.Serve(lis); err != nil {
		log.Fatalf("failed to serve: %v", err)
	}

}

func (s *server) AddProduct(ctx context.Context, in *pb.Product) (*pb.ProductID, error) {
	// 구현
}

func (s *server) GetProduct(ctx context.Context, in *pb.ProductID) (*pb.Product, error) {
	// 구현
}
