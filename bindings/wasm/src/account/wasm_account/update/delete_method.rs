// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use js_sys::Promise;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;

use identity::account::Update;

use crate::account::wasm_account::WasmAccount;
use crate::error::Result;
use crate::error::WasmResult;

#[wasm_bindgen(js_class = Account)]
impl WasmAccount {
  /// Deletes a verification method if the method exists.
  #[wasm_bindgen(js_name = deleteMethod)]
  pub fn delete_method(&mut self, fragment: String) -> Result<Promise> {
    let account = self.0.clone();

    let promise: Promise = future_to_promise(async move {
      let update = Update::DeleteMethod { fragment };

      account
        .as_ref()
        .borrow_mut()
        .process_update(update)
        .await
        .wasm_result()
        .and_then(|output| JsValue::from_serde(&output).wasm_result())
    });

    Ok(promise)
  }
}
