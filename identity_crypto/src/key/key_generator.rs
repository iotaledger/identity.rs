use zeroize::Zeroize;

use crate::key::SecretKey;

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum KeyGenerator {
  None,
  Seed(Vec<u8>),
  Load(SecretKey),
}

impl Default for KeyGenerator {
  fn default() -> Self {
    Self::None
  }
}

impl Drop for KeyGenerator {
  fn drop(&mut self) {
    match self {
      Self::None => {}
      Self::Seed(ref mut seed) => seed.zeroize(),
      Self::Load(ref mut skey) => skey.zeroize(),
    }
  }
}

impl Zeroize for KeyGenerator {
  fn zeroize(&mut self) {
    match self {
      Self::None => {}
      Self::Seed(ref mut seed) => seed.zeroize(),
      Self::Load(ref mut skey) => skey.zeroize(),
    }
  }
}
