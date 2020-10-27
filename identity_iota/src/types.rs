use identity_core::did::DIDDocument;
use serde::{Deserialize, Serialize};

use crate::did::DIDDiff;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct TangleDoc {
    pub hash: String,
    pub data: DIDDocument,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct TangleDiff {
    pub hash: String,
    pub data: DIDDiff,
}
