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
  /// # Errors
  /// Errors if the operation leads to a timestamp not in the valid range for [RFC 3339](https://tools.ietf.org/html/rfc3339).
  #[wasm_bindgen(js_name = tryAdd)]
  pub fn try_add(self, duration: WasmDuration) -> Result<WasmTimestamp> {
    self.0.try_add(duration.0).map(WasmTimestamp).wasm_result()
  }

  /// Computes `self - duration`
  ///
  /// # Errors
  /// Errors if the operation leads to a timestamp not in the valid range for [RFC 3339](https://tools.ietf.org/html/rfc3339).
  #[wasm_bindgen(js_name = trySub)]
  pub fn try_sub(self, duration: WasmDuration) -> Result<WasmTimestamp> {
    self.0.try_sub(duration.0).map(WasmTimestamp).wasm_result()
  }
}

impl From<Timestamp> for WasmTimestamp {
  fn from(timestamp: Timestamp) -> Self {
    Self(timestamp)
  }
}

#[wasm_bindgen(js_name = Duration, inspectable)]
pub struct WasmDuration(pub(crate) Duration);

#[wasm_bindgen(js_class = Duration)]
impl WasmDuration {
  /// Create a new `Duration` with the given amount of seconds.
  pub fn seconds(seconds: u32) -> WasmDuration {
    Self(Duration::seconds(seconds))
  }
  /// Create a new `Duration` with the given amount of minutes.
  pub fn minutes(minutes: u32) -> WasmDuration {
    Self(Duration::minutes(minutes))
  }

  /// Create a new `Duration` with the given amount of hours.
  pub fn hours(hours: u32) -> WasmDuration {
    Self(Duration::hours(hours))
  }

  /// Create a new `Duration` with the given amount of weeks.
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
