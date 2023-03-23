// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::error::Result;
use crate::error::WasmResult;
use crate::verification::WasmMethodScope;
use identity_iota::document::verifiable::JwsVerificationOptions;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = JwsVerificationOptions, inspectable)]
pub struct WasmJwsVerificationOptions(pub(crate) JwsVerificationOptions);

#[wasm_bindgen(js_class = JwsVerificationOptions)]
impl WasmJwsVerificationOptions {
  #[wasm_bindgen(constructor)]
  pub fn new(options: Option<IJwsVerificationOptions>) -> Result<WasmJwsVerificationOptions> {
    if let Some(options) = options {
      let options: JwsVerificationOptions = options.into_serde().wasm_result()?;
      Ok(WasmJwsVerificationOptions(options))
    } else {
      Ok(WasmJwsVerificationOptions(Default::default()))
    }
  }

  #[wasm_bindgen(js_name = setNonce)]
  pub fn set_nonce(&mut self, value: String) {
    self.0.nonce = Some(value);
  }

  #[wasm_bindgen(js_name = addCrit)]
  pub fn add_crit(&mut self, value: String) {
    self.0.crits.get_or_insert(Vec::new()).push(value);
  }

  #[wasm_bindgen(js_name = setScope)]
  pub fn set_scope(&mut self, value: &WasmMethodScope) {
    self.0.method_scope = Some(value.0);
  }
}

impl_wasm_json!(WasmJwsVerificationOptions, JwsVerificationOptions);
impl_wasm_clone!(WasmJwsVerificationOptions, JwsVerificationOptions);

/// Duck-typed interface to allow creating `JwsVerificationOptions` easily.
#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(typescript_type = "IJwsVerificationOptions")]
  pub type IJwsVerificationOptions;
}

#[wasm_bindgen(typescript_custom_section)]
const I_JWS_SIGNATURE_OPTIONS: &'static str = r#"
/** Holds options to create `JwsVerificationOptions`. */
interface IJwsVerificationOptions {
    /** 
    *
    */
    readonly crits?: [string];

    /** 
    *
    * Default: false
    */
    readonly nonce?: string;

    /**  */
    readonly methodScope?: MethodScope;
}"#;
