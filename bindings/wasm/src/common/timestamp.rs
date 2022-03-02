// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity::core::Timestamp;
use wasm_bindgen::prelude::*;

use crate::error::Result;
use crate::error::WasmResult;

#[wasm_bindgen(js_name = Timestamp, inspectable)]
pub struct WasmTimestamp(pub(crate) Timestamp);

#[wasm_bindgen(js_class = Timestamp)]
impl WasmTimestamp {
  /// Parses a `Timestamp` from the provided input string.
  #[wasm_bindgen]
  pub fn parse(input: &str) -> Result<WasmTimestamp> {
    Ok(Self(Timestamp::parse(input).wasm_result()?))
  }

  /// Creates a new `Timestamp` with the current date and time.
  #[wasm_bindgen(js_name = nowUTC)]
  pub fn now_utc() -> Self {
    Self(Timestamp::now_utc())
  }

  /// Returns the `Timestamp` as an RFC 3339 `String`.
  #[wasm_bindgen(js_name = toRFC3339)]
  #[allow(clippy::wrong_self_convention)]
  pub fn to_rfc3339(&self) -> String {
    self.0.to_rfc3339()
  }

  /// Returns the `Timestamp` as a Unix timestamp.
  #[wasm_bindgen(js_name = toUnix)]
  #[allow(clippy::wrong_self_convention)]
  pub fn to_unix(&self) -> i64 {
    self.0.unix_timestamp()
  }

  /// Creates a new `Timestamp` from the given Unix timestamp.
  ///
  /// The timestamp must be in the valid range for [RFC 3339](https://tools.ietf.org/html/rfc3339).
  ///
  /// # Errors
  /// [`Error::InvalidTimestamp`] if `seconds` is outside of the interval [-62167219200,253402300799].
  pub fn from_unix(seconds: i64) -> Result<Self> {
    let offset_date_time = OffsetDateTime::from_unix_timestamp(seconds).map_err(time::error::Error::from)?;
    // Reject years outside of the range 0000AD - 9999AD per Rfc3339
    // upfront to prevent conversion errors in to_rfc3339().
    // https://datatracker.ietf.org/doc/html/rfc3339#section-1
    if !(0..10_000).contains(&offset_date_time.year()) {
      return Err(time::error::Error::Format(time::error::Format::InvalidComponent("invalid year")).into());
    }
    Ok(Self(offset_date_time))
  }
}

impl From<Timestamp> for WasmTimestamp {
  fn from(timestamp: Timestamp) -> Self {
    Self(timestamp)
  }
}
