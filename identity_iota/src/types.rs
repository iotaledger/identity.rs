use identity_core::{
    common::Timestamp,
    did::{DIDDocument, DID},
};
use serde::{Deserialize, Serialize};

// =============================================================================
// DIDDiff
// =============================================================================

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct DIDDiff {
    pub did: DID,
    pub diff: String,
    pub time: Timestamp,
    pub signature: String,
}

// =============================================================================
// Tangle Doc
// =============================================================================

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct TangleDoc {
    pub hash: String,
    pub data: DIDDocument,
}

// =============================================================================
// Tangle Diff
// =============================================================================

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct TangleDiff {
    pub hash: String,
    pub data: DIDDiff,
}
