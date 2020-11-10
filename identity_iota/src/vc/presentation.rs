use core::ops::{Deref, DerefMut};
use identity_core::vc::Presentation;
use identity_crypto::SecretKey;
use identity_proof::{HasProof, LdSignature, SignatureOptions};
use serde::{Deserialize, Serialize};

use crate::{did::IotaDocument, error::Result};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct VerifiablePresentation {
    #[serde(flatten)]
    inner: Presentation,
    proof: LdSignature,
}

impl VerifiablePresentation {
    pub fn new(inner: Presentation) -> Self {
        Self::with_proof(inner, LdSignature::new("", SignatureOptions::new("")))
    }

    pub fn with_proof(inner: Presentation, proof: LdSignature) -> Self {
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

impl Deref for VerifiablePresentation {
    type Target = Presentation;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for VerifiablePresentation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl From<Presentation> for VerifiablePresentation {
    fn from(other: Presentation) -> Self {
        Self::new(other)
    }
}

impl HasProof for VerifiablePresentation {
    fn proof(&self) -> &LdSignature {
        VerifiablePresentation::proof(self)
    }

    fn proof_mut(&mut self) -> &mut LdSignature {
        VerifiablePresentation::proof_mut(self)
    }
}
