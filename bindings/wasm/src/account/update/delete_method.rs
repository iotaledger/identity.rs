// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use js_sys::Promise;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;

use identity::account::Update;
use identity::account::UpdateError::MissingRequiredField;

use crate::account::wasm_account::WasmAccount;
use crate::error::{Result, wasm_error};
use crate::error::WasmResult;

#[wasm_bindgen(js_class = Account)]
impl WasmAccount {
  /// Deletes a verification method if the method exists.
  #[wasm_bindgen(js_name = deleteMethod)]
  pub fn delete_method(&mut self, options: DeleteMethodOptions) -> Result<Promise> {
    let account = self.0.clone();
    let fragment: String = options.fragment().ok_or(wasm_error(MissingRequiredField("fragment")))?;
    let promise: Promise = future_to_promise(async move {
      let update = Update::DeleteMethod { fragment };
      account
        .as_ref()
        .borrow_mut()
        .process_update(update)
        .await
        .wasm_result()
        .map(|_| JsValue::undefined())
    });

    Ok(promise)
  }
}

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(typescript_type = "DeleteMethodOptions")]
  pub type DeleteMethodOptions;

  #[wasm_bindgen(structural, getter, method)]
  pub fn fragment(this: &DeleteMethodOptions) -> Option<String>;
}

#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"
export type DeleteMethodOptions = {
  fragment: string,
};
"#;
