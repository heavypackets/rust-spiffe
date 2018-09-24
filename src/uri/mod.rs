use hyper::Url;
use std::string::ToString;

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
        // This unwrap should never fail -- if it does, somehthing is wrong and please file bug
        self.uri.host_str().unwrap().to_string()
    }

    pub fn from_str(uri: &str) -> Result<URI> {
        URI::from_string(uri.to_string())
    }

    pub fn from_string(uri: String) -> Result<URI> {
        match uri.parse::<Url>() {
            Ok(uri) => {
                match URI::validate_spiffe_uri(uri) {
                    Ok(validated_uri) => return Ok(URI{ uri: validated_uri }),
                    Err(_) => Err(ErrorKind::InvalidUri)?
                }
            },
            Err(_) => Err(ErrorKind::InvalidUri)?
        }
    }

    pub fn validate_spiffe_uri(uri: Url) -> Result<Url> {
        match uri.scheme() {
            "spiffe" => (), 
            _ => Err(ErrorKind::InvalidUri)?
        };

        match uri.host() {
            Some(_) => (),
            _ => Err(ErrorKind::InvalidUri)?
        };

        match uri.path() {
            "/" => Err(ErrorKind::InvalidUri)?,
            "" => Err(ErrorKind::InvalidUri)?,
            _ => ()
        };

        match uri.query() {
            Some(_) => Err(ErrorKind::InvalidUri)?,
            _ => ()
        };

        match uri.port() {
            Some(_) => Err(ErrorKind::InvalidUri)?,
            _ => ()
        };

        match uri.username() {
            "" => (),
            _ => Err(ErrorKind::InvalidUri)?
        };

        match uri.password() {
            Some(_) => Err(ErrorKind::InvalidUri)?,
            _ => ()
        };

        match uri.fragment() {
            Some(_) => Err(ErrorKind::InvalidUri)?,
            _ => ()
        };

        return Ok(uri)
    }
}

impl ToString for URI {
    fn to_string(&self) -> String {
        self.uri.as_str().to_string()
    }
}
