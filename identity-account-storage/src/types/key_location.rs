// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt::Debug;
use core::fmt::Display;
use core::fmt::Formatter;
use core::fmt::Result;
use identity_core::common::Fragment;
use identity_core::crypto::KeyType;
use identity_did::verification::MethodData;
use identity_did::verification::MethodType;
use identity_iota_core::document::IotaVerificationMethod;
use rand::distributions::Alphanumeric;
use rand::prelude::ThreadRng;
use rand::Rng;
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
#[derive(Clone, Hash, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct KeyLocation {
  key_type: KeyType,
  fragment: Fragment,
  key_hash: u64,
}

impl KeyLocation {
  pub fn new(key_type: KeyType, fragment: String, public_key: &[u8]) -> Self {
    let mut hasher = SeaHasher::new();
    public_key.hash(&mut hasher);
    let key_hash = hasher.finish();

    Self {
      key_type,
      fragment: Fragment::new(fragment),
      key_hash,
    }
  }

  /// Generates a random location for a key of the given [`KeyType`].
  pub fn random(key_type: KeyType) -> Self {
    let mut thread_rng: ThreadRng = rand::thread_rng();
    let fragment: String = (&mut thread_rng)
      .sample_iter(Alphanumeric)
      .take(32)
      .map(char::from)
      .collect();
    let key_hash: u64 = (&mut thread_rng).gen();

    Self {
      key_type,
      fragment: Fragment::new(fragment),
      key_hash,
    }
  }

  /// Returns the [`KeyType`] of the key at the location.
  pub fn key_type(&self) -> KeyType {
    self.key_type
  }

  /// Returns the fragment without the leading `#`.
  pub fn fragment(&self) -> &str {
    self.fragment.name()
  }

  /// Returns the hash of the public key at the location.
  pub fn key_hash(&self) -> u64 {
    self.key_hash
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
}

impl Display for KeyLocation {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    f.write_fmt(format_args!("{}:{}", self.fragment.name(), self.key_hash))
  }
}

impl Debug for KeyLocation {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    f.write_fmt(format_args!("KeyLocation({})", self))
  }
}
