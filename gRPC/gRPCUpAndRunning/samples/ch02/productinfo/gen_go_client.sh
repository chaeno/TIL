#!/bin/bash
protoc -I proto/ proto/product_info.proto --go_out=plugins=grpc:go/client/ecommerce
