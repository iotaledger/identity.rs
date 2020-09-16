use crate::crypto::message_digest;
use crate::error::Result;

pub const SHA256: HashAlgorithm = HashAlgorithm::Sha256;
pub const SHA384: HashAlgorithm = HashAlgorithm::Sha384;
pub const SHA512: HashAlgorithm = HashAlgorithm::Sha512;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum HashAlgorithm {
  Sha256,
  Sha384,
  Sha512,
}

impl HashAlgorithm {
  pub const fn size(&self) -> usize {
    match self {
      Self::Sha256 => 32,
      Self::Sha384 => 48,
      Self::Sha512 => 64,
    }
  }

  pub fn digest(self, message: impl AsRef<[u8]>) -> Result<Vec<u8>> {
    message_digest(self, message.as_ref())
  }
}
