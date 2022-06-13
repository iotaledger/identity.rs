// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt::Debug;
use core::fmt::Formatter;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;

use serde;
use serde::__private::ser::FlatMapSerializer;
use serde::ser::SerializeMap;
use serde::ser::Serializer;
use serde::Deserialize;
use serde::Serialize;

use crate::common::Timestamp;
use crate::crypto::ProofOptions;
use crate::crypto::ProofPurpose;
use crate::crypto::ProofValue;
use crate::error::Result;

/// A digital signature.
///
/// For field definitions see: [the WC3 Security vocabulary specification](https://w3c-ccg.github.io/security-vocab/). 
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Deserialize)]
pub struct Proof {
  #[serde(rename = "type")]
  type_: String,
  #[serde(flatten)]
  value: ProofValue,
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

impl Proof {
  /// Creates a new [`Proof`] instance with the given `type_` and `method`, with the rest
  /// of its properties left unset.
  pub fn new(type_: impl Into<String>, method: impl Into<String>) -> Self {
    Self::new_with_options(type_, method, ProofOptions::default())
  }

  /// Creates a new [`Proof`] instance with the given properties.
  pub fn new_with_options(type_: impl Into<String>, method: impl Into<String>, options: ProofOptions) -> Self {
    Self {
      type_: type_.into(),
      value: ProofValue::None,
      method: method.into(),
      created: options.created,
      expires: options.expires,
      challenge: options.challenge,
      domain: options.domain,
      purpose: options.purpose,
      hidden: AtomicBoolCell(AtomicBool::new(false)),
    }
  }

  /// Returns the `type` property of the proof.
  pub fn type_(&self) -> &str {
    &*self.type_
  }

  /// Returns the identifier of the DID method used to create this proof.
  pub fn verification_method(&self) -> &str {
    &*self.method
  }

  /// Returns a reference to the proof `value`.
  pub const fn value(&self) -> &ProofValue {
    &self.value
  }

  /// Returns a mutable reference to the proof `value`.
  pub fn value_mut(&mut self) -> &mut ProofValue {
    &mut self.value
  }

  /// Sets the [`ProofValue`] of the object.
  pub fn set_value(&mut self, value: ProofValue) {
    self.value = value;
  }

  /// Clears the current proof value - all other properties are unchanged.
  pub fn clear_value(&mut self) {
    self.value = ProofValue::None;
  }

  /// Flag the proof value so it is ignored during serialization
  pub fn hide_value(&self) {
    self.hidden.set(true);
  }

  /// Restore the proof value state so serialization behaves normally
  pub fn show_value(&self) {
    self.hidden.set(false);
  }

  fn __hide(&self) -> bool {
    self.hidden.get() || self.value.is_none()
  }
}

impl Debug for Proof {
  fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    f.debug_struct("Proof")
      .field("type_", &self.type_)
      .field("value", &self.value)
      .field("method", &self.method)
      .field("created", &self.created)
      .field("expires", &self.expires)
      .field("challenge", &self.challenge)
      .field("domain", &self.domain)
      .field("purpose", &self.purpose)
      .finish()
  }
}

impl Serialize for Proof {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let hide: bool = self.__hide();

    let mut count_fields: usize = if hide {
      2 // type + method
    } else {
      3 // type + method + value
    };
    count_fields += if self.created.is_some() { 1 } else { 0 };
    count_fields += if self.expires.is_some() { 1 } else { 0 };
    count_fields += if self.challenge.is_some() { 1 } else { 0 };
    count_fields += if self.domain.is_some() { 1 } else { 0 };
    count_fields += if self.purpose.is_some() { 1 } else { 0 };
    let mut state: S::SerializeMap = serializer.serialize_map(Some(count_fields))?;

    state.serialize_entry("type", &self.type_)?;
    state.serialize_entry("verificationMethod", &self.method)?;
    if !hide {
      Serialize::serialize(&self.value, FlatMapSerializer(&mut state))?;
    }

    if let Some(created) = &self.created {
      state.serialize_entry("created", &created)?;
    }
    if let Some(expires) = &self.expires {
      state.serialize_entry("expires", &expires)?;
    }
    if let Some(challenge) = &self.challenge {
      state.serialize_entry("challenge", &challenge)?;
    }
    if let Some(domain) = &self.domain {
      state.serialize_entry("domain", &domain)?;
    }
    if let Some(purpose) = &self.purpose {
      state.serialize_entry("proofPurpose", &purpose)?;
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

#[cfg(test)]
mod tests {
  use std::str::FromStr;

  use crate::convert::FromJson;
  use crate::convert::ToJson;
  use crate::json;

  use super::*;

  fn generate_options() -> ProofOptions {
    ProofOptions {
      created: Some(Timestamp::from_str("1970-01-01T00:00:00Z").unwrap()),
      expires: Some(Timestamp::from_str("2000-01-01T00:00:00Z").unwrap()),
      challenge: Some("some-challenge".to_owned()),
      domain: Some("some.domain".to_owned()),
      purpose: Some(ProofPurpose::Authentication),
    }
  }

  #[test]
  fn test_signature_serialise() {
    let signature: Proof = Proof::new_with_options("JcsEd25519Signature2020", "#sign-0", ProofOptions::default());
    let expected = r##"{"type":"JcsEd25519Signature2020","verificationMethod":"#sign-0"}"##;
    assert_eq!(signature.to_json().unwrap(), expected);
  }

  #[test]
  fn test_signature_serialise_options() {
    let mut signature: Proof = Proof::new_with_options("JcsEd25519Signature2020", "#sign-0", generate_options());
    signature.set_value(ProofValue::Signature("somesignaturevalue123456789".to_owned()));
    // Compare against JSON to ignore field order.
    let expected = json!({
      "type":"JcsEd25519Signature2020",
      "verificationMethod":"#sign-0",
      "signatureValue":"somesignaturevalue123456789",
      "created":"1970-01-01T00:00:00Z",
      "expires":"2000-01-01T00:00:00Z",
      "challenge":"some-challenge",
      "domain":"some.domain",
      "proofPurpose":"authentication",
    });
    assert_eq!(signature.to_json_value().unwrap(), expected);
  }

  #[test]
  fn test_signature_json() {
    let mut signature: Proof = Proof::new_with_options("JcsEd25519Signature2020", "#sign-0", generate_options());
    signature.set_value(ProofValue::Signature("somesignaturevalue123456789".to_owned()));

    let deserialized: Proof = Proof::from_json(&signature.to_json().unwrap()).unwrap();
    assert_eq!(signature, deserialized);
  }
}
