use identity_diff::Diff;
use serde::{Deserialize, Serialize};

use crate::{
    common::Object,
    error::Result,
    utils::{decode_b58, decode_hex},
};

/// Encoding method used for the specified public key.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, Diff)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub enum KeyData {
    EthereumAddress(String),
    PublicKeyHex(String),
    PublicKeyJwk(Object), // TODO: Replace this with libjose type
    PublicKeyBase58(String),
    PublicKeyPem(String),
}

impl KeyData {
    pub fn try_decode(&self) -> Option<Result<Vec<u8>>> {
        match self {
            Self::EthereumAddress(_) => None,
            Self::PublicKeyJwk(_) => None,
            Self::PublicKeyPem(_) => None,
            Self::PublicKeyHex(inner) => Some(decode_hex(inner)),
            Self::PublicKeyBase58(inner) => Some(decode_b58(inner)),
        }
    }
}
