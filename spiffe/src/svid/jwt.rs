use crate::svid::{SVIDKind, SVID};
use crate::uri::URI;
use error_chain::error_chain;
use std::ops::Deref;
use std::str::FromStr;
use zeroize::Zeroize;

error_chain! {
    errors {
        InvalidURI {
            description("An error occured during the parsing of the SPIFFE ID")
            display("The SPIFFE ID can not be parsed into a valid SPIFFE URI")
        }
    }
}

impl SVIDKind for Jwt {}

#[derive(Zeroize)]
#[zeroize(drop)]
pub struct Jwt {
    svid: String,
}

impl Jwt {
    pub fn new(svid: String) -> Jwt {
        Jwt { svid }
    }

    pub fn svid(&self) -> &str {
        &self.svid
    }
}

impl SVID<Jwt> {
    pub fn new(svid: String, uri: &str) -> Result<SVID<Jwt>> {
        Ok(SVID::<Jwt> {
            doc: Jwt { svid },
            uri: URI::from_str(uri).chain_err(|| ErrorKind::InvalidURI)?,
        })
    }
}

impl Deref for SVID<Jwt> {
    type Target = Jwt;

    fn deref(&self) -> &Jwt {
        let SVID::<Jwt> { doc, .. } = self;
        &doc
    }
}
