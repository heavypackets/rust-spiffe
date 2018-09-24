use hyper::Url;
use std::string::ToString;
use std::str::FromStr;

use ::errors::*;

#[derive(Clone, Debug)]
pub struct URI {
    uri: Url
}

impl URI {
    pub fn path(&self) -> String {
        self.uri.path().to_string()
    }

    pub fn trust_domain(&self) -> String {
        // This unwrap should never fail -- if it does, something is wrong and please file bug
        self.uri.host_str().unwrap().to_string()
    }

    pub fn validate_spiffe_uri(uri: Url) -> Result<Url> {
        match uri.scheme() {
            "spiffe" => (),
            _ => Err(ErrorKind::InvalidUri)?
        };

        if uri.host().is_none() { Err(ErrorKind::InvalidUri)? }

        match uri.path() {
            "/" => Err(ErrorKind::InvalidUri)?,
            "" => Err(ErrorKind::InvalidUri)?,
            _ => ()
        };

        if uri.query().is_some() { Err(ErrorKind::InvalidUri)? }
        if uri.port().is_some() { Err(ErrorKind::InvalidUri)? }

        match uri.username() {
            "" => (),
            _ => Err(ErrorKind::InvalidUri)?
        };

        if uri.password().is_some() { Err(ErrorKind::InvalidUri)? }
        if uri.fragment().is_some() { Err(ErrorKind::InvalidUri)? }

        Ok(uri)
    }
}

impl ToString for URI {
    fn to_string(&self) -> String {
        self.uri.as_str().to_string()
    }
}

impl FromStr for URI {
    type Err = Error;

    fn from_str(uri: &str) -> Result<URI> {
        match uri.parse::<Url>() {
            Ok(uri) => {
                match URI::validate_spiffe_uri(uri) {
                    Ok(validated_uri) => Ok(URI{ uri: validated_uri }),
                    Err(_) => Err(ErrorKind::InvalidUri)?
                }
            },
            Err(_) => Err(ErrorKind::InvalidUri)?
        }
    }
}
