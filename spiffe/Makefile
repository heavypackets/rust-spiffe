.PHONY: protobuf test spiffe

default: spiffe
	
spiffe:
	cargo build --color always

protobuf:
	protoc --rust_out=src/api/ --grpc_out=src/api/ --plugin=protoc-gen-grpc=`which grpc_rust_plugin` src/api/workload_api.proto

test:
	cargo test --color always
