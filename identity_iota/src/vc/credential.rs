use core::ops::{Deref, DerefMut};
use identity_core::vc::Credential;
use identity_crypto::SecretKey;
use identity_proof::{HasProof, LdSignature, SignatureOptions};
use serde::{Deserialize, Serialize};

use crate::{did::IotaDocument, error::Result};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct VerifiableCredential {
    #[serde(flatten)]
    inner: Credential,
    proof: LdSignature,
}

impl VerifiableCredential {
    pub fn new(inner: Credential) -> Self {
        Self::with_proof(inner, LdSignature::new("", SignatureOptions::new("")))
    }

    pub fn with_proof(inner: Credential, proof: LdSignature) -> Self {
        Self { inner, proof }
    }

    pub fn sign(&mut self, document: &IotaDocument, secret: &SecretKey) -> Result<()> {
        document.sign_data(self, secret)
    }

    pub fn verify(&self, document: &IotaDocument) -> Result<()> {
        document.verify_data(self)
    }

    pub fn proof(&self) -> &LdSignature {
        &self.proof
    }

    pub fn proof_mut(&mut self) -> &mut LdSignature {
        &mut self.proof
    }
}

impl Deref for VerifiableCredential {
    type Target = Credential;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for VerifiableCredential {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl From<Credential> for VerifiableCredential {
    fn from(other: Credential) -> Self {
        Self::new(other)
    }
}

impl HasProof for VerifiableCredential {
    fn proof(&self) -> &LdSignature {
        VerifiableCredential::proof(self)
    }

    fn proof_mut(&mut self) -> &mut LdSignature {
        VerifiableCredential::proof_mut(self)
    }
}
