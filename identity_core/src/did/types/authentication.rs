use identity_diff::Diff;
use serde::{Deserialize, Serialize};

use crate::{
    did::{PublicKey, DID},
    utils::HasId,
};

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Diff, Serialize, Deserialize)]
#[serde(untagged)]
#[diff(from_into)]
pub enum Authentication {
    Method(DID),
    Key(PublicKey),
}

impl Default for Authentication {
    fn default() -> Self {
        Self::Method(Default::default())
    }
}

impl HasId for Authentication {
    type Id = DID;

    fn id(&self) -> &Self::Id {
        match self {
            Authentication::Method(subject) => subject,
            Authentication::Key(key) => &key.id,
        }
    }
}
