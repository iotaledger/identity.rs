// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt::Debug;
use core::fmt::Display;
use core::fmt::Formatter;
use core::fmt::Result;
use identity_core::common::Fragment;
use identity_did::verification::MethodData;
use identity_did::verification::MethodType;
use identity_iota_core::document::IotaVerificationMethod;
use seahash::SeaHasher;
use std::hash::Hash;
use std::hash::Hasher;

use crate::types::Generation;

/// The storage location of a verification method key.
#[derive(Clone, Hash, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct KeyLocation2 {
  method: MethodType,
  fragment: String,
  key_hash: u64,
}

impl KeyLocation2 {
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

impl Display for KeyLocation2 {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    f.write_fmt(format_args!(
      "({}:{}:{})",
      self.method.as_u32(),
      self.fragment,
      self.key_hash
    ))
  }
}

impl Debug for KeyLocation2 {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    f.write_fmt(format_args!("KeyLocation{}", self))
  }
}

/// Returns the [`KeyLocation2`] of an [`IotaVerificationMethod`].
pub fn method_key_location(method: &IotaVerificationMethod) -> KeyLocation2 {
  let fragment: &str = method.id().fragment().expect("TODO");
  let method_data: &MethodData = method.key_data();

  KeyLocation2::new(method.key_type(), fragment.to_owned(), method_data)
}

/// The storage location of a verification method key.
#[derive(Clone, Hash, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct KeyLocation {
  method: MethodType,
  fragment: Fragment,
  generation: Generation,
}

impl KeyLocation {
  /// Creates a new `KeyLocation`.
  pub fn new(method: MethodType, fragment: String, generation: Generation) -> Self {
    Self {
      method,
      fragment: Fragment::new(fragment),

      generation,
    }
  }

  /// Returns the method type of the key location.
  pub fn method(&self) -> MethodType {
    self.method
  }

  /// Returns the fragment name of the key location.
  pub fn fragment(&self) -> &Fragment {
    &self.fragment
  }

  /// Returns the fragment name of the key location.
  pub fn fragment_name(&self) -> &str {
    self.fragment.name()
  }

  /// Returns the integration generation when this key was created.
  pub fn generation(&self) -> Generation {
    self.generation
  }
}

impl Display for KeyLocation {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    f.write_fmt(format_args!(
      "({}:{}:{})",
      self.generation,
      self.fragment,
      self.method.as_u32()
    ))
  }
}

impl Debug for KeyLocation {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    f.write_fmt(format_args!("KeyLocation{}", self))
  }
}
