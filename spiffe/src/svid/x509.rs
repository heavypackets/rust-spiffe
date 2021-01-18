use crate::svid::{SVIDKind, SVID};
use crate::uri;
use crate::uri::URI;
use error_chain::error_chain;
use std::fmt;
use std::fs;
use std::ops::Deref;
use std::path::Path;

type OpenSSlX509Cert = ::openssl::x509::X509;

error_chain! {
    errors {
        InvalidFilePath(path: String) {
            description("An IO error during the parsing of an SVID")
            display("Unable to parse SVID: Invalid file path {}", path)
        }

        InvalidPEM {
            description("An error during the parsing of an SVID PEM")
            display("Unable to parse SVID: Not a valid PEM")
        }

        InvalidDER {
            description("An error during the parsing of an SVID DER")
            display("Unable to parse SVID: Not a valid DER")
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
        Uri(uri::Error, uri::ErrorKind);
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

pub type Bundle = Vec<u8>;
pub type Key = Vec<u8>;

pub struct X509 {
    cert: OpenSSlX509Cert,
    key: Option<Key>,
    bundle: Option<Bundle>,
}

impl X509 {
    pub fn new(cert: OpenSSlX509Cert, key: Option<Key>, bundle: Option<Bundle>) -> X509 {
        X509 {
            cert,
            key: match key {
                Some(k) => Some(k.to_vec()),
                None => None,
            },
            bundle: match bundle {
                Some(b) => Some(b.to_vec()),
                None => None,
            },
        }
    }

    pub fn cert(&self) -> &OpenSSlX509Cert {
        &self.cert
    }

    pub fn key(&self) -> Option<&Vec<u8>> {
        match self.key {
            Some(ref k) => Some(&k),
            None => None,
        }
    }

    pub fn bundle(&self) -> Option<&Vec<u8>> {
        match self.bundle {
            Some(ref b) => Some(&b),
            None => None,
        }
    }
}

impl fmt::Debug for X509 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let X509 { cert, .. } = self;
        write!(
            f,
            "OpenSSL X509 Certificate: {{ {:?} }}",
            cert.to_pem().unwrap_or_else(|_| vec![])
        )
    }
}

impl SVIDKind for X509 {}

impl SVID<X509> {
    pub fn from_pem(pem: &[u8], key: Option<Key>, bundle: Option<Bundle>) -> Result<SVID<X509>> {
        let cert = OpenSSlX509Cert::from_pem(pem).chain_err(|| ErrorKind::InvalidPEM)?;

        match SVID::<X509>::parse_uri(&cert) {
            Ok(uri) => Ok(SVID::<X509> {
                doc: X509::new(cert, key, bundle),
                uri,
            }),
            Err(e) => Err(e.chain_err(|| ErrorKind::InvalidSAN)),
        }
    }

    pub fn from_path(path: &Path, key: Option<&Path>, bundle: Option<&Path>) -> Result<SVID<X509>> {
        let contents = fs::read(path).chain_err(|| path)?;
        let cert =
            OpenSSlX509Cert::from_pem(contents.as_slice()).chain_err(|| ErrorKind::InvalidPEM)?;

        let key_contents = match key {
            Some(path) => Some(fs::read(path).chain_err(|| path)?.to_vec()),
            None => None,
        };
        let bundle_contents = match bundle {
            Some(path) => Some(fs::read(path).chain_err(|| path)?.to_vec()),
            None => None,
        };

        match SVID::<X509>::parse_uri(&cert) {
            Ok(uri) => Ok(SVID::<X509> {
                doc: X509::new(cert, key_contents, bundle_contents),
                uri,
            }),
            Err(e) => Err(e.chain_err(|| ErrorKind::InvalidPEM)),
        }
    }

    pub fn from_der(der: &[u8], key: Option<Key>, bundle: Option<Bundle>) -> Result<SVID<X509>> {
        let cert = OpenSSlX509Cert::from_der(der).chain_err(|| ErrorKind::InvalidDER)?;

        match SVID::<X509>::parse_uri(&cert) {
            Ok(uri) => Ok(SVID::<X509> {
                doc: X509::new(cert, key, bundle),
                uri,
            }),
            Err(e) => Err(e.chain_err(|| ErrorKind::InvalidSAN)),
        }
    }

    pub fn from_x509(
        cert: OpenSSlX509Cert,
        key: Option<Key>,
        bundle: Option<Bundle>,
    ) -> Result<SVID<X509>> {
        match SVID::<X509>::parse_uri(&cert) {
            Ok(uri) => Ok(SVID::<X509> {
                doc: X509::new(cert, key, bundle),
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
            None => return Err(ErrorKind::InvalidSAN.into()),
        };

        let mut validated_uri: Option<URI> = None;
        // Only allows one valid SPIFFE uri in SAN field per SPIFFE specification - returns error if multiple found
        for san_entry in sans {
            if let Some(uri) = san_entry.uri() {
                if let Ok(spiffe_uri) = uri.parse::<URI>() {
                    if let Some(validated_uri) = validated_uri {
                        return Err(ErrorKind::MultipleURIFound(
                            validated_uri.to_string(),
                            uri.to_string(),
                        )
                        .into());
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
