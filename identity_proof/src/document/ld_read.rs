use identity_core::{did::DIDDocument, key::KeyIndex};
use serde::Serialize;

use crate::{
    document::{HasProof, LdDocument},
    error::Result,
    signature::LdSignature,
};

#[derive(Debug, Serialize)]
pub struct LdRead<'a, T> {
    #[serde(flatten)]
    data: &'a T,
    #[serde(skip)]
    document: &'a DIDDocument,
}

impl<'a, T> LdRead<'a, T> {
    pub fn new(data: &'a T, document: &'a DIDDocument) -> Self {
        Self { data, document }
    }
}

impl<T> LdDocument for LdRead<'_, T>
where
    T: HasProof + Serialize,
{
    fn verification_method(&self) -> Option<&str> {
        Some(&*self.data.proof().verification_method)
    }

    fn resolve_key(&self, index: KeyIndex) -> Result<Vec<u8>> {
        LdDocument::resolve_key(self.document, index)
    }

    fn set_proof(&mut self, _value: LdSignature) -> Result<()> {
        Ok(())
    }

    fn set_signature(&mut self, _value: String) -> Result<()> {
        Ok(())
    }
}
