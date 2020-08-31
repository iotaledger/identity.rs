use identity_crypto::{key::PublicKey, proof::LinkedDataProof};
use std::ops::Deref;

use crate::{common::OneOrMany, error::Result, presentation::Presentation, utils::verify_document};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct VerifiablePresentation {
  #[serde(flatten)]
  presentation: Presentation,
  proof: OneOrMany<LinkedDataProof>,
}

impl VerifiablePresentation {
  pub fn new(presentation: Presentation, proof: impl Into<OneOrMany<LinkedDataProof>>) -> Self {
    Self {
      presentation,
      proof: proof.into(),
    }
  }

  pub fn presentation(&self) -> &Presentation {
    &self.presentation
  }

  pub fn presentation_mut(&mut self) -> &mut Presentation {
    &mut self.presentation
  }

  pub fn proof(&self) -> &OneOrMany<LinkedDataProof> {
    &self.proof
  }

  pub fn proof_mut(&mut self) -> &mut OneOrMany<LinkedDataProof> {
    &mut self.proof
  }

  pub fn verify(&self, resolve: impl Fn(&str) -> Result<PublicKey>) -> Result<bool> {
    verify_document(&self.presentation, &self.proof, resolve)
  }
}

impl Deref for VerifiablePresentation {
  type Target = Presentation;

  fn deref(&self) -> &Self::Target {
    &self.presentation
  }
}
