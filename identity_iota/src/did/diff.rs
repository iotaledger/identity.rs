use identity_core::{
    convert::{SerdeInto as _, ToJson as _},
    did_doc::{Document, SetSignature, Signature, TrySignature, TrySignatureMut},
    identity_diff::Diff,
};

use crate::{
    did::{IotaDID, IotaDocument},
    error::Result,
};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct DIDDiff {
    pub did: IotaDID,
    pub prev_msg: String,
    pub diff: String,
    pub proof: Option<Signature>,
}

impl DIDDiff {
    pub fn new(document: &IotaDocument, other: &IotaDocument, prev_msg: String) -> Result<Self> {
        let a: Document = document.serde_into()?;
        let b: Document = other.serde_into()?;
        let diff: String = Diff::diff(&a, &b)?.to_json()?;

        Ok(Self {
            did: document.id().clone(),
            prev_msg,
            diff,
            proof: None,
        })
    }
}

impl TrySignature for DIDDiff {
    fn try_signature(&self) -> Option<&Signature> {
        self.proof.as_ref()
    }
}

impl TrySignatureMut for DIDDiff {
    fn try_signature_mut(&mut self) -> Option<&mut Signature> {
        self.proof.as_mut()
    }
}

impl SetSignature for DIDDiff {
    fn set_signature(&mut self, value: Signature) {
        self.proof = Some(value);
    }
}
