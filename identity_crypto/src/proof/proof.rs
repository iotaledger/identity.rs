use crate::{
  error::Result,
  key::{KeyGenerator, KeyPair, PublicKey, SecretKey},
  proof::{LinkedDataProof, ProofDocument, ProofOptions},
  signature::SignatureSuite,
};

pub struct Proof(Box<dyn SignatureSuite>);

impl Proof {
  pub fn new(suite: impl SignatureSuite + 'static) -> Self {
    Self(Box::new(suite))
  }

  pub fn signature_type(&self) -> &'static str {
    self.0.signature_type()
  }

  pub fn key_type(&self) -> &'static str {
    self.0.key_type()
  }

  pub fn keypair(&self, generator: KeyGenerator) -> Result<KeyPair> {
    self.0.keypair(generator)
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
