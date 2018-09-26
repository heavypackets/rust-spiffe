mod workload_api;
mod workload_api_grpc;

use self::workload_api::X509SVIDRequest;
use self::workload_api_grpc::*;
use std::collections::HashMap;
use std::vec::Vec;
use svid::SVID;

error_chain!{
    errors {}

    links {
        Uri(::uri::Error, ::uri::ErrorKind);
        SVID(::svid::Error, ::svid::ErrorKind);
    }
}

pub trait APIKind {}

pub struct WorkloadX509(SpiffeWorkloadApiClient);
impl APIKind for WorkloadX509 {}

#[derive(Debug)]
pub struct X509Response {
    svids: Vec<SVID<::svid::X509>>,
    federated_bundles: HashMap<String, String>,
    crl: Vec<String>,
}
/**
impl From<X509SVIDRequest> for X509Response {
    fn from(request: X509SVIDRequest) -> Self {}
}*/

#[allow(dead_code)]
pub struct Client<T: APIKind> {
    client: T,
}

impl Client<WorkloadX509> {
    pub fn fetch_bundle() -> Result<X509Response> {
        Err("")?
    }
}
