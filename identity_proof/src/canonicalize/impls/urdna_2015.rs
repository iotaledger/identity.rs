use identity_common::Object;

use crate::{canonicalize::Canonicalize, error::Result};

/// Universal RDF Dataset Normalization Algorithm 2015 (Also known as GCA2015)
///
/// Ref: http://json-ld.github.io/normalization/spec/#introduction
pub struct Urdna2015;

impl Canonicalize for Urdna2015 {
    fn canonicalize(&self, _object: &Object) -> Result<Vec<u8>> {
        todo!("Implement Urdna2015.canonicalize(..)")
    }
}
