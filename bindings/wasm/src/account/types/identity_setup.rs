// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity::account::IdentitySetup;
use identity::crypto::PrivateKey;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(typescript_type = "IdentitySetup")]
  pub type WasmIdentitySetup;

  #[wasm_bindgen(getter, method)]
  pub fn privateKey(this: &WasmIdentitySetup) -> Option<Vec<u8>>;
}

#[wasm_bindgen(typescript_custom_section)]
const TS_IDENTITY_SETUP: &'static str = r#"
/**
 * Configuration used to create a new Identity.
 */
export type IdentitySetup = {
    /**
     * Use a pre-generated Ed25519 private key for the DID.
     */
    privateKey?: Uint8Array,
};
"#;

impl From<WasmIdentitySetup> for IdentitySetup {
  fn from(wasm_setup: WasmIdentitySetup) -> Self {
    let mut setup: IdentitySetup = IdentitySetup::new();
    if let Some(private_key) = wasm_setup.privateKey() {
      setup = setup.private_key(PrivateKey::from(private_key));
    }
    setup
  }
}
