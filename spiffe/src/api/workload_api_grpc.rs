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

const METHOD_SPIFFE_WORKLOAD_API_FETCH_X509_SVID: ::grpcio::Method<super::workload_api::X509SVIDRequest, super::workload_api::X509SVIDResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ServerStreaming,
    name: "/SpiffeWorkloadAPI/FetchX509SVID",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

pub struct SpiffeWorkloadApiClient {
    client: ::grpcio::Client,
}

impl SpiffeWorkloadApiClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        SpiffeWorkloadApiClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn fetch_x509_svid_opt(&self, req: &super::workload_api::X509SVIDRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::workload_api::X509SVIDResponse>> {
        self.client.server_streaming(&METHOD_SPIFFE_WORKLOAD_API_FETCH_X509_SVID, req, opt)
    }

    pub fn fetch_x509_svid(&self, req: &super::workload_api::X509SVIDRequest) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::workload_api::X509SVIDResponse>> {
        self.fetch_x509_svid_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait SpiffeWorkloadApi {
    fn fetch_x509_svid(&self, ctx: ::grpcio::RpcContext, req: super::workload_api::X509SVIDRequest, sink: ::grpcio::ServerStreamingSink<super::workload_api::X509SVIDResponse>);
}

pub fn create_spiffe_workload_api<S: SpiffeWorkloadApi + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let instance = s.clone();
    builder = builder.add_server_streaming_handler(&METHOD_SPIFFE_WORKLOAD_API_FETCH_X509_SVID, move |ctx, req, resp| {
        instance.fetch_x509_svid(ctx, req, resp)
    });
    builder.build()
}
