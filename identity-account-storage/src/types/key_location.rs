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
pub struct KeyLocation {
  pub key_type: KeyType,
  pub fragment: String,
  pub key_hash: u64,
}

impl KeyLocation {
  pub fn new(key_type: KeyType, fragment: String, method_data: &MethodData) -> Self {
    let mut hasher = SeaHasher::new();
    method_data.hash(&mut hasher);
    let key_hash = hasher.finish();

    Self {
      key_type,
      fragment,
      key_hash,
    }
  }

  /// Generates a random location for a key of the given [`MethodType`].
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

  // TODO: Should probably be removed here and put into an extension trait for Stronghold?
  // Returns a byte representation of the `fragment` and `key_hash`.
  pub fn to_bytes(&self) -> Vec<u8> {
    format!("{}:{}", self.fragment, self.key_hash).into_bytes()
  }
}

impl Display for KeyLocation {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    f.write_fmt(format_args!("({}:{})", self.fragment, self.key_hash))
  }
}

impl Debug for KeyLocation {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    f.write_fmt(format_args!("KeyLocation{}", self))
  }
}

pub trait IotaVerificationMethodExt {
  /// Returns the [`KeyLocation`] of an [`IotaVerificationMethod`].
  fn key_location(&self) -> crate::Result<KeyLocation>;
}

impl IotaVerificationMethodExt for IotaVerificationMethod {
  fn key_location(&self) -> crate::Result<KeyLocation> {
    let fragment: &str = self
      .id()
      .fragment()
      .ok_or(crate::Error::DIDError(identity_did::Error::MissingIdFragment))?;
    let method_data: &MethodData = self.data();

    let key_type: KeyType = method_to_key_type(self.type_());

    Ok(KeyLocation::new(key_type, fragment.to_owned(), method_data))
  }
}

pub fn method_to_key_type(method_type: MethodType) -> KeyType {
  match method_type {
    MethodType::Ed25519VerificationKey2018 => KeyType::Ed25519,
    MethodType::X25519KeyAgreementKey2019 => KeyType::X25519,
    MethodType::MerkleKeyCollection2021 => todo!(),
  }
}
