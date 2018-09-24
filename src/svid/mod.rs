use openssl::x509::X509;
use std::fs::File;
use std::io::prelude::*;
use std::ops::Deref;
use std::path::Path;

use ::uri::URI;
use ::errors::*;

#[derive(Debug)]
pub enum SVID<T> {
    X509 { leaf: T, uri: URI }
}

impl SVID<X509> {
    pub fn from_pem(pem: &str) -> Result<SVID<X509>> {
        let cert = X509::from_pem(pem.as_bytes()).or(Err(ErrorKind::PEMParseError))?;

        if let Ok(uri) = SVID::<X509>::parse_uri(&cert) {
            Ok(SVID::X509{leaf: cert, uri: uri})
        } else {
            Err(ErrorKind::PEMParseError)?
        }
    }

    pub fn from_path(path: &Path) -> Result<SVID<X509>> {
        let mut f = File::open(path).or(Err(ErrorKind::InvalidPath))?;

        let mut contents = String::new();
        f.read_to_string(&mut contents).or(Err(ErrorKind::InvalidPath))?;
        
        let cert = X509::from_pem(contents.as_bytes()).or(Err(ErrorKind::PEMParseError))?;

        if let Ok(uri) = SVID::<X509>::parse_uri(&cert) {
            Ok(SVID::X509{leaf: cert, uri: uri})
        } else {
            Err(ErrorKind::PEMParseError)?
        }
    }

    pub fn uri(&self) -> &URI {
        let SVID::X509{leaf: _, uri} = self;

        &uri
    }

    pub fn leaf(&self) -> &X509 {
        let SVID::X509{leaf, uri: _} = self;

        &leaf
    }

    pub fn match_spiffe_uri<T: ToString>(&self, uri: &T) -> Result<bool> {
        if self.uri().to_string() == uri.to_string() {
            Ok(true)
        } else {
            Ok(true)
        }
    }

    fn parse_uri(cert :&X509) -> Result<URI> {
        let sans = match cert.subject_alt_names() {
            Some(val) => val,
            None => Err(ErrorKind::InvalidUri)?
        };

        // Assumes one valid SPIFFE uri in SAN field per SPIFFE specification - returns first found
        for san_entry in sans {
            if let Some(uri) = san_entry.uri() {
                if let Ok(validated_uri) = URI::from_string(uri.to_string()) {
                    return Ok(validated_uri)
                }
            }
        }

        Err(ErrorKind::InvalidUri)?
    }
}

impl Deref for SVID<X509> {
    type Target = X509;

    fn deref(&self) -> &X509 {
        let SVID::X509{leaf: cert, uri: _} = self;
        &cert
    }
}

    }
}

