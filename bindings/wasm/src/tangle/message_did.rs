// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity::iota::MessageId;
use wasm_bindgen::prelude::*;

use crate::error::Result;
use crate::error::WasmResult;

#[wasm_bindgen(js_name = MessageId, inspectable)]
pub struct WasmMessageId(pub(crate) MessageId);

#[wasm_bindgen(js_class = MessageId)]
impl WasmMessageId {
  // Serializes a `ChainState` object as a JSON object.
  #[wasm_bindgen(js_name = toJSON)]
  pub fn to_json(&self) -> Result<JsValue> {
    JsValue::from_serde(&self.0).wasm_result()
  }

  /// Deserializes a JSON object as `MessageId`.
  #[wasm_bindgen(js_name = fromJSON)]
  pub fn from_json(json_value: JsValue) -> Result<WasmMessageId> {
    json_value.into_serde().map(Self).wasm_result()
  }
}

impl From<WasmMessageId> for MessageId {
  fn from(wasm_message_id: WasmMessageId) -> Self {
    wasm_message_id.0
  }
}

impl From<MessageId> for WasmMessageId {
  fn from(message_id: MessageId) -> Self {
    WasmMessageId(message_id)
  }
}
