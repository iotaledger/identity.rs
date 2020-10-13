use identity_diff::Diff;
use serde::{Deserialize, Serialize};

/// Encoding method used for the specified public key.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize, Diff)]
#[serde(rename_all = "camelCase")]
pub enum KeyData {
    EthereumAddress(String),
    PublicKeyHex(String),
    PublicKeyJwk(String),
    PublicKeyBase58(String),
    PublicKeyPem(String),
    IotaAddress(String),
}

impl KeyData {
    pub fn as_str(&self) -> &str {
        match self {
            Self::EthereumAddress(inner) => inner.as_str(),
            Self::PublicKeyHex(inner) => inner.as_str(),
            Self::PublicKeyJwk(inner) => inner.as_str(),
            Self::PublicKeyBase58(inner) => inner.as_str(),
            Self::PublicKeyPem(inner) => inner.as_str(),
            Self::IotaAddress(inner) => inner.as_str(),
        }
    }
}
