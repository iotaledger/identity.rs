use crate::{
  signature::SignatureSuite,
  error::Result,
  key::{PublicKey, SecretKey},
  proof::{LinkedDataProof, ProofDocument, ProofOptions},
};

pub struct Proof(Box<dyn SignatureSuite>);

impl Proof {
  pub fn new(suite: impl SignatureSuite + 'static) -> Self {
    Self(Box::new(suite))
  }

  pub fn sign(
    &self,
    document: &dyn ProofDocument,
    secret: &SecretKey,
    mut options: ProofOptions,
  ) -> Result<LinkedDataProof> {
    todo!()
  }

  pub fn verify(&self, document: &dyn ProofDocument, proof: &LinkedDataProof, public: &PublicKey) -> Result<bool> {
    todo!()
  }
}
