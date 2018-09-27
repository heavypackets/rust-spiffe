use futures::Future;
use futures::Stream;
use grpcio::{ChannelBuilder, EnvBuilder};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use std::vec::Vec;
use svid::{Bundle, SVID, X509};

mod workload_api;
mod workload_api_grpc;

use self::workload_api::{X509SVIDRequest, X509SVIDResponse};
use self::workload_api_grpc::SpiffeWorkloadApiClient;

lazy_static! {
    static ref INITIAL_CONNECTION_TIMEOUT: Duration = { Duration::new(15, 0) };
    static ref MAX_CLIENT_BACKOFF: Duration = { Duration::new(300, 0) };
}

error_chain!{
    errors {
        ClientConfigFailure {
            description("An error during the configuration of client")
            display("Unable to configure native client")
        }
        FetchFailure 
            description("An error during the the fetch of api payload")
            display("Unable to fetch api payload")
        }
    }

    links {
        Uri(::uri::Error, ::uri::ErrorKind);
        SVID(::svid::Error, ::svid::ErrorKind);
    }

    foreign_links {
        GRPCIO(::grpcio::Error);
    }
}

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
            let mut svid = SVID::<X509>::from_der(
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

pub type X509Stream = ::grpcio::ClientSStreamReceiver<::workload::workload_api::X509SVIDResponse>;

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

        let rx = self
            .client
            .fetch_x509_svid_opt(&X509SVIDRequest::new(), options)
            .chain_err(|| ErrorKind::ClientConfigFailure)?;

        let items = rx.take(1).collect().wait();

        match items {
            Ok(mut item) => {
                if let Some(val) = item.pop() {
                    Ok(X509Payload::new(val))
                } else {
                    Err(ErrorKind::FetchFailure)?
                }
            }
            Err(e) => Err(Error::with_chain(e, ErrorKind::FetchFailure))?,
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
