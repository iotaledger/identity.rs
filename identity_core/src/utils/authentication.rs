use identity_diff::Diff;
use serde::{Deserialize, Serialize};

use crate::utils::{PublicKey, Subject};

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, Diff, Serialize, Deserialize)]
#[serde(untagged)]
#[diff(from_into)]
pub enum Authentication {
    Method(Subject),
    Key(PublicKey),
}

impl Default for Authentication {
    fn default() -> Self {
        Self::Method(Subject::default())
    }
}
