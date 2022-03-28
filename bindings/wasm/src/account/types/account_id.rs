// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity::account_storage::AccountId;
use wasm_bindgen::prelude::*;

use crate::error::Result;
use crate::error::WasmResult;

/// A unique storage identifier for an account.
#[wasm_bindgen(js_name = AccountId, inspectable)]
pub struct WasmAccountId(pub(crate) AccountId);

#[wasm_bindgen(js_class = AccountId)]
impl WasmAccountId {
  /// Generates a new random identifier.
  #[wasm_bindgen(constructor)]
  pub fn generate(&self) -> Self {
    Self(AccountId::generate())
  }

  /// Returns a string representation of the identifier.
  #[wasm_bindgen(js_name = toString)]
  pub fn to_string(&self) -> String {
    self.0.to_string()
  }

  /// Returns the bytes that make up the identifier.
  #[wasm_bindgen(js_name = toBytes)]
  pub fn to_bytes(&self) -> Vec<u8> {
    self.0.as_bytes().to_vec()
  }

  /// Constructs an `AccountId` from 16 bytes.
  #[wasm_bindgen(js_name = fromBytes)]
  pub fn from_bytes(bytes: Vec<u8>) -> Result<WasmAccountId> {
    let bytes: [u8; 16] = bytes
      .try_into()
      .map_err(|_| JsValue::from_str("expected 16 bytes"))?;

    Ok(Self(AccountId::from(bytes)))
  }

  /// Serializes an `AccountId` as a JSON object.
  #[wasm_bindgen(js_name = toJSON)]
  pub fn to_json(&self) -> Result<JsValue> {
    JsValue::from_serde(&self.0).wasm_result()
  }

  /// Deserializes an `AccountId` from a JSON object.
  #[wasm_bindgen(js_name = fromJSON)]
  pub fn from_json(json_value: JsValue) -> Result<WasmAccountId> {
    json_value.into_serde().map(Self).wasm_result()
  }
}

impl_wasm_clone!(WasmAccountId, AccountId);

impl From<AccountId> for WasmAccountId {
  fn from(account_id: AccountId) -> Self {
    Self(account_id)
  }
}
