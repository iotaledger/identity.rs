// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(typescript_type = "IJwkParams")]
  pub type IJwkParams;
  #[wasm_bindgen(typescript_type = "JwsAlgorithm")]
  pub type WasmJwsAlgorithm;
  #[wasm_bindgen(typescript_type = "JwkUse")]
  pub type WasmJwkUse;
  #[wasm_bindgen(typescript_type = "JwkType")]
  pub type WasmJwkType;
  #[wasm_bindgen(typescript_type = "Array<JwkOperation>")]
  pub type ArrayJwkOperation;
  #[wasm_bindgen(typescript_type = "JwkParamsEc")]
  pub type WasmJwkParamsEc;
  #[wasm_bindgen(typescript_type = "JwkParamsOkp")]
  pub type WasmJwkParamsOkp;
  #[wasm_bindgen(typescript_type = "JwkParamsRsa")]
  pub type WasmJwkParamsRsa;
  #[wasm_bindgen(typescript_type = "JwkParamsOct")]
  pub type WasmJwkParamsOct;
}
