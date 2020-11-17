use identity_core::did_doc::{SetSignature, Signature, TrySignature, TrySignatureMut};

use crate::{
    did::{IotaDID, IotaDocument},
    error::Result,
};

pub type Diff = (); // TODO: FIXME

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct DIDDiff<'a> {
    pub did: IotaDID<'a>,
    pub diff: Diff,
    pub proof: Option<Signature>,
}

impl<'a> DIDDiff<'a> {
    pub fn new(document: &'a IotaDocument, _other: &IotaDocument) -> Result<Self> {
        let diff: Diff = ();

        Ok(Self {
            did: document.id(),
            diff,
            proof: None,
        })
    }
}

impl TrySignature for DIDDiff<'_> {
    fn try_signature(&self) -> Option<&Signature> {
        self.proof.as_ref()
    }
}

impl TrySignatureMut for DIDDiff<'_> {
    fn try_signature_mut(&mut self) -> Option<&mut Signature> {
        self.proof.as_mut()
    }
}

impl SetSignature for DIDDiff<'_> {
    fn set_signature(&mut self, value: Signature) {
        self.proof = Some(value);
    }
}
