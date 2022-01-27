// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::crypto::KeyType;
use wasm_bindgen::prelude::*;
use crate::account::method_secret::WasmMethodSecret;

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(typescript_type = "IdentitySetup")]
  pub type WasmIdentitySetup;

  #[wasm_bindgen(structural, getter, method)]
  pub fn keyType(this: &WasmIdentitySetup) -> Option<KeyType>;

  #[wasm_bindgen(structural, getter, method)]
  pub fn methodSecret(this: &WasmIdentitySetup) -> Option<WasmMethodSecret>;
}

#[wasm_bindgen(typescript_custom_section)]
const TS_IDENTITY_SETUP: &'static str = r#"
/**
 * Overrides the default creation of private and public keys.
 */
export type IdentitySetup = {
    keyType?: KeyType,
    methodSecret?: MethodSecret
};
"#;
