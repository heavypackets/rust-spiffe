extern crate openssl;
#[macro_use]
extern crate error_chain;
extern crate hyper;

pub mod svid;
pub mod uri;
pub mod errors;

// For future use only -- crate users should use URI ID type directly
trait SpiffeID {
}
