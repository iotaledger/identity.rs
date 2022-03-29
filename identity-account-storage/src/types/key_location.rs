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
use rand::distributions::Alphanumeric;
use rand::prelude::ThreadRng;
use rand::Rng;
use seahash::SeaHasher;
use std::hash::Hash;
use std::hash::Hasher;

/// The storage location of a verification method key.
#[derive(Clone, Hash, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct KeyLocation {
  pub key_type: KeyType,
  // TODO: Use `Fragment`?
  pub fragment: String,
  pub key_hash: u64,
}

impl KeyLocation {
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
}

impl Display for KeyLocation {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    f.write_fmt(format_args!("{}:{}", self.fragment, self.key_hash))
  }
}

impl Debug for KeyLocation {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    f.write_fmt(format_args!("KeyLocation({})", self))
  }
}
