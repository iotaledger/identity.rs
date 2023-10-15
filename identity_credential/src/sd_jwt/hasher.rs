use crypto::hashes::sha::{SHA256, SHA256_LEN};
use identity_core::convert::{Base, BaseEncoding};

///
pub trait Hasher {
  ///
  fn digest(&self, input: &str) -> String;
  ///
  fn alg_name() -> &'static str;
}

///
pub struct ShaHasher {}

impl ShaHasher {
  ///
  pub fn new() -> Self {
    ShaHasher {}
  }
}

impl Hasher for ShaHasher {
  fn digest(&self, input: &str) -> String {
    let mut digest: [u8; SHA256_LEN] = Default::default();
    SHA256(input.as_bytes(), &mut digest);
    BaseEncoding::encode(digest.as_ref(), Base::Base64Url)
  }

  fn alg_name() -> &'static str {
    "sha"
  }
}

#[cfg(test)]
mod test {
  use super::{Hasher, ShaHasher};

  #[test]
  fn test() {
    let hasher = ShaHasher::new();
    let hash: String = hasher
      .digest("WyJlSThaV205UW5LUHBOUGVOZW5IZGhRIiwgImVtYWlsIiwgIlwidW51c3VhbCBlbWFpbCBhZGRyZXNzXCJAZXhhbXBsZS5qcCJd");
    assert_eq!(hash, "Kuet1yAa0HIQvYnOVd59hcViO9Ug6J2kSfqYRBeowvE".to_owned());
  }
}
