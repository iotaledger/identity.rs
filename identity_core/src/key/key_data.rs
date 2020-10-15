use identity_diff::Diff;
use serde::{Deserialize, Serialize};

use crate::common::Object;

/// Encoding method used for the specified public key.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, Diff)]
#[serde(rename_all = "camelCase")]
pub enum KeyData {
    EthereumAddress(String),
    PublicKeyHex(String),
    PublicKeyJwk(Object), // TODO: Replace this with libjose type
    PublicKeyBase58(String),
    PublicKeyPem(String),
}
