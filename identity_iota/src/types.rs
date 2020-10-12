use core::ops::{Deref, DerefMut};
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
// HashObject
// =============================================================================

pub type TangleDoc = HashObject<DIDDocument>;

pub type TangleDiff = HashObject<DIDDiff>;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct HashObject<T> {
    pub hash: String,
    pub data: T,
}

impl<T> Deref for HashObject<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> DerefMut for HashObject<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}
