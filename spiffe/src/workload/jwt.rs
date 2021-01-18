use crate::svid::jwt::Jwt;
use crate::svid::SVID;
use crate::uri::URI;
use crate::workload::workload_api::{JWTSVIDRequest, ValidateJWTSVIDRequest};
use crate::workload::workload_api_grpc::SpiffeWorkloadApiClient;
use crate::workload::INITIAL_CONNECTION_TIMEOUT;
use crate::workload::MAX_CLIENT_BACKOFF;
use crate::workload::{ErrorKind, Result, ResultExt};
use grpcio::{ChannelBuilder, EnvBuilder};
use log::error;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;

pub struct JWTClient {
    client: SpiffeWorkloadApiClient,
    timeout: Duration,
}

pub struct ValidateResponse {
    spiffe_id: URI,
    claims: Option<protobuf::well_known_types::Struct>,
}

impl ValidateResponse {
    pub fn spiffe_id(&self) -> &URI {
        &self.spiffe_id
    }

    pub fn claims(&self) -> Option<&protobuf::well_known_types::Struct> {
        self.claims.as_ref()
    }
}

pub use protobuf::well_known_types::Struct;

impl JWTClient {
    pub fn new(addr: &str, backoff: Option<Duration>, timeout: Option<Duration>) -> JWTClient {
        let backoff = backoff.unwrap_or(*MAX_CLIENT_BACKOFF);
        let env = Arc::new(EnvBuilder::new().build());
        let channel = ChannelBuilder::new(env)
            .initial_reconnect_backoff(backoff)
            .max_reconnect_backoff(backoff)
            .connect(addr);
        JWTClient {
            client: SpiffeWorkloadApiClient::new(channel),
            timeout: timeout.unwrap_or(*INITIAL_CONNECTION_TIMEOUT),
        }
    }

    pub fn validate(&self, audience: String, svid: Jwt) -> Result<ValidateResponse> {
        let mut metadata = ::grpcio::MetadataBuilder::new();
        metadata
            .add_str("workload.spiffe.io", "true")
            .chain_err(|| ErrorKind::ClientConfigFailure)?;

        let options = ::grpcio::CallOption::default()
            .timeout(self.timeout)
            .headers(metadata.build());

        let mut req = ValidateJWTSVIDRequest::new();
        req.set_audience(audience);
        req.set_svid((&svid.svid()).to_string());

        let res = self
            .client
            .validate_jwtsvid_opt(&req, options)
            .map_err(|e| {
                error!("Error during validation: {}.", e);
                ErrorKind::ValidateFailure
            })?;

        Ok(ValidateResponse {
            spiffe_id: URI::from_str(&res.spiffe_id).chain_err(|| ErrorKind::ValidateFailure)?,
            claims: res.claims.into_option(),
        })
    }

    /// Fetch the first JWT-SVID of the workload
    pub fn fetch(&self, audience: String) -> Result<SVID<Jwt>> {
        let mut metadata = ::grpcio::MetadataBuilder::new();
        metadata
            .add_str("workload.spiffe.io", "true")
            .chain_err(|| ErrorKind::ClientConfigFailure)?;

        let options = ::grpcio::CallOption::default()
            .timeout(self.timeout)
            .headers(metadata.build());

        let mut req = JWTSVIDRequest::new();
        let mut audience_field = protobuf::RepeatedField::new();
        audience_field.push(audience);
        req.set_audience(audience_field);

        let mut res = self
            .client
            .fetch_jwtsvid_opt(&req, options)
            .chain_err(|| ErrorKind::FetchFailure)?;

        // Only take the first one.
        let svid = res.svids.pop().chain_err(|| ErrorKind::FetchFailure)?;

        SVID::<Jwt>::new(svid.svid, &svid.spiffe_id).chain_err(|| ErrorKind::FetchFailure)
    }
}
