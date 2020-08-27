use serde::{Deserialize, Serialize};
use serde_diff::SerdeDiff;

use crate::utils::{PublicKey, Subject};

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, SerdeDiff, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Authentication {
    Method(Subject),
    Key(PublicKey),
}
