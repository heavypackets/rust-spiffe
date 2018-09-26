use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::ops::Deref;
use std::path::Path;

type OpenSSlX509Cert = ::openssl::x509::X509;

use uri::URI;

error_chain!{
    errors {
        InvalidFilePath(path: String) {
            description("An IO error during the parsing of an SVID")
            display("Unable to parse SVID: Invalid file path {}", path)
        }

        InvalidPEM {
            description("An error during the parsing of an SVID PEM")
            display("Unable to parse SVID: Not a valid PEM")
        }

        InvalidSAN {
            description("An error during the validation of SVID SANs")
            display("Unable to parse SVID: SANs do not contain a valid SPIFFE URI")
        }

        MultipleURIFound(first: String, next: String) {
            description("An error during the validation of SVID certificate")
            display("Multiple valid SPIFFE URIs found in SVID: {} & {}", first, next)
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

pub trait SVIDKind {}

pub struct X509(OpenSSlX509Cert);

impl fmt::Debug for X509 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let X509(cert) = self;
        write!(
            f,
            "OpenSSL X509 Certificate: {{ {:?} }}",
            cert.to_pem().unwrap_or_else(|_| vec![])
        )
    }
}

impl SVIDKind for X509 {}

#[derive(Debug)]
pub struct SVID<T: SVIDKind> {
    doc: T,
    uri: URI,
}

impl SVID<X509> {
    pub fn from_pem(pem: &str) -> Result<SVID<X509>> {
        let cert = OpenSSlX509Cert::from_pem(pem.as_bytes()).chain_err(|| ErrorKind::InvalidPEM)?;

        match SVID::<X509>::parse_uri(&cert) {
            Ok(uri) => Ok(SVID::<X509> {
                doc: X509(cert),
                uri,
            }),
            Err(e) => Err(e.chain_err(|| ErrorKind::InvalidSAN)),
        }
    }

    pub fn from_path(path: &Path) -> Result<SVID<X509>> {
        let mut f = File::open(path).chain_err(|| path)?;

        let mut contents = String::new();
        f.read_to_string(&mut contents).chain_err(|| path)?;

        let cert =
            OpenSSlX509Cert::from_pem(contents.as_bytes()).chain_err(|| ErrorKind::InvalidPEM)?;

        match SVID::<X509>::parse_uri(&cert) {
            Ok(uri) => Ok(SVID::<X509> {
                doc: X509(cert),
                uri,
            }),
            Err(e) => Err(e.chain_err(|| ErrorKind::InvalidPEM)),
        }
    }

    pub fn from_x509(cert: OpenSSlX509Cert) -> Result<SVID<X509>> {
        match SVID::<X509>::parse_uri(&cert) {
            Ok(uri) => Ok(SVID::<X509> {
                doc: X509(cert),
                uri,
            }),
            Err(e) => Err(e.chain_err(|| ErrorKind::InvalidPEM)),
        }
    }

    pub fn uri(&self) -> &URI {
        let SVID::<X509> { uri, .. } = self;
        &uri
    }

    pub fn x509(&self) -> &X509 {
        let SVID::<X509> { doc, .. } = self;
        &doc
    }

    pub fn match_spiffe_uri(&self, uri: &str) -> Result<bool> {
        Ok(self.uri().to_string().eq_ignore_ascii_case(uri))
    }

    fn parse_uri(cert: &OpenSSlX509Cert) -> Result<URI> {
        let sans = match cert.subject_alt_names() {
            Some(val) => val,
            None => Err(ErrorKind::InvalidSAN)?,
        };

        let mut validated_uri: Option<URI> = None;
        // Only allows one valid SPIFFE uri in SAN field per SPIFFE specification - returns error if multiple found
        for san_entry in sans {
            if let Some(uri) = san_entry.uri() {
                if let Ok(spiffe_uri) = uri.parse::<URI>() {
                    if validated_uri.is_some() {
                        Err(ErrorKind::MultipleURIFound(
                            validated_uri.unwrap().to_string(),
                            uri.to_string(),
                        ))?;
                    }
                    validated_uri = Some(spiffe_uri);
                }
            }
        }

        validated_uri.ok_or_else(|| Error::from(ErrorKind::InvalidSAN))
    }
}

impl Deref for SVID<X509> {
    type Target = X509;

    fn deref(&self) -> &X509 {
        let SVID::<X509> { doc, .. } = self;
        &doc
    }
}
