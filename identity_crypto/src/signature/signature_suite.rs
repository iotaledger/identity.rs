use crate::{
  error::Result,
  key::{KeyGenerator, KeyPair, PublicKey, SecretKey},
};

pub trait SignatureSuite {
  fn keypair(&self, generator: KeyGenerator) -> Result<KeyPair>;

  fn sign(&self, document: &[u8], secret: &SecretKey) -> Result<Vec<u8>>;

  fn verify(&self, document: &[u8], signature: &[u8], public: &PublicKey) -> Result<bool>;

  fn digest(&self, message: &[u8]) -> Result<Vec<u8>>;
}
