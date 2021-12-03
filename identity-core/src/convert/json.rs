// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt::Formatter;

use crypto::hashes::sha::Sha256;
use crypto::hashes::Digest;
use crypto::hashes::Output;
use serde::Deserialize;
use serde::Serialize;

pub use self::errors::JsonDecodingError;
pub use self::errors::JsonEncodingError;

/// A convenience-trait for types that can be serialized as JSON.
pub trait ToJson: Serialize + Sized {
  /// Serialize `self` as a string of JSON.
  fn to_json(&self) -> Result<String, JsonEncodingError> {
    serde_json::to_string(self).map_err(From::from)
  }

  /// Serialize `self` as a JSON byte vector.
  fn to_json_vec(&self) -> Result<Vec<u8>, JsonEncodingError> {
    serde_json::to_vec(self).map_err(From::from)
  }

  /// Serialize `self` as a [`serde_json::Value`].
  fn to_json_value(&self) -> Result<serde_json::Value, JsonEncodingError> {
    serde_json::to_value(self).map_err(From::from)
  }

  /// Serialize `self` as a pretty-printed string of JSON.
  fn to_json_pretty(&self) -> Result<String, JsonEncodingError> {
    serde_json::to_string_pretty(self).map_err(From::from)
  }

  /// Serialize `self` as a JSON byte vector, normalized using JSON
  /// Canonicalization Scheme (JCS).
  fn to_jcs(&self) -> Result<Vec<u8>, JsonEncodingError> {
    serde_jcs::to_vec(self).map_err(From::from)
  }

  /// Returns the given `data` serialized using JSON Canonicalization Scheme and
  /// hashed using SHA-256.
  fn to_jcs_sha256(&self) -> Result<Output<Sha256>, JsonEncodingError> {
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

// =============================================================================
// =============================================================================

/// A convenience-trait to format types as JSON strings for display.
pub trait FmtJson: ToJson {
  /// Format this as a JSON string or pretty-JSON string based on whether the `#` format flag
  /// was used.
  fn fmt_json(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    if f.alternate() {
      f.write_str(&self.to_json_pretty().map_err(|_| core::fmt::Error)?)
    } else {
      f.write_str(&self.to_json().map_err(|_| core::fmt::Error)?)
    }
  }
}

impl<T> FmtJson for T where T: ToJson {}

// =============================================================================
// =============================================================================

mod errors {
  use thiserror::Error as DeriveError;
  /*
  We implement From serde_json::Error for the errors in this module because we want the ToJson and FromJson traits to be easy
  to implement downstream and serde_json is both stable and has millions of downloads hence
  it is relatively safe with regards to stability to include this error type in our public API.
  For more interoperability we also implement Into serde_json::Error
  */

  /// Caused by a failure to encode Rust types as JSON
  #[derive(Debug, DeriveError)]
  #[error("failed to encode JSON: {cause}")]
  pub struct JsonEncodingError {
    #[from]
    cause: serde_json::Error,
  }

  impl From<JsonEncodingError> for serde_json::Error {
    fn from(err: JsonEncodingError) -> Self {
      err.cause
    }
  }

  /// Caused by a failure to decode Rust types as JSON
  #[derive(Debug, DeriveError)]
  #[error("failed to decode JSON: {cause}")]
  pub struct JsonDecodingError {
    #[from]
    cause: serde_json::Error,
  }

  impl From<JsonDecodingError> for serde_json::Error {
    fn from(err: JsonDecodingError) -> Self {
      err.cause
    }
  }
}
