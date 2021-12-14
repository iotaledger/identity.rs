// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt::Debug;
use core::fmt::Formatter;

use crate::common::Timestamp;
use crate::crypto::signature::signature_options::ProofPurpose;
use serde::__private::ser::FlatMapSerializer;
use serde::ser::SerializeMap;
use serde::ser::Serializer;
use serde::Serialize;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;

use crate::crypto::SignatureOptions;
use crate::crypto::SignatureValue;
use crate::error::Result;

/// A digital signature.
///
/// For field definitions see: https://w3c-ccg.github.io/security-vocab/
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Deserialize)]
pub struct Signature {
  #[serde(rename = "type")]
  type_: String,
  #[serde(flatten)]
  value: SignatureValue,
  #[serde(rename = "verificationMethod")]
  method: String,

  /// When the proof was generated.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub created: Option<Timestamp>,
  /// When the proof expires.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub expires: Option<Timestamp>,
  /// Challenge from a proof requester to mitigate replay attacks.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub challenge: Option<String>,
  /// Domain for which a proof is valid to mitigate replay attacks.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub domain: Option<String>,
  /// Purpose for which the proof was generated.
  #[serde(rename = "proofPurpose", skip_serializing_if = "Option::is_none")]
  pub purpose: Option<ProofPurpose>,

  #[serde(default, skip_deserializing)]
  hidden: AtomicBoolCell,
}

impl Signature {
  /// Creates a new [`Signature`] instance with the given `type_` and `method`, with the rest
  /// of its properties left unset.
  pub fn new(type_: impl Into<String>, method: impl Into<String>) -> Self {
    Self::new_with_options(type_, method, SignatureOptions::default())
  }

  /// Creates a new [`Signature`] instance with the given properties.
  pub fn new_with_options(type_: impl Into<String>, method: impl Into<String>, options: SignatureOptions) -> Self {
    Self {
      type_: type_.into(),
      value: SignatureValue::None,
      method: method.into(),
      created: options.created,
      expires: options.expires,
      challenge: options.challenge,
      domain: options.domain,
      purpose: options.purpose,
      hidden: AtomicBoolCell(AtomicBool::new(false)),
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
  fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
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

/// Cell-style wrapper around an AtomicBool.
/// This is essentially a `Cell` but with `Sync` implemented.
pub(crate) struct AtomicBoolCell(AtomicBool);

impl AtomicBoolCell {
  pub(crate) fn set(&self, value: bool) {
    self.0.store(value, Ordering::Relaxed);
  }

  pub(crate) fn get(&self) -> bool {
    self.0.load(Ordering::Relaxed)
  }
}

impl Clone for AtomicBoolCell {
  fn clone(&self) -> Self {
    Self(AtomicBool::new(self.0.load(Ordering::Relaxed)))
  }
}

impl PartialEq for AtomicBoolCell {
  fn eq(&self, other: &Self) -> bool {
    self.0.load(Ordering::Relaxed) == other.0.load(Ordering::Relaxed)
  }
}

impl Eq for AtomicBoolCell {}

impl Default for AtomicBoolCell {
  fn default() -> Self {
    Self(AtomicBool::new(false))
  }
}

impl PartialOrd for AtomicBoolCell {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    self
      .0
      .load(Ordering::Relaxed)
      .partial_cmp(&other.0.load(Ordering::Relaxed))
  }
}

impl Ord for AtomicBoolCell {
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    self.0.load(Ordering::Relaxed).cmp(&other.0.load(Ordering::Relaxed))
  }
}
