pub mod jwt;
mod workload_api;
mod workload_api_grpc;
pub mod x509;

use crate::svid;
use crate::uri;
use error_chain::error_chain;
use lazy_static::lazy_static;
use std::time::Duration;

lazy_static! {
    static ref INITIAL_CONNECTION_TIMEOUT: Duration = Duration::new(15, 0);
    static ref MAX_CLIENT_BACKOFF: Duration = Duration::new(300, 0);
}

error_chain! {
    errors {
        ClientConfigFailure {
            description("An error during the configuration of client")
            display("Unable to configure native client")
        }
        FetchFailure {
            description("An error during the the fetch of api payload")
            display("Unable to fetch api payload")
        }
        ValidateFailure {
            description("An error during the the validation of api payload")
            display("Unable to validate api payload")
        }
    }

    links {
        Uri(uri::Error, uri::ErrorKind);
        X509SVID(svid::x509::Error, svid::x509::ErrorKind);
        JWTSVID(svid::jwt::Error, svid::jwt::ErrorKind);
    }

    foreign_links {
        GRPCIO(grpcio::Error);
    }
}
