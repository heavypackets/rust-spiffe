use crate::svid::{x509::Bundle, x509::X509, SVID};
use crate::workload::workload_api::{X509SVIDRequest, X509SVIDResponse};
use crate::workload::workload_api_grpc::SpiffeWorkloadApiClient;
use crate::workload::INITIAL_CONNECTION_TIMEOUT;
use crate::workload::MAX_CLIENT_BACKOFF;
use crate::workload::{Error, ErrorKind, Result, ResultExt};
use futures::executor::block_on;
use futures::StreamExt;
use grpcio::{ChannelBuilder, EnvBuilder};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use std::vec::Vec;

type CRL = Vec<u8>;

#[derive(Debug)]
pub struct X509Payload {
    svids: Vec<SVID<X509>>,
    federated_bundles: HashMap<String, Bundle>,
    crl: Vec<CRL>,
}

pub type X509Response = X509SVIDResponse;

impl X509Payload {
    pub fn new(response: X509Response) -> Result<X509Payload> {
        let mut svids = Vec::<SVID<X509>>::with_capacity(response.svids.len());

        for x in response.svids.into_iter() {
            let svid = SVID::<X509>::from_der(
                &x.x509_svid,
                Some(x.bundle.to_vec()),
                Some(x.x509_svid_key.to_vec()),
            )?;

            svids.push(svid);
        }

        Ok(X509Payload {
            svids,
            federated_bundles: response.federated_bundles,
            crl: response.crl.into_vec(),
        })
    }

    pub fn svids(&self) -> &Vec<SVID<X509>> {
        &self.svids
    }

    pub fn federated_bundles(&self) -> &HashMap<String, Bundle> {
        &self.federated_bundles
    }

    pub fn crl(&self) -> &Vec<CRL> {
        &self.crl
    }
}

pub type X509Stream = ::grpcio::ClientSStreamReceiver<X509SVIDResponse>;

pub struct X509Client {
    client: SpiffeWorkloadApiClient,
}

impl X509Client {
    pub fn new(addr: &str, backoff: Option<Duration>) -> X509Client {
        let backoff = backoff.unwrap_or(*MAX_CLIENT_BACKOFF);
        let env = Arc::new(EnvBuilder::new().build());
        let channel = ChannelBuilder::new(env)
            .initial_reconnect_backoff(backoff)
            .max_reconnect_backoff(backoff)
            .connect(addr);
        X509Client {
            client: SpiffeWorkloadApiClient::new(channel),
        }
    }

    pub fn fetch(&self, timeout: Option<Duration>) -> Result<Result<X509Payload>> {
        let mut metadata = ::grpcio::MetadataBuilder::new();
        metadata
            .add_str("workload.spiffe.io", "true")
            .chain_err(|| ErrorKind::ClientConfigFailure)?;

        let options = ::grpcio::CallOption::default()
            .timeout(timeout.unwrap_or(*INITIAL_CONNECTION_TIMEOUT))
            .headers(metadata.build());

        let mut rx = self
            .client
            .fetch_x509_svid_opt(&X509SVIDRequest::new(), options)
            .chain_err(|| ErrorKind::ClientConfigFailure)?;

        let item = block_on(rx.next());

        match item {
            Some(Ok(item)) => Ok(X509Payload::new(item)),
            Some(Err(e)) => Err(Error::with_chain(e, ErrorKind::FetchFailure)),
            None => Err(ErrorKind::FetchFailure.into()),
        }
    }

    pub fn stream(&self, timeout: Option<Duration>) -> Result<X509Stream> {
        let mut metadata = ::grpcio::MetadataBuilder::new();
        metadata
            .add_str("workload.spiffe.io", "true")
            .chain_err(|| ErrorKind::ClientConfigFailure)?;

        let options = ::grpcio::CallOption::default()
            .timeout(timeout.unwrap_or(*INITIAL_CONNECTION_TIMEOUT))
            .headers(metadata.build());

        let rx = self
            .client
            .fetch_x509_svid_opt(&X509SVIDRequest::new(), options)
            .chain_err(|| ErrorKind::ClientConfigFailure)?;

        Ok(rx)
    }
}
