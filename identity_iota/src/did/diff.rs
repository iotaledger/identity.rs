use identity_core::did::{DiffDIDDocument, DID};
use identity_proof::{HasProof, LdSignature};
use serde::{Deserialize, Serialize};

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
