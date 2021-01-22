use sha2::{digest::Output, Digest, Sha256};

use crate::{convert::ToJson, error::Result};

/// Returns the given `data` serialized using JSON Canonicalization Scheme and
/// hashed using SHA-256.
pub fn jcs_sha256<T>(data: &T) -> Result<Output<Sha256>>
where
    T: ToJson,
{
    data.to_jcs().map(|json| Sha256::digest(&json))
}
