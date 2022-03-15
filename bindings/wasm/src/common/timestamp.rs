// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity::core::Duration;
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

  /// Computes `self + duration`
  ///
  /// Returns `null` if the operation leads to a timestamp not in the valid range for [RFC 3339](https://tools.ietf.org/html/rfc3339).
  #[wasm_bindgen(js_name = checkedAdd)]
  pub fn checked_add(&self, duration: &WasmDuration) -> Option<WasmTimestamp> {
    self.0.checked_add(duration.0).map(WasmTimestamp)
  }

  /// Computes `self - duration`
  ///
  /// Returns `null` if the operation leads to a timestamp not in the valid range for [RFC 3339](https://tools.ietf.org/html/rfc3339).
  #[wasm_bindgen(js_name = checkedSub)]
  pub fn checked_sub(&self, duration: &WasmDuration) -> Option<WasmTimestamp> {
    self.0.checked_sub(duration.0).map(WasmTimestamp)
  }

  /// Serializes a `Timestamp` as a JSON object.
  #[wasm_bindgen(js_name = toJSON)]
  pub fn to_json(&self) -> Result<JsValue> {
    JsValue::from_serde(&self.0).wasm_result()
  }

  /// Deserializes a `Timestamp` from a JSON object.
  #[wasm_bindgen(js_name = fromJSON)]
  pub fn from_json(json: &JsValue) -> Result<WasmTimestamp> {
    json.into_serde().map(Self).wasm_result()
  }
}

impl From<Timestamp> for WasmTimestamp {
  fn from(timestamp: Timestamp) -> Self {
    Self(timestamp)
  }
}

/// A span of time.
#[wasm_bindgen(js_name = Duration, inspectable)]
pub struct WasmDuration(pub(crate) Duration);

#[wasm_bindgen(js_class = Duration)]
impl WasmDuration {
  /// Create a new `Duration` with the given number of seconds.
  #[wasm_bindgen]
  pub fn seconds(seconds: u32) -> WasmDuration {
    Self(Duration::seconds(seconds))
  }
  /// Create a new `Duration` with the given number of minutes.
  #[wasm_bindgen]
  pub fn minutes(minutes: u32) -> WasmDuration {
    Self(Duration::minutes(minutes))
  }

  /// Create a new `Duration` with the given number of hours.
  #[wasm_bindgen]
  pub fn hours(hours: u32) -> WasmDuration {
    Self(Duration::hours(hours))
  }

  /// Create a new `Duration` with the given number of days.
  #[wasm_bindgen]
  pub fn days(days: u32) -> WasmDuration {
    Self(Duration::days(days))
  }

  /// Create a new `Duration` with the given number of weeks.
  #[wasm_bindgen]
  pub fn weeks(weeks: u32) -> WasmDuration {
    Self(Duration::weeks(weeks))
  }
}

impl From<Duration> for WasmDuration {
  fn from(duration: Duration) -> Self {
    Self(duration)
  }
}

impl From<WasmDuration> for Duration {
  fn from(duration: WasmDuration) -> Self {
    duration.0
  }
}
