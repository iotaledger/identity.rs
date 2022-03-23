// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::convert::TryFrom;
use std::result::Result as StdResult;

use identity::account::IdentitySetup;
use identity::account::MethodSecret;
use wasm_bindgen::prelude::*;

use crate::account::types::method_secret::OptionMethodSecret;
use crate::account::types::WasmMethodSecret;
use crate::crypto::WasmKeyType;
use crate::error::WasmResult;

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(typescript_type = "IdentitySetup")]
  pub type WasmIdentitySetup;

  #[wasm_bindgen(getter, method)]
  pub fn keyType(this: &WasmIdentitySetup) -> Option<WasmKeyType>;

  #[wasm_bindgen(getter, method)]
  pub fn methodSecret(this: &WasmIdentitySetup) -> OptionMethodSecret;
}

#[wasm_bindgen(typescript_custom_section)]
const TS_IDENTITY_SETUP: &'static str = r#"
/**
 * Configuration used to create a new Identity.
 * Overrides the default creation of private and public keys.
 */
export type IdentitySetup = {
    /**
     * Key type of the initial verification method.
     */
    keyType?: KeyType,

    /**
     * {@link MethodSecret} used for the identity creation.
     */
    methodSecret?: MethodSecret
};
"#;

impl TryFrom<WasmIdentitySetup> for IdentitySetup {
  type Error = JsValue;

  fn try_from(wasm_identity_setup: WasmIdentitySetup) -> StdResult<Self, Self::Error> {
    let mut setup = IdentitySetup::new();
    if let Some(key_type) = wasm_identity_setup.keyType() {
      setup = setup.key_type(key_type.into());
    }
    if let Some(method_secret) = wasm_identity_setup
      .methodSecret()
      .into_serde::<Option<WasmMethodSecret>>()
      .wasm_result()?
      .map(MethodSecret::try_from)
      .transpose()?
    {
      setup = setup.method_secret(method_secret);
    };
    Ok(setup)
  }
}
