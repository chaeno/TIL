#!/bin/bash
protoc -I proto/ proto/order_management.proto --go_out=plugins=grpc:go/client/ecommerce
protoc -I proto/ proto/order_management.proto --go_out=plugins=grpc:go/server/ecommerce
