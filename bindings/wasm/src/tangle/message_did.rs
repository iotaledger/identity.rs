// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use wasm_bindgen::prelude::*;

use identity::iota::MessageId as MessageId_;

use crate::error::Result;
use crate::error::WasmResult;

#[wasm_bindgen(js_name = MessageId, inspectable)]
#[derive(Clone, Copy, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub struct WasmMessageId(pub(crate) MessageId_);

#[wasm_bindgen(js_class = MessageId)]
impl WasmMessageId {
  #[wasm_bindgen(constructor)]
  pub fn new(bytes: &str) -> Result<WasmMessageId> {
    MessageId_::from_str(bytes).map(|x| x.into()).wasm_result()
  }

  /// Create a null `MessageId`.
  #[wasm_bindgen]
  pub fn null() -> Self {
    WasmMessageId(MessageId_::null())
  }
}

impl From<WasmMessageId> for MessageId_ {
  fn from(wasm_message_id: WasmMessageId) -> Self {
    wasm_message_id.0
  }
}

impl From<MessageId_> for WasmMessageId {
  fn from(message_id: MessageId_) -> Self {
    WasmMessageId(message_id)
  }
}
