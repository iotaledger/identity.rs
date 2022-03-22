// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::str::FromStr;
use core::fmt::Display;
use core::fmt::Formatter;

use crate::crypto::merkle_key::MerkleDigest;
use crate::crypto::merkle_key::MerkleKey;
use crate::crypto::merkle_tree::Hash;
use crate::crypto::Ed25519;
use crate::error::Error;
use crate::error::Result;

/// Supported cryptographic key types.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub enum KeyType {
  /// An `Ed25519` cryptographic key.
  Ed25519,
  /// An `X25519` cryptographic key.
  X25519,
}

impl KeyType {
  /// Returns the [`KeyType`] name as a static `str`.
  pub const fn as_str(&self) -> &'static str {
    match self {
      Self::Ed25519 => "Ed25519",
      Self::X25519 => "X25519",
    }
  }

  /// Creates a DID Document public key value for the given Merkle `root`.
  pub fn encode_merkle_key<D>(&self, root: &Hash<D>) -> Vec<u8>
  where
    D: MerkleDigest,
  {
    match self {
      Self::Ed25519 => MerkleKey::encode_key::<D, Ed25519>(root),
      _ => panic!("X25519 not supported for MerkleKeyCollection"),
    }
  }
}

impl FromStr for KeyType {
  type Err = Error;

  fn from_str(string: &str) -> Result<Self, Self::Err> {
    if string.eq_ignore_ascii_case("Ed25519") {
      Ok(Self::Ed25519)
    } else if string.eq_ignore_ascii_case("X25519") {
      Ok(Self::X25519)
    } else {
      Err(Error::InvalidKeyFormat)
    }
  }
}

impl Display for KeyType {
  fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    f.write_str(self.as_str())
  }
}

#[cfg(test)]
mod tests {
  use serde_json::Value;
  use super::*;

  #[test]
  fn test_key_type_serde() {
    for key_type in [KeyType::Ed25519, KeyType::X25519] {
      let ser: Value = serde_json::to_value(&key_type).unwrap();
      assert_eq!(ser.as_str().unwrap(), key_type.as_str());
      let de: KeyType = serde_json::from_value(ser.clone()).unwrap();
      assert_eq!(de, key_type);

      assert_eq!(KeyType::from_str(key_type.as_str()).unwrap(), key_type);
      assert_eq!(KeyType::from_str(ser.as_str().unwrap()).unwrap(), key_type);
    }
  }
}
