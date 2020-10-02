use identity_diff::Diff;
use serde::{Deserialize, Serialize};

use crate::utils::{HasId, PublicKey, Subject};

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

impl HasId for Authentication {
    type Id = Subject;

    fn id(&self) -> &Self::Id {
        match self {
            Authentication::Method(subject) => subject,
            Authentication::Key(key) => &key.id,
        }
    }
}
