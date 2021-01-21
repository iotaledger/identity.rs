use serde::Serialize;
use serde_jcs::to_vec;
use sha2::{digest::Output, Digest, Sha256};

use crate::error::{Error, Result};

pub fn jcs_sha256<T>(data: &T) -> Result<Output<Sha256>>
where
    T: Serialize,
{
    to_vec(data)
        .map_err(Error::EncodeJSON)
        .map(|json| Sha256::digest(&json))
}
