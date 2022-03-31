// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt::Debug;
use core::fmt::Display;
use core::fmt::Formatter;
use core::fmt::Result;
use identity_core::crypto::KeyType;
use identity_did::verification::MethodData;
use identity_did::verification::MethodType;
use identity_iota_core::document::IotaVerificationMethod;
use seahash::SeaHasher;
use std::hash::Hash;
use std::hash::Hasher;

/// The storage location of a verification method key.
///
/// A key is uniquely identified by the fragment and a hash of its public key.
/// Importantly, the fragment alone is insufficient to represent the storage location.
/// For example, when rotating a key, there will be two keys in storage for the
/// same identity with the same fragment. The `key_hash` disambiguates the keys in
/// situations like these.
///
/// The string representation of that location can be obtained via `canonical_repr`.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct KeyLocation {
  /// The [`KeyType`] of the key.
  pub key_type: KeyType,
  /// The fragment of the key.
  fragment: String,
  /// The hash of the public key.
  pub(in crate::types::key_location) key_hash: String,
}

impl KeyLocation {
  /// Create a location from a [`KeyType`], the fragment of a verification method
  /// and the bytes of a public key.
  pub fn new(key_type: KeyType, fragment: String, public_key: &[u8]) -> Self {
    let mut hasher = SeaHasher::new();
    hasher.write(public_key);
    let key_hash: u64 = hasher.finish();

    Self {
      key_type,
      fragment,
      key_hash: key_hash.to_string(),
    }
  }

  /// Obtain the location of a verification method's key in storage.
  pub fn from_verification_method(method: &IotaVerificationMethod) -> crate::Result<Self> {
    let fragment: &str = method
      .id()
      .fragment()
      .ok_or(crate::Error::DIDError(identity_did::Error::MissingIdFragment))?;
    let method_data: &MethodData = method.data();

    let key_type: KeyType = match method.type_() {
      MethodType::Ed25519VerificationKey2018 => KeyType::Ed25519,
      MethodType::X25519KeyAgreementKey2019 => KeyType::X25519,
    };

    let public_key: Vec<u8> = method_data.try_decode()?;

    Ok(KeyLocation::new(key_type, fragment.to_owned(), public_key.as_ref()))
  }

  /// Returns the canonical string representation of the location.
  ///
  /// This should be used as the representation for storage keys.
  pub fn canonical(&self) -> String {
    format!("{}:{}", self.fragment, self.key_hash)
  }
}

impl Display for KeyLocation {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    f.write_str(&self.canonical())
  }
}

// Custom Hash and Equality implementations to not include the key_type.

impl Hash for KeyLocation {
  fn hash<H: Hasher>(&self, state: &mut H) {
    state.write(self.canonical().as_bytes());
  }
}

impl PartialEq for KeyLocation {
  fn eq(&self, other: &Self) -> bool {
    self.fragment == other.fragment && self.key_hash == other.key_hash
  }
}

impl Eq for KeyLocation {}

#[cfg(test)]
mod tests {
  use identity_core::crypto::KeyType;

  use super::KeyLocation;

  // These same test vector should also be tested in Wasm
  // to ensure hashes are consistent across architectures.

  static TEST_VECTOR_1: ([u8; 32], &'static str) = (
    [
      187, 104, 26, 87, 133, 152, 0, 180, 17, 232, 218, 46, 190, 140, 102, 34, 42, 94, 9, 101, 87, 249, 167, 237, 194,
      182, 240, 2, 150, 78, 110, 218,
    ],
    "74874706796298672",
  );

  static TEST_VECTOR_2: ([u8; 32], &'static str) = (
    [
      125, 153, 99, 21, 23, 190, 149, 109, 84, 120, 40, 91, 181, 57, 67, 254, 11, 25, 152, 214, 84, 46, 105, 186, 16,
      39, 141, 151, 100, 163, 138, 222,
    ],
    "10201576743536852223",
  );

  #[test]
  fn test_key_location_canonical_representation() {
    for (test_vector, expected_hash) in [TEST_VECTOR_1, TEST_VECTOR_2] {
      let fragment: String = rand::Rng::sample_iter(rand::thread_rng(), rand::distributions::Alphanumeric)
        .take(32)
        .map(char::from)
        .collect::<String>();

      let location: KeyLocation = KeyLocation::new(KeyType::Ed25519, fragment.clone(), &test_vector);

      let canonical_repr: String = location.canonical();

      let mut parts = canonical_repr.split(':');

      let fragment_str: &str = parts.next().unwrap();
      let key_hash_str: &str = parts.next().unwrap();

      assert_eq!(fragment_str, &fragment);
      assert_eq!(key_hash_str, expected_hash);
    }
  }
}
