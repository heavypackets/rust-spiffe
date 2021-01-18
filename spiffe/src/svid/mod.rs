pub mod jwt;
pub mod x509;

use crate::uri::URI;

pub trait SVIDKind {}

#[derive(Debug)]
pub struct SVID<T: SVIDKind> {
    doc: T,
    uri: URI,
}
