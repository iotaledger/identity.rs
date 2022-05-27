// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity::iota_core::EmbeddedRevocationService;
use wasm_bindgen::prelude::*;

use crate::did::WasmDIDUrl;
use crate::error::Result;
use crate::error::WasmResult;

/// A DID Document Service used to enable validators to check the status of a credential.
#[wasm_bindgen(js_name = EmbeddedRevocationService, inspectable)]
pub struct WasmEmbeddedRevocationService(pub(crate) EmbeddedRevocationService);

#[wasm_bindgen(js_class = EmbeddedRevocationService)]
impl WasmEmbeddedRevocationService {
  #[wasm_bindgen(constructor)]
  pub fn new() -> WasmEmbeddedRevocationService {
    unimplemented!();
  }

  /// Returns a copy of the `EmbeddedRevocationService` id.
  #[wasm_bindgen]
  pub fn id(&self) -> WasmDIDUrl {
    WasmDIDUrl::from(self.0.id().clone())
  }

  /// Returns a copy of the `Service` type.
  #[wasm_bindgen(js_name = type)]
  pub fn type_(&self) -> String {
    self.0.type_().to_owned()
  }

  /// Sets the `EmbeddedRevocationService` id.
  #[wasm_bindgen(js_name = setId)]
  pub fn set_id(&mut self, id: &WasmDIDUrl) -> Result<()> {
    self.0.set_id(id.clone().into()).wasm_result()
  }

  /// Serializes a `EmbeddedRevocationService` object as a JSON object.
  #[wasm_bindgen(js_name = toJSON)]
  pub fn to_json(&self) -> Result<JsValue> {
    JsValue::from_serde(&self.0).wasm_result()
  }

  /// Deserializes a `EmbeddedRevocationService` object from a JSON object.
  #[wasm_bindgen(js_name = fromJSON)]
  pub fn from_json(value: &JsValue) -> Result<WasmEmbeddedRevocationService> {
    value.into_serde().map(Self).wasm_result()
  }
}
