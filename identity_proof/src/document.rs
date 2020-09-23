use identity_common::{Object, SerdeInto};
use serde::Serialize;

use crate::error::Result;

/// A document that can be signed and verified with a `LinkedDataSignature`
pub trait LinkedDataDocument {
    /// Returns the document represented as an `Object`.
    fn to_object(&self) -> Result<Object>;
}

impl<T> LinkedDataDocument for T
where
    T: Serialize,
{
    fn to_object(&self) -> Result<Object> {
        self.serde_into().map_err(Into::into)
    }
}
