use identity_common::{Object, Timestamp};
use serde::{Deserialize, Serialize};

/// Options permitted to create/customize a linked data signature
///
/// Note: The exact definition of these options is still uncertain
///
/// Ref: https://github.com/w3c-ccg/ld-proofs/issues/16
/// Ref: https://github.com/w3c-ccg/ld-proofs/issues/19
/// Ref: https://github.com/w3c-ccg/ld-proofs/issues/27
///
/// Ref: https://github.com/w3c-ccg/ld-cryptosuite-registry/issues/3
/// Ref: https://github.com/w3c-ccg/ld-cryptosuite-registry/issues/34
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SignatureOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purpose: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nonce: Option<String>,
    #[serde(flatten)]
    pub properties: Object,
}
