// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

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

const METHOD_SPIFFE_WORKLOAD_API_FETCH_JWTSVID: ::grpcio::Method<super::workload_api::JWTSVIDRequest, super::workload_api::JWTSVIDResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/SpiffeWorkloadAPI/FetchJWTSVID",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_SPIFFE_WORKLOAD_API_FETCH_JWT_BUNDLES: ::grpcio::Method<super::workload_api::JWTBundlesRequest, super::workload_api::JWTBundlesResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ServerStreaming,
    name: "/SpiffeWorkloadAPI/FetchJWTBundles",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_SPIFFE_WORKLOAD_API_VALIDATE_JWTSVID: ::grpcio::Method<super::workload_api::ValidateJWTSVIDRequest, super::workload_api::ValidateJWTSVIDResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/SpiffeWorkloadAPI/ValidateJWTSVID",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_SPIFFE_WORKLOAD_API_FETCH_X509_SVID: ::grpcio::Method<super::workload_api::X509SVIDRequest, super::workload_api::X509SVIDResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ServerStreaming,
    name: "/SpiffeWorkloadAPI/FetchX509SVID",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct SpiffeWorkloadApiClient {
    client: ::grpcio::Client,
}

impl SpiffeWorkloadApiClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        SpiffeWorkloadApiClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn fetch_jwtsvid_opt(&self, req: &super::workload_api::JWTSVIDRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::workload_api::JWTSVIDResponse> {
        self.client.unary_call(&METHOD_SPIFFE_WORKLOAD_API_FETCH_JWTSVID, req, opt)
    }

    pub fn fetch_jwtsvid(&self, req: &super::workload_api::JWTSVIDRequest) -> ::grpcio::Result<super::workload_api::JWTSVIDResponse> {
        self.fetch_jwtsvid_opt(req, ::grpcio::CallOption::default())
    }

    pub fn fetch_jwtsvid_async_opt(&self, req: &super::workload_api::JWTSVIDRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::workload_api::JWTSVIDResponse>> {
        self.client.unary_call_async(&METHOD_SPIFFE_WORKLOAD_API_FETCH_JWTSVID, req, opt)
    }

    pub fn fetch_jwtsvid_async(&self, req: &super::workload_api::JWTSVIDRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::workload_api::JWTSVIDResponse>> {
        self.fetch_jwtsvid_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn fetch_jwt_bundles_opt(&self, req: &super::workload_api::JWTBundlesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::workload_api::JWTBundlesResponse>> {
        self.client.server_streaming(&METHOD_SPIFFE_WORKLOAD_API_FETCH_JWT_BUNDLES, req, opt)
    }

    pub fn fetch_jwt_bundles(&self, req: &super::workload_api::JWTBundlesRequest) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::workload_api::JWTBundlesResponse>> {
        self.fetch_jwt_bundles_opt(req, ::grpcio::CallOption::default())
    }

    pub fn validate_jwtsvid_opt(&self, req: &super::workload_api::ValidateJWTSVIDRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::workload_api::ValidateJWTSVIDResponse> {
        self.client.unary_call(&METHOD_SPIFFE_WORKLOAD_API_VALIDATE_JWTSVID, req, opt)
    }

    pub fn validate_jwtsvid(&self, req: &super::workload_api::ValidateJWTSVIDRequest) -> ::grpcio::Result<super::workload_api::ValidateJWTSVIDResponse> {
        self.validate_jwtsvid_opt(req, ::grpcio::CallOption::default())
    }

    pub fn validate_jwtsvid_async_opt(&self, req: &super::workload_api::ValidateJWTSVIDRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::workload_api::ValidateJWTSVIDResponse>> {
        self.client.unary_call_async(&METHOD_SPIFFE_WORKLOAD_API_VALIDATE_JWTSVID, req, opt)
    }

    pub fn validate_jwtsvid_async(&self, req: &super::workload_api::ValidateJWTSVIDRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::workload_api::ValidateJWTSVIDResponse>> {
        self.validate_jwtsvid_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn fetch_x509_svid_opt(&self, req: &super::workload_api::X509SVIDRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::workload_api::X509SVIDResponse>> {
        self.client.server_streaming(&METHOD_SPIFFE_WORKLOAD_API_FETCH_X509_SVID, req, opt)
    }

    pub fn fetch_x509_svid(&self, req: &super::workload_api::X509SVIDRequest) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::workload_api::X509SVIDResponse>> {
        self.fetch_x509_svid_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Output = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait SpiffeWorkloadApi {
    fn fetch_jwtsvid(&mut self, ctx: ::grpcio::RpcContext, req: super::workload_api::JWTSVIDRequest, sink: ::grpcio::UnarySink<super::workload_api::JWTSVIDResponse>);
    fn fetch_jwt_bundles(&mut self, ctx: ::grpcio::RpcContext, req: super::workload_api::JWTBundlesRequest, sink: ::grpcio::ServerStreamingSink<super::workload_api::JWTBundlesResponse>);
    fn validate_jwtsvid(&mut self, ctx: ::grpcio::RpcContext, req: super::workload_api::ValidateJWTSVIDRequest, sink: ::grpcio::UnarySink<super::workload_api::ValidateJWTSVIDResponse>);
    fn fetch_x509_svid(&mut self, ctx: ::grpcio::RpcContext, req: super::workload_api::X509SVIDRequest, sink: ::grpcio::ServerStreamingSink<super::workload_api::X509SVIDResponse>);
}

pub fn create_spiffe_workload_api<S: SpiffeWorkloadApi + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_SPIFFE_WORKLOAD_API_FETCH_JWTSVID, move |ctx, req, resp| {
        instance.fetch_jwtsvid(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_server_streaming_handler(&METHOD_SPIFFE_WORKLOAD_API_FETCH_JWT_BUNDLES, move |ctx, req, resp| {
        instance.fetch_jwt_bundles(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_SPIFFE_WORKLOAD_API_VALIDATE_JWTSVID, move |ctx, req, resp| {
        instance.validate_jwtsvid(ctx, req, resp)
    });
    let mut instance = s;
    builder = builder.add_server_streaming_handler(&METHOD_SPIFFE_WORKLOAD_API_FETCH_X509_SVID, move |ctx, req, resp| {
        instance.fetch_x509_svid(ctx, req, resp)
    });
    builder.build()
}
