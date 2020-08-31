use crate::{
  canonicalize::{CanonicalJSON, Canonicalize},
  error::Result,
  identity_core::Object,
  key::{KeyGenerator, KeyPair, PublicKey, SecretKey},
};

pub trait SignatureSuite {
  fn signature_type(&self) -> &'static str;

  fn key_type(&self) -> &'static str;

  fn keypair(&self, generator: KeyGenerator) -> Result<KeyPair>;

  fn sign(&self, document: &[u8], secret: &SecretKey) -> Result<Vec<u8>>;

  fn verify(&self, document: &[u8], signature: &[u8], public: &PublicKey) -> Result<bool>;

  fn digest(&self, message: &[u8]) -> Result<Vec<u8>>;

  fn canonicalize(&self, object: Object) -> Result<Vec<u8>> {
    CanonicalJSON::canonicalize(object)
  }
}
