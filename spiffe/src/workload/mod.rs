mod workload_api;
mod workload_api_grpc;

use self::workload_api::{X509SVIDRequest, X509SVIDResponse};
use self::workload_api_grpc::*;
use futures::Stream;
use grpcio::{ChannelBuilder, EnvBuilder};
use std::collections::HashMap;
use std::sync::Arc;
use std::vec::Vec;
use svid;
use svid::{Bundle, Key};

error_chain!{
    errors {
        ClientFail
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
    pub fn new(response: X509SVIDResponse) -> Result<X509Response> {
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
        let channel = ChannelBuilder::new(env).connect(addr);
        Client::<X509> {
            client: X509(SpiffeWorkloadApiClient::new(channel)),
        }
    }

    pub fn fetch(&self) -> Result<X509Response> {
        let X509(ref client) = self.client;

        let rx = client
            .fetch_x509_svid(&X509SVIDRequest::new())
            .chain_err(|| ErrorKind::ClientFail)?;

        let mut result = None;
        {
            let rx = rx.and_then(|rs| {
                result = Some(rs.clone());

                Ok(())
            });

            let _wait = rx.wait();
        }

        if let Some(res) = result {
            X509Response::new(res)
        } else {
            Err(ErrorKind::FetchFail)?
        }
    }

    /*
    // do(options, sucessful fetch, failed fetch)
    pub fn spawn_fetch<S, F>(
        c: Client<X509>,
        options: &ClientSpawnOptions,
        on_fetch: S,
        on_retry: F,
    ) -> JoinHandle<Result<()>> {
    } */
}
