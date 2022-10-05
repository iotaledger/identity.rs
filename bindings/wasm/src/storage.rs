// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_storage::IdentitySuite;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::key_storage::WasmKeyStorage;

#[wasm_bindgen(js_name = IdentitySuite)]
pub struct WasmIdentitySuite(IdentitySuite<WasmKeyStorage>);

#[wasm_bindgen(js_class = IdentitySuite)]
impl WasmIdentitySuite {
  #[wasm_bindgen(constructor)]
  #[allow(non_snake_case)]
  pub fn new(storage: WasmKeyStorage) -> Self {
    WasmIdentitySuite(IdentitySuite::new(storage))
  }
}

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(typescript_type = "SignatureHandler")]
  pub(crate) type MapSignatureHandler;

  #[wasm_bindgen(typescript_type = "IdentitySuiteConfig")]
  pub type IdentitySuiteConfig;

  #[wasm_bindgen(method, getter)]
  pub(crate) fn handlers(this: &IdentitySuiteConfig) -> Option<MapSignatureHandler>;

}

// TODO: Change to match Rust's SignatureHandler trait.
// Workaround because JSDocs does not support arrows (=>) while TS does not support the "function" word in type
// definitions (which would be accepted by JSDocs).
#[wasm_bindgen(typescript_custom_section)]
const HANDLERS: &'static str =
  "export type SignatureHandler = Map<string, (did: string) => Promise<IotaDocument | CoreDocument>>;";

#[wasm_bindgen(typescript_custom_section)]
const TS_RESOLVER_CONFIG: &'static str = r#"
export type IdentitySuiteConfig = {
    handlers?: Map<string, (did: string) => Promise<IotaDocument | CoreDocument>>
};
"#;
