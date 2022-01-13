// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::account::account::WasmAccount;
use crate::error::{wasm_error, Result, WasmResult};
use identity::account::Update;
use identity::account::UpdateError::MissingRequiredField;
use identity::core::Url;
use identity::did::ServiceEndpoint;
use js_sys::Promise;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;

#[wasm_bindgen(js_class = Account)]
impl WasmAccount {
  #[wasm_bindgen(js_name = deleteService)]
  pub fn delete_service(&mut self, fragment: String) -> Result<Promise> {
    let account = self.0.clone();

    let update = Update::DeleteService {
      fragment,
    };

    let promise: Promise = future_to_promise(async move {
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
