use std::ops::Deref;

use crate::{
  common::{Object, OneOrMany},
  credential::Credential,
};

#[derive(Debug, Deserialize, Serialize)]
pub struct VerifiableCredential {
  #[serde(flatten)]
  credential: Credential,
  proof: OneOrMany<Object>,
}

impl VerifiableCredential {
  pub fn new(credential: Credential, proof: impl Into<OneOrMany<Object>>) -> Self {
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

  pub fn proof(&self) -> &OneOrMany<Object> {
    &self.proof
  }

  pub fn proof_mut(&mut self) -> &mut OneOrMany<Object> {
    &mut self.proof
  }
}

impl Deref for VerifiableCredential {
  type Target = Credential;

  fn deref(&self) -> &Self::Target {
    &self.credential
  }
}
