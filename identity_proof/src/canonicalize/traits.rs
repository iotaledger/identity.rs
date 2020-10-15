use identity_core::common::Object;

use crate::error::Result;

pub trait Canonicalize {
    /// Returns the normalized bytes of an object according to *some*
    /// canonicalization algorithm.
    fn canonicalize(&self, object: &Object) -> Result<Vec<u8>>;
}
