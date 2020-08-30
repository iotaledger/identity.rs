use zeroize::Zeroize;

use crate::key::{PublicKey, SecretKey};

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct KeyPair {
  public: PublicKey,
  secret: SecretKey,
}

impl KeyPair {
  pub fn new(public: PublicKey, secret: SecretKey) -> Self {
    Self { public, secret }
  }

  pub fn public(&self) -> &PublicKey {
    &self.public
  }

  pub fn secret(&self) -> &SecretKey {
    &self.secret
  }

  pub fn to_hex(&self) -> (String, String) {
    (self.public.to_hex(), self.secret.to_hex())
  }
}

impl Drop for KeyPair {
  fn drop(&mut self) {
    self.public.zeroize();
    self.secret.zeroize();
  }
}

impl Zeroize for KeyPair {
  fn zeroize(&mut self) {
    self.public.zeroize();
    self.secret.zeroize();
  }
}
