// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use wasm_bindgen::prelude::*;

use crate::account::types::WasmMethodSecret;
use crate::crypto::KeyType;

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
 * Configuration used to create a new Identity.
 * Overrides the default creation of private and public keys.
 */
export type IdentitySetup = {
    keyType?: KeyType,
    methodSecret?: MethodSecret
};
"#;
