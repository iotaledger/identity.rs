// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crypto::hashes::sha::Sha256;
use crypto::hashes::Digest;
use crypto::hashes::Output;
use serde::Deserialize;
use serde::Serialize;

/*
We use serde_json::Error for errors in this module because we want these traits to be easy
to implement downstream and serde_json is both stable and has millions of downloads hence
it is relatively safe with regards to stability to include this error type in our public apis.
*/
/// A convenience-trait for types that can be serialized as JSON.
pub trait ToJson: Serialize + Sized {
  /// Serialize `self` as a string of JSON.
  fn to_json(&self) -> Result<String, serde_json::Error> {
    serde_json::to_string(self)
  }

  /// Serialize `self` as a JSON byte vector.
  fn to_json_vec(&self) -> Result<Vec<u8>, serde_json::Error> {
    serde_json::to_vec(self)
  }

  /// Serialize `self` as a [`serde_json::Value`].
  fn to_json_value(&self) -> Result<serde_json::Value, serde_json::Error> {
    serde_json::to_value(self)
  }

  /// Serialize `self` as a pretty-printed string of JSON.
  fn to_json_pretty(&self) -> Result<String, serde_json::Error> {
    serde_json::to_string_pretty(self)
  }

  /// Serialize `self` as a JSON byte vector, normalized using JSON
  /// Canonicalization Scheme (JCS).
  fn to_jcs(&self) -> Result<Vec<u8>, serde_json::Error> {
    serde_jcs::to_vec(self)
  }

  /// Returns the given `data` serialized using JSON Canonicalization Scheme and
  /// hashed using SHA-256.
  fn to_jcs_sha256(&self) -> Result<Output<Sha256>, serde_json::Error> {
    self.to_jcs().map(|json| Sha256::digest(&json))
  }
}

impl<T> ToJson for T where T: Serialize {}

// =============================================================================
// =============================================================================

/// A convenience-trait for types that can be deserialized from JSON.
pub trait FromJson: for<'de> Deserialize<'de> + Sized {
  /// Deserialize `Self` from a string of JSON text.
  fn from_json(json: &(impl AsRef<str> + ?Sized)) -> Result<Self, serde_json::Error> {
    serde_json::from_str(json.as_ref())
  }

  /// Deserialize `Self` from bytes of JSON text.
  fn from_json_slice(json: &(impl AsRef<[u8]> + ?Sized)) -> Result<Self, serde_json::Error> {
    serde_json::from_slice(json.as_ref())
  }

  /// Deserialize `Self` from a [`serde_json::Value`].
  fn from_json_value(json: serde_json::Value) -> Result<Self, serde_json::Error> {
    serde_json::from_value(json)
  }
}

impl<T> FromJson for T where T: for<'de> Deserialize<'de> + Sized {}
