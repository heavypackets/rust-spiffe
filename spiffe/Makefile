.PHONY: protobuf test spiffe

default: spiffe
	
spiffe:
	cargo build --color always

protobuf:
	protoc --rust_out=src/api/ src/api/workload_api.proto
	protoc --rust-grpc_out=src/api/ src/api/workload_api.proto

test:
	cargo test --color always
