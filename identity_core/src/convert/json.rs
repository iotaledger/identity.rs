// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt::Formatter;

use serde::Deserialize;
use serde::Serialize;

use crate::error::Error;
use crate::error::Result;

/// A convenience-trait for types that can be serialized as JSON.
pub trait ToJson: Serialize + Sized {
  /// Serialize `self` as a string of JSON.
  fn to_json(&self) -> Result<String> {
    serde_json::to_string(self).map_err(Error::EncodeJSON)
  }

  /// Serialize `self` as a JSON byte vector.
  fn to_json_vec(&self) -> Result<Vec<u8>> {
    serde_json::to_vec(self).map_err(Error::EncodeJSON)
  }

  /// Serialize `self` as a [`serde_json::Value`].
  fn to_json_value(&self) -> Result<serde_json::Value> {
    serde_json::to_value(self).map_err(Error::EncodeJSON)
  }

  /// Serialize `self` as a pretty-printed string of JSON.
  fn to_json_pretty(&self) -> Result<String> {
    serde_json::to_string_pretty(self).map_err(Error::EncodeJSON)
  }
}

impl<T> ToJson for T where T: Serialize {}

// =============================================================================
// =============================================================================

/// A convenience-trait for types that can be deserialized from JSON.
pub trait FromJson: for<'de> Deserialize<'de> + Sized {
  /// Deserialize `Self` from a string of JSON text.
  fn from_json(json: &(impl AsRef<str> + ?Sized)) -> Result<Self> {
    serde_json::from_str(json.as_ref()).map_err(Error::DecodeJSON)
  }

  /// Deserialize `Self` from bytes of JSON text.
  fn from_json_slice(json: &(impl AsRef<[u8]> + ?Sized)) -> Result<Self> {
    serde_json::from_slice(json.as_ref()).map_err(Error::DecodeJSON)
  }

  /// Deserialize `Self` from a [`serde_json::Value`].
  fn from_json_value(json: serde_json::Value) -> Result<Self> {
    serde_json::from_value(json).map_err(Error::DecodeJSON)
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
