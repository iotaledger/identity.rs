use identity_diff::Diff;
use serde::{Deserialize, Serialize};

use crate::{
    did::{PublicKey, DID},
    utils::HasId,
};

#[derive(Debug, Clone, PartialEq, Diff, Serialize, Deserialize)]
#[serde(untagged)]
#[diff(from_into)]
pub enum Authentication {
    DID(DID),
    Key(PublicKey),
}

impl Default for Authentication {
    fn default() -> Self {
        Self::DID(Default::default())
    }
}

impl From<DID> for Authentication {
    fn from(other: DID) -> Self {
        Self::DID(other)
    }
}

impl From<PublicKey> for Authentication {
    fn from(other: PublicKey) -> Self {
        Self::Key(other)
    }
}

impl HasId for Authentication {
    type Id = DID;

    fn id(&self) -> &Self::Id {
        match self {
            Authentication::DID(subject) => subject,
            Authentication::Key(key) => key.id(),
        }
    }
}
