use identity_core::did::{DiffDIDDocument, DID};
use identity_proof::{HasProof, LdSignature};
use serde::{Deserialize, Serialize};

use core::str::FromStr;
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct DIDDiff {
    pub id: DID,
    pub diff: DiffDIDDocument,
    pub proof: LdSignature,
}

impl HasProof for DIDDiff {
    fn proof(&self) -> &LdSignature {
        &self.proof
    }

    fn proof_mut(&mut self) -> &mut LdSignature {
        &mut self.proof
    }
}

impl ToString for DIDDiff {
    fn to_string(&self) -> String {
        serde_json::to_string(&self).expect("Unable to serialize diff")
    }
}
impl FromStr for DIDDiff {
    type Err = identity_core::error::Error;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        serde_json::from_str::<DIDDiff>(string).map_err(identity_core::Error::DecodeJSON)
    }
}
