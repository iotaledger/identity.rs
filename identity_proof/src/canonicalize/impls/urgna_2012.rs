use identity_core::common::Object;

use crate::{canonicalize::Canonicalize, error::Result};

/// Universal RDF Graph Normalization Algorithm 2012
///
/// Ref: http://json-ld.github.io/normalization/spec/#urgna2012
pub struct Urgna2012;

impl Canonicalize for Urgna2012 {
    fn canonicalize(&self, _object: &Object) -> Result<Vec<u8>> {
        todo!("Implement Urgna2012.canonicalize(..)")
    }
}
