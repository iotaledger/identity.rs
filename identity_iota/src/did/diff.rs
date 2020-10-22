use identity_core::{
    did::{DIDDocument, DiffDIDDocument, DID},
    key::KeyIndex,
};
use identity_proof::{self as proof, LdDocument, LdSignature, SignatureValue};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct DIDDiff {
    pub id: DID,
    pub diff: DiffDIDDocument,
    pub proof: LdSignature,
}

// =============================================================================
// Ld Diff Read
// =============================================================================

#[derive(Debug, Serialize)]
pub(crate) struct LdDiffRead<'a> {
    #[serde(flatten)]
    diff: &'a DIDDiff,
    #[serde(skip)]
    document: &'a DIDDocument,
}

impl<'a> LdDiffRead<'a> {
    pub fn new(diff: &'a DIDDiff, document: &'a DIDDocument) -> Self {
        Self { diff, document }
    }
}

impl LdDocument for LdDiffRead<'_> {
    fn verification_method(&self) -> Option<&str> {
        Some(&*self.diff.proof.verification_method)
    }

    fn resolve_key(&self, index: KeyIndex) -> proof::Result<Vec<u8>> {
        LdDocument::resolve_key(self.document, index)
    }

    fn set_proof(&mut self, _value: LdSignature) -> proof::Result<()> {
        Ok(())
    }

    fn set_signature(&mut self, _value: String) -> proof::Result<()> {
        Ok(())
    }
}

// =============================================================================
// Ld Diff Write
// =============================================================================

#[derive(Debug, Serialize)]
pub(crate) struct LdDiffWrite<'a> {
    #[serde(flatten)]
    diff: &'a mut DIDDiff,
    #[serde(skip)]
    document: &'a DIDDocument,
}

impl<'a> LdDiffWrite<'a> {
    pub fn new(diff: &'a mut DIDDiff, document: &'a DIDDocument) -> Self {
        Self { diff, document }
    }
}

impl LdDocument for LdDiffWrite<'_> {
    fn verification_method(&self) -> Option<&str> {
        Some(&*self.diff.proof.verification_method)
    }

    fn resolve_key(&self, index: KeyIndex) -> proof::Result<Vec<u8>> {
        LdDocument::resolve_key(self.document, index)
    }

    fn set_proof(&mut self, value: LdSignature) -> proof::Result<()> {
        self.diff.proof = value;
        Ok(())
    }

    fn set_signature(&mut self, value: String) -> proof::Result<()> {
        self.diff.proof.data = SignatureValue::Signature(value).into();
        Ok(())
    }
}
