use identity_core::{did::DIDDocument, key::KeyIndex};
use serde::Serialize;

use crate::{
    document::{HasProof, LdDocument},
    error::Result,
    signature::{LdSignature, SignatureValue},
};

#[derive(Debug, Serialize)]
pub struct LdWrite<'a, T> {
    #[serde(flatten)]
    data: &'a mut T,
    #[serde(skip)]
    document: &'a DIDDocument,
}

impl<'a, T> LdWrite<'a, T> {
    pub fn new(data: &'a mut T, document: &'a DIDDocument) -> Self {
        Self { data, document }
    }
}

impl<T> LdDocument for LdWrite<'_, T>
where
    T: HasProof + Serialize,
{
    fn verification_method(&self) -> Option<&str> {
        Some(&*self.data.proof().verification_method)
    }

    fn resolve_key(&self, index: KeyIndex) -> Result<Vec<u8>> {
        LdDocument::resolve_key(self.document, index)
    }

    fn set_proof(&mut self, value: LdSignature) -> Result<()> {
        *self.data.proof_mut() = value;
        Ok(())
    }

    fn set_signature(&mut self, value: String) -> Result<()> {
        self.data.proof_mut().data = SignatureValue::Signature(value).into();
        Ok(())
    }
}
