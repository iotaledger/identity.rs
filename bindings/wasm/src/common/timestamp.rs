// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity::core::Timestamp;
use wasm_bindgen::prelude::*;

use crate::error::Result;
use crate::error::WasmResult;

#[wasm_bindgen(js_name = Timestamp, inspectable)]
#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
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
}

impl From<Timestamp> for WasmTimestamp {
  fn from(timestamp: Timestamp) -> Self {
    Self(timestamp)
  }
}
