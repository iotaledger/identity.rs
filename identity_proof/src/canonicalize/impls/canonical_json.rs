use anyhow::anyhow;
use canonical_json::to_string;
use identity_core::common::Object;
use serde_json::to_value;

use crate::{
    canonicalize::Canonicalize,
    error::{Error, Result},
};

/// Document normalization via canonical JSON
///
/// Ref: https://github.com/gibson042/canonicaljson-spec
pub struct CanonicalJson;

impl Canonicalize for CanonicalJson {
    fn canonicalize(&self, object: &Object) -> Result<Vec<u8>> {
        let value = to_value(object).map_err(|error| Error::Canonicalize(error.into()))?;
        let json = to_string(&value).map_err(|error| Error::Canonicalize(anyhow!("{:?}", error)))?;

        Ok(json.into_bytes())
    }
}
