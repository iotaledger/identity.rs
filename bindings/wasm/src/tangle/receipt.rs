// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity::iota::Receipt;
use wasm_bindgen::prelude::*;

use crate::error::Result;
use crate::error::WasmResult;
use crate::tangle::WasmNetwork;

#[wasm_bindgen(js_name = Receipt, inspectable)]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct WasmReceipt(pub(crate) Receipt);

// Workaround for Typescript type annotations on async function returns.
#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(typescript_type = "Promise<Receipt>")]
  pub type PromiseReceipt;
}

#[wasm_bindgen(js_class = Receipt)]
impl WasmReceipt {
  /// Returns a copy of the associated IOTA Tangle `Network`.
  #[wasm_bindgen]
  pub fn network(&self) -> WasmNetwork {
    WasmNetwork::from(self.0.network())
  }

  /// Returns a copy of the message `id`.
  #[wasm_bindgen(js_name = messageId)]
  pub fn message_id(&self) -> String {
    self.0.message_id().to_string()
  }

  /// Returns a copy of the message `network_id`.
  #[wasm_bindgen(js_name = networkId)]
  pub fn network_id(&self) -> String {
    // NOTE: do not return u64 to avoid BigInt64Array/BigUint64Array compatibility issues.
    self.0.network_id().to_string()
  }

  /// Returns a copy of the message `nonce`.
  #[wasm_bindgen]
  pub fn nonce(&self) -> String {
    // NOTE: do not return u64 to avoid BigInt64Array/BigUint64Array compatibility issues.
    self.0.nonce().to_string()
  }

  /// Serializes a `Receipt` as a JSON object.
  #[wasm_bindgen(js_name = toJSON)]
  pub fn to_json(&self) -> Result<JsValue> {
    JsValue::from_serde(&self.0).wasm_result()
  }

  /// Deserializes a `Receipt` from a JSON object.
  #[wasm_bindgen(js_name = fromJSON)]
  pub fn from_json(json: &JsValue) -> Result<WasmReceipt> {
    json.into_serde().map(Self).wasm_result()
  }
}

impl_wasm_clone!(WasmReceipt, Receipt);

impl From<Receipt> for WasmReceipt {
  fn from(receipt: Receipt) -> Self {
    Self(receipt)
  }
}

impl From<WasmReceipt> for Receipt {
  fn from(receipt: WasmReceipt) -> Self {
    receipt.0
  }
}
