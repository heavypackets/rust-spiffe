// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]


// interface

pub trait SpiffeWorkloadAPI {
    fn fetch_x509_svid(&self, o: ::grpc::RequestOptions, p: super::workload_api::X509SVIDRequest) -> ::grpc::StreamingResponse<super::workload_api::X509SVIDResponse>;
}

// client

pub struct SpiffeWorkloadAPIClient {
    grpc_client: ::grpc::Client,
    method_FetchX509SVID: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::workload_api::X509SVIDRequest, super::workload_api::X509SVIDResponse>>,
}

impl SpiffeWorkloadAPIClient {
    pub fn with_client(grpc_client: ::grpc::Client) -> Self {
        SpiffeWorkloadAPIClient {
            grpc_client: grpc_client,
            method_FetchX509SVID: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/SpiffeWorkloadAPI/FetchX509SVID".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::ServerStreaming,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
        }
    }

    pub fn new_plain(host: &str, port: u16, conf: ::grpc::ClientConf) -> ::grpc::Result<Self> {
        ::grpc::Client::new_plain(host, port, conf).map(|c| {
            SpiffeWorkloadAPIClient::with_client(c)
        })
    }
    pub fn new_tls<C : ::tls_api::TlsConnector>(host: &str, port: u16, conf: ::grpc::ClientConf) -> ::grpc::Result<Self> {
        ::grpc::Client::new_tls::<C>(host, port, conf).map(|c| {
            SpiffeWorkloadAPIClient::with_client(c)
        })
    }
}

impl SpiffeWorkloadAPI for SpiffeWorkloadAPIClient {
    fn fetch_x509_svid(&self, o: ::grpc::RequestOptions, p: super::workload_api::X509SVIDRequest) -> ::grpc::StreamingResponse<super::workload_api::X509SVIDResponse> {
        self.grpc_client.call_server_streaming(o, p, self.method_FetchX509SVID.clone())
    }
}

// server

pub struct SpiffeWorkloadAPIServer;


impl SpiffeWorkloadAPIServer {
    pub fn new_service_def<H : SpiffeWorkloadAPI + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::rt::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::rt::ServerServiceDefinition::new("/SpiffeWorkloadAPI",
            vec![
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/SpiffeWorkloadAPI/FetchX509SVID".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::ServerStreaming,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerServerStreaming::new(move |o, p| handler_copy.fetch_x509_svid(o, p))
                    },
                ),
            ],
        )
    }
}
