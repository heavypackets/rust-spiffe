use hyper::Url;
use std::string::ToString;
use std::str::FromStr;

error_chain!{
    errors { 
        InvalidURI(uri: String) {
            description("An error dURIng the parsing of a SPIFFE URI")
            display("Unable to parse SVID: Not a valid SPIFFE URI {}", uri)
        }

    }
}

impl<'a> From<&'a Url> for ErrorKind {
    fn from(url: &'a Url) -> Self {
        ErrorKind::InvalidURI(url.as_str().to_string())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
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
        if uri.query().is_some() || uri.port().is_some() || (uri.username() != "") || 
            uri.password().is_some() || uri.fragment().is_some() || uri.host().is_none() || 
            (uri.scheme() != "spiffe") || (uri.path() == "/") || (uri.path() == "") 
        { 
                Err(ErrorKind::from(&uri))?
        }

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
                    Err(e) => Err(e)?
                }
            },
            Err(_) => Err(ErrorKind::InvalidURI(uri.to_string()))?
        }
    }
}
