mod workload_api;
mod workload_api_grpc;

use self::workload_api::{X509SVIDRequest, X509SVIDResponse};
use self::workload_api_grpc::*;
use futures::Future;
use futures::Stream;
use grpcio::{ChannelBuilder, EnvBuilder};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use std::vec::Vec;
use svid;
use svid::Bundle;

lazy_static! {
    static ref DEFAULT_CLIENT_TIMEOUT: Duration = { Duration::new(15, 0) };
}

error_chain!{
    errors {
        WorkloadClientFail
        FetchFail
    }

    links {
        Uri(::uri::Error, ::uri::ErrorKind);
        SVID(::svid::Error, ::svid::ErrorKind);
    }

    foreign_links {
        GRPCIO(::grpcio::Error);
    }
}

pub trait APIKind {}

pub struct X509(SpiffeWorkloadApiClient);
impl APIKind for X509 {}

type CRL = Vec<u8>;

#[derive(Debug)]
pub struct X509Response {
    svids: Vec<svid::SVID<svid::X509>>,
    federated_bundles: HashMap<String, Bundle>,
    crl: Vec<CRL>,
}

impl X509Response {
    fn new(response: X509SVIDResponse) -> Result<X509Response> {
        let mut svids = Vec::<svid::SVID<svid::X509>>::with_capacity(response.svids.len());

        for x in response.svids.into_iter() {
            let mut svid = svid::SVID::<svid::X509>::from_der(
                &x.x509_svid,
                Some(x.bundle.to_vec()),
                Some(x.x509_svid_key.to_vec()),
            )?;

            svids.push(svid);
        }

        Ok(X509Response {
            svids,
            federated_bundles: response.federated_bundles,
            crl: response.crl.into_vec(),
        })
    }

    pub fn svids(&self) -> &Vec<svid::SVID<svid::X509>> {
        &self.svids
    }

    pub fn federated_bundles(&self) -> &HashMap<String, Bundle> {
        &self.federated_bundles
    }

    pub fn crl(&self) -> &Vec<CRL> {
        &self.crl
    }
}

#[allow(dead_code)]
pub struct Client<T: APIKind> {
    client: T,
}

impl Client<X509> {
    pub fn new(addr: &str) -> Client<X509> {
        let env = Arc::new(EnvBuilder::new().build());
        let channel = ChannelBuilder::new(env)
            .initial_reconnect_backoff(::std::time::Duration::new(30, 0))
            .max_reconnect_backoff(::std::time::Duration::new(300, 0))
            .connect(addr);
        Client::<X509> {
            client: X509(SpiffeWorkloadApiClient::new(channel)),
        }
    }

    pub fn fetch_once(&self, timeout: Option<Duration>) -> Result<Result<X509Response>> {
        let X509(ref client) = self.client;
        let mut metadata = ::grpcio::MetadataBuilder::new();
        metadata.add_str("workload.spiffe.io", "true").unwrap();

        let options = ::grpcio::CallOption::default()
            .timeout(timeout.unwrap_or(*DEFAULT_CLIENT_TIMEOUT))
            .headers(metadata.build());

        let rx = client
            .fetch_x509_svid_opt(&X509SVIDRequest::new(), options)
            .chain_err(|| ErrorKind::WorkloadClientFail)?;

        let items = rx.take(1).collect().wait();

        match items {
            Ok(item) => {
                if let Some(val) = item.get(0) {
                    Ok(X509Response::new(val.clone()))
                } else {
                    Err(ErrorKind::FetchFail)?
                }
            }
            Err(e) => Err(Error::with_chain(e, ErrorKind::FetchFail))?,
        }
    }
}
