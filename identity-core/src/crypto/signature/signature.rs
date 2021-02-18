// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::cell::Cell;
use core::fmt::Debug;
use core::fmt::Formatter;
use core::fmt::Result as FmtResult;
use serde::__private::ser::FlatMapSerializer;
use serde::ser::SerializeMap;
use serde::ser::Serializer;
use serde::Serialize;

use crate::crypto::SignatureValue;
use crate::error::Result;

/// A DID Document digital signature.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Deserialize)]
pub struct Signature {
  #[serde(rename = "type")]
  type_: String,
  #[serde(flatten)]
  value: SignatureValue,
  #[serde(rename = "verificationMethod")]
  method: String,
  #[serde(default, skip_deserializing)]
  hidden: Cell<bool>,
}

impl Signature {
  /// Creates a new [`Signature`] instance with the given `type_` and `method`.
  pub fn new(type_: impl Into<String>, method: impl Into<String>) -> Self {
    Self {
      type_: type_.into(),
      value: SignatureValue::None,
      method: method.into(),
      hidden: Cell::new(false),
    }
  }

  /// Returns the `type` property of the signature.
  pub fn type_(&self) -> &str {
    &*self.type_
  }

  /// Returns the identifier of the DID method used to create this signature.
  pub fn verification_method(&self) -> &str {
    &*self.method
  }

  /// Returns a reference to the signature `value`.
  pub const fn value(&self) -> &SignatureValue {
    &self.value
  }

  /// Returns a mutable reference to the signature `value`.
  pub fn value_mut(&mut self) -> &mut SignatureValue {
    &mut self.value
  }

  /// Sets the [`SignatureValue`] of the object.
  pub fn set_value(&mut self, value: SignatureValue) {
    self.value = value;
  }

  /// Clears the current signature value - all other properties are unchanged.
  pub fn clear_value(&mut self) {
    self.value = SignatureValue::None;
  }

  /// Flag the signature value so it is ignored during serialization
  pub fn hide_value(&self) {
    self.hidden.set(true);
  }

  /// Restore the signature value state so serialization behaves normally
  pub fn show_value(&self) {
    self.hidden.set(false);
  }

  fn __hide(&self) -> bool {
    self.hidden.get() || self.value.is_none()
  }
}

impl Debug for Signature {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    f.debug_struct("Signature")
      .field("type_", &self.type_)
      .field("value", &self.value)
      .field("method", &self.method)
      .finish()
  }
}

impl Serialize for Signature {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let hide: bool = self.__hide();

    let mut state: S::SerializeMap = if hide {
      serializer.serialize_map(Some(1 + 5))?
    } else {
      serializer.serialize_map(Some(2 + 5))?
    };

    state.serialize_entry("type", &self.type_)?;
    state.serialize_entry("verificationMethod", &self.method)?;

    if !hide {
      Serialize::serialize(&self.value, FlatMapSerializer(&mut state))?;
    }

    state.end()
  }
}
