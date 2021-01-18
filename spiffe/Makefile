.PHONY: protobuf test spiffe

default: spiffe
	
spiffe:
	cargo build --color always

protobuf:
	protoc --rust_out=src/workload/ --grpc_out=src/workload/ --plugin=protoc-gen-grpc=`which grpc_rust_plugin` src/workload/workload_api.proto

test:
	cargo test --color always
