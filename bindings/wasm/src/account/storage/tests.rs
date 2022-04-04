// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity::account_storage::tests;
use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_futures::future_to_promise;

use crate::common::PromiseVoid;

use super::WasmStorage;

#[wasm_bindgen(js_name = storageDidCreateTest)]
pub fn storage_did_create_test(storage: WasmStorage) -> PromiseVoid {
  let promise = future_to_promise(async move {
    tests::storage_did_create_test(Box::new(storage))
      .await
      .map_err(|err| JsValue::from_str(&err.to_string()))
      .map(|_| JsValue::undefined())
  });

  promise.unchecked_into::<PromiseVoid>()
}
