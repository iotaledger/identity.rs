use serde::{Deserialize, Serialize};
use serde_diff::SerdeDiff;

use crate::utils::{PublicKey, Subject};

#[derive(Debug, Clone, PartialEq, SerdeDiff, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Authentication {
    Method(Subject),
    PublicKey(PublicKey),
}
