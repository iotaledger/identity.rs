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
/// The string representation of that location can be obtained via `to_string`.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct KeyLocation {
  /// The [`KeyType`] of the key.
  pub key_type: KeyType,
  /// The fragment of the key.
  fragment: String,
  /// The hash of the public key.
  key_hash: u64,
}

impl KeyLocation {
  /// Create a location from a [`KeyType`], the fragment of a verification method
  /// and the bytes of a public key.
  pub fn new(key_type: KeyType, fragment: String, public_key: &[u8]) -> Self {
    let mut hasher = SeaHasher::new();
    public_key.hash(&mut hasher);
    let key_hash = hasher.finish();

    Self {
      key_type,
      fragment,
      key_hash,
    }
  }

  /// Create the [`KeyLocation`] of an [`IotaVerificationMethod`].
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
  pub fn canonical_repr(&self) -> String {
    format!("{}:{}", self.fragment, self.key_hash)
  }
}

impl Display for KeyLocation {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    f.write_str(&self.canonical_repr())
  }
}

// Custom Hash and Equality implementations to not include the key_type.

impl Hash for KeyLocation {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.fragment.hash(state);
    self.key_hash.hash(state);
  }
}

impl PartialEq for KeyLocation {
  fn eq(&self, other: &Self) -> bool {
    self.fragment == other.fragment && self.key_hash == other.key_hash
  }
}

impl Eq for KeyLocation {}
