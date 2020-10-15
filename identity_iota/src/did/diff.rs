use identity_core::did::DID;
use serde::{Deserialize, Serialize};

use crate::did::DIDProof;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct DIDDiff {
    pub id: DID,
    pub diff: String, // TODO: Replace with DiffDIDDocument
    pub proof: DIDProof,
}
