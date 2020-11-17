use identity_core::did_doc::{SetSignature, Signature, TrySignature, TrySignatureMut};

use crate::{
    did::{IotaDID, IotaDocument},
    error::Result,
};

pub type Diff = (); // TODO: FIXME

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct DIDDiff {
    pub did: IotaDID,
    pub diff: Diff,
    pub proof: Option<Signature>,
}

impl DIDDiff {
    pub fn new(document: &IotaDocument, _other: &IotaDocument) -> Result<Self> {
        let diff: Diff = ();

        Ok(Self {
            did: document.id().clone(),
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
