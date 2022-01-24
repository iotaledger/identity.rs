// Copyright 2020-2022 IOTA Stiftun
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use wasm_bindgen::prelude::*;

use identity::iota::MessageId;

use crate::error::Result;
use crate::error::WasmResult;

#[wasm_bindgen(js_name = MessageId, inspectable)]
pub struct WasmMessageId(pub(crate) MessageId);

#[wasm_bindgen(js_class = MessageId)]
impl WasmMessageId {
  #[wasm_bindgen(constructor)]
  pub fn new(bytes: &str) -> Result<WasmMessageId> {
    MessageId::from_str(bytes).map(|x| x.into()).wasm_result()
  }

  /// Create a null `MessageId`.
  #[wasm_bindgen]
  pub fn null() -> Self {
    WasmMessageId(MessageId::null())
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
