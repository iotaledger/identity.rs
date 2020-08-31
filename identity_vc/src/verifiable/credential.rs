use identity_crypto::{key::PublicKey, proof::LinkedDataProof};
use std::ops::Deref;

use crate::{common::OneOrMany, credential::Credential, error::Result, utils::verify_document};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct VerifiableCredential {
  #[serde(flatten)]
  credential: Credential,
  proof: OneOrMany<LinkedDataProof>,
}

impl VerifiableCredential {
  pub fn new(credential: Credential, proof: impl Into<OneOrMany<LinkedDataProof>>) -> Self {
    Self {
      credential,
      proof: proof.into(),
    }
  }

  pub fn credential(&self) -> &Credential {
    &self.credential
  }

  pub fn credential_mut(&mut self) -> &mut Credential {
    &mut self.credential
  }

  pub fn proof(&self) -> &OneOrMany<LinkedDataProof> {
    &self.proof
  }

  pub fn proof_mut(&mut self) -> &mut OneOrMany<LinkedDataProof> {
    &mut self.proof
  }

  pub fn verify(&self, resolve: impl Fn(&str) -> Result<PublicKey>) -> Result<bool> {
    verify_document(&self.credential, &self.proof, resolve)
  }
}

impl Deref for VerifiableCredential {
  type Target = Credential;

  fn deref(&self) -> &Self::Target {
    &self.credential
  }
}
