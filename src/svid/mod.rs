use openssl::x509::X509;
use std::fs::File;
use std::io::prelude::*;
use std::ops::Deref;
use std::path::Path;

use ::uri::URI;

error_chain!{
    errors { 
        InvalidFilePath(path: String) {
            description("An error during the parsing of an SVID")
            display("Unable to parse SVID: Invalid file path {}", path)
        }

        InvalidPEM {
            description("An error during the parsing of an SVID")
            display("Unable to parse SVID: Not a valid PEM")
        }

        InvalidPEMSAN {
            description("An error during the parsing of an SVID")
            display("Unable to parse SVID: PEM SANs (Subject Alternate Name) do not contain a valid SPIFFE URI")
        }
    }

    links {
        Uri(::uri::Error, ::uri::ErrorKind);
    }

    foreign_links {
        SSL(::openssl::error::ErrorStack);
        Io(::std::io::Error);
    }
}

impl<'a> From<&'a Path> for ErrorKind {
    fn from(path: &'a Path) -> Self {
        ErrorKind::InvalidFilePath(path.to_str().unwrap_or("").to_string())
    }
}

#[derive(Debug)]
pub enum SVID<T> {
    X509 { cert: T, uri: URI }
}

impl SVID<X509> {
    pub fn from_pem(pem: &str) -> Result<SVID<X509>> {
        let cert = X509::from_pem(pem.as_bytes()).chain_err(|| ErrorKind::InvalidPEM)?;

        match SVID::<X509>::parse_uri(&cert) {
            Ok(uri) => Ok(SVID::X509{cert, uri}),
            Err(e) => Err(e.chain_err(|| ErrorKind::InvalidPEMSAN))
        }
    }

    pub fn from_path(path: &Path) -> Result<SVID<X509>> {
        let mut f = File::open(path).chain_err(|| path)?;

        let mut contents = String::new();
        f.read_to_string(&mut contents).chain_err(|| path)?;
        
        let cert = X509::from_pem(contents.as_bytes()).chain_err(|| ErrorKind::InvalidPEM)?;

        match SVID::<X509>::parse_uri(&cert) {
            Ok(uri) => Ok(SVID::X509{cert, uri}),
            Err(e) => Err(e.chain_err(|| ErrorKind::InvalidPEM))
        }
    }
        }
    }

    pub fn uri(&self) -> &URI {
        let SVID::X509{uri, ..} = self;
        &uri
    }

    pub fn x509(&self) -> &X509 {
        let SVID::X509{cert, ..} = self;
        &cert
    }

    pub fn match_spiffe_uri(&self, uri: &str) -> Result<bool> {
        Ok(self.uri().to_string().eq_ignore_ascii_case(uri))
    }

    fn parse_uri(cert :&X509) -> Result<URI> {
        let sans = match cert.subject_alt_names() {
            Some(val) => val,
            None => Err(ErrorKind::InvalidPEMSAN)?
        };

        // Assumes one valid SPIFFE uri in SAN field per SPIFFE specification - returns first found
        for san_entry in sans {
            if let Some(uri) = san_entry.uri() {
                if let Ok(validated_uri) = uri.parse::<URI>() {
                    return Ok(validated_uri)
                }
            }
        }

        Err(ErrorKind::InvalidPEMSAN)?
    }
}

impl Deref for SVID<X509> {
    type Target = X509;

    fn deref(&self) -> &X509 {
        let SVID::X509{cert
, ..} = self;
        &cert

    }
}

