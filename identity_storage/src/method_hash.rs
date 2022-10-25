// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt::Debug;
use core::fmt::Display;
use core::fmt::Formatter;
use identity_did::verification::MethodData;
use identity_did::verification::MethodType;
use identity_did::verification::VerificationMethod;
use seahash::SeaHasher;
use std::hash::Hash;
use std::hash::Hasher;
use std::str::FromStr;

/// The unique identifier of a verification method.
///
/// A method is uniquely identified by a hash of the fragment, method type and method data.
#[derive(Debug, Clone, Hash, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MethodHash {
  /// The hash of method type and method data.
  pub(crate) hash: u64,
}

impl MethodHash {
  /// Create a location from a [`KeyType`], the fragment of a verification method
  /// and the bytes of a public key.
  fn new(fragment: &str, method_type: &MethodType, method_data: &MethodData) -> Self {
    let mut hasher = SeaHasher::new();
    // We use `hasher.write(slice)` instead of `slice.hash(&mut hasher)` because the latter
    // writes the len of the slice into the hash which differs in length on 32- and 64-bit platforms,
    // such as Wasm and x86_64, resulting in different hashes.
    hasher.write(fragment.as_bytes());
    hasher.write(method_type.as_str().as_bytes());

    // TODO: Perhaps use try_decode on `MethodData` instead to make
    // it less likely to miss handling a new variant in MethodData.
    match method_data {
      MethodData::PublicKeyMultibase(string) => {
        hasher.write(string.as_bytes());
      }
      MethodData::PublicKeyBase58(string) => {
        hasher.write(string.as_bytes());
      }
      _ => todo!("TODO: return error"),
    }

    let key_hash: u64 = hasher.finish();

    Self { hash: key_hash }
  }

  /// Obtain the [`MethodHash`] of a verification method.
  // TODO: Implement TryFrom<&VerificationMethod> instead?
  pub fn from_verification_method(method: &VerificationMethod) -> crate::Result<Self> {
    let fragment: &str = method
      .id()
      .fragment()
      .ok_or_else(|| crate::Error::MethodHashConstruction("missing fragment on method".to_owned()))?;
    Ok(MethodHash::new(fragment, method.type_(), method.data()))
  }
}

impl FromStr for MethodHash {
  type Err = crate::Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    s.parse::<u64>()
      .map_err(|e| crate::Error::MethodHashConstruction(e.to_string()))
      .map(|hash| Self { hash })
  }
}

impl Display for MethodHash {
  fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    f.write_str(&self.hash.to_string())
  }
}

// TODO:
/*
#[cfg(test)]
mod tests {
  use identity_core::crypto::KeyType;
  use rand::distributions::DistString;
  use rand::rngs::OsRng;

  use super::MethodHash;

  // These same test vector should also be tested in Wasm
  // to ensure hashes are consistent across architectures.

  static TEST_VECTOR_1: ([u8; 32], &str) = (
    [
      187, 104, 26, 87, 133, 152, 0, 180, 17, 232, 218, 46, 190, 140, 102, 34, 42, 94, 9, 101, 87, 249, 167, 237, 194,
      182, 240, 2, 150, 78, 110, 218,
    ],
    "74874706796298672",
  );

  static TEST_VECTOR_2: ([u8; 32], &str) = (
    [
      125, 153, 99, 21, 23, 190, 149, 109, 84, 120, 40, 91, 181, 57, 67, 254, 11, 25, 152, 214, 84, 46, 105, 186, 16,
      39, 141, 151, 100, 163, 138, 222,
    ],
    "10201576743536852223",
  );

  #[test]
  fn test_key_location_canonical_representation() {
    for (test_vector, expected_hash) in [TEST_VECTOR_1, TEST_VECTOR_2] {
      let fragment: String = rand::distributions::Alphanumeric.sample_string(&mut OsRng, 32);

      let location: MethodHash = MethodHash::new(KeyType::Ed25519, fragment.clone(), &test_vector);

      let canonical_repr: String = location.canonical();

      let mut parts = canonical_repr.split(':');

      let fragment_str: &str = parts.next().unwrap();
      let key_hash_str: &str = parts.next().unwrap();

      assert_eq!(fragment_str, &fragment);
      assert_eq!(key_hash_str, expected_hash);
    }
  }
}
*/
