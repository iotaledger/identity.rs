// Copyright 2020-2025 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_iota::iota::DidResolutionHandler;
use identity_iota::iota::IotaDID;
use identity_iota::iota::IotaDocument;
use js_sys::Promise;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;

use crate::error::JsValueResult;
use crate::iota::PromiseIotaDocument;
use crate::iota::WasmIotaDID;

#[wasm_bindgen(typescript_custom_section)]
const WASM_DID_DID_RESOLUTION_HANDLER: &str = r#"
interface WasmDidDidResolutionHandler {
  resolveDid: (did: IotaDID) => Promise<IotaDocument>;
}
"#;

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(typescript_type = "WasmDidDidResolutionHandler")]
  pub type WasmDidDidResolutionHandler;

  #[wasm_bindgen(js_name = "resolveDid", method)]
  pub fn resolve_did(this: &WasmDidDidResolutionHandler, did: WasmIotaDID) -> PromiseIotaDocument;
}

#[async_trait::async_trait(?Send)]
impl DidResolutionHandler for WasmDidDidResolutionHandler {
  async fn resolve_did(&self, did: &IotaDID) -> Result<IotaDocument, identity_iota::iota::Error> {
    let promise: Promise = Promise::resolve(&self.resolve_did(WasmIotaDID(did.clone())));
    let result: JsValueResult = JsFuture::from(promise).await.into();
    let js_value: JsValue = result.to_iota_core_error()?;

    js_value.into_serde().map_err(|err| {
      identity_iota::iota::Error::JsError(format!(
        "failed to parse resolved DID document to `IotaDocument`: {err}"
      ))
    })
  }
}
