// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt::Debug;
use core::fmt::Display;
use core::fmt::Formatter;
use core::fmt::Result;
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
  pub method: MethodType,
  pub fragment: String,
  pub key_hash: u64,
}

impl KeyLocation {
  pub fn new(method: MethodType, fragment: String, method_data: &MethodData) -> Self {
    let mut hasher = SeaHasher::new();
    method_data.hash(&mut hasher);
    let key_hash = hasher.finish();

    Self {
      method,
      fragment,
      key_hash,
    }
  }

  /// Generates a random location for a key of the given [`MethodType`].
  pub fn random(method: MethodType) -> Self {
    let mut thread_rng: ThreadRng = rand::thread_rng();
    let fragment: String = (&mut thread_rng)
      .sample_iter(Alphanumeric)
      .take(32)
      .map(char::from)
      .collect();
    let key_hash: u64 = (&mut thread_rng).gen();

    Self {
      method,
      fragment,
      key_hash,
    }
  }

  /// Returns the method type of the key location.
  pub fn method(&self) -> MethodType {
    self.method
  }

  /// Returns the fragment name of the key location.
  pub fn fragment(&self) -> &str {
    &self.fragment
  }

  // TODO: Should probably be removed here and put into an extension trait for Stronghold?
  // Returns a byte representation of the `fragment` and `key_hash`.
  pub fn to_bytes(&self) -> Vec<u8> {
    format!("{}:{}", self.fragment, self.key_hash).into_bytes()
  }
}

impl Display for KeyLocation {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    f.write_fmt(format_args!(
      "({}:{}:{})",
      self.method.as_u32(),
      self.fragment,
      self.key_hash
    ))
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
    let method_data: &MethodData = self.key_data();

    Ok(KeyLocation::new(self.key_type(), fragment.to_owned(), method_data))
  }
}
