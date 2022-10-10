// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_iota::account::UpdateError::MissingRequiredField;
use js_sys::Promise;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::future_to_promise;

use crate::account::wasm_account::WasmAccount;
use crate::common::PromiseVoid;
use crate::error::Result;
use crate::error::WasmResult;

#[wasm_bindgen(js_class = Account)]
impl WasmAccount {
  /// Deletes a verification method if the method exists.
  #[wasm_bindgen(js_name = deleteMethod)]
  pub fn delete_method(&mut self, options: &DeleteMethodOptions) -> Result<PromiseVoid> {
    let fragment: String = options
      .fragment()
      .ok_or(MissingRequiredField("fragment"))
      .wasm_result()?;
    let account = self.0.clone();
    let promise: Promise = future_to_promise(async move {
      account
        .as_ref()
        .write()
        .await
        .update_identity()
        .delete_method()
        .fragment(fragment)
        .apply()
        .await
        .wasm_result()
        .map(|_| JsValue::undefined())
    });

    Ok(promise.unchecked_into::<PromiseVoid>())
  }
}

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(typescript_type = "DeleteMethodOptions")]
  pub type DeleteMethodOptions;

  #[wasm_bindgen(getter, method)]
  pub fn fragment(this: &DeleteMethodOptions) -> Option<String>;
}

#[wasm_bindgen(typescript_custom_section)]
const TS_DELETE_METHOD_OPTIONS: &'static str = r#"
/**
 * Options for deleting a method on an identity.
 */
export type DeleteMethodOptions = {
    /**
     * The identifier of the method in the document.
     */
    fragment: string,
};
"#;
