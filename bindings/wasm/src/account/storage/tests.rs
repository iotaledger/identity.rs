// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#![allow(non_snake_case)]

use identity::account_storage::tests;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::future_to_promise;

use crate::common::PromiseVoid;

use super::WasmStorage;

macro_rules! expose_to_wasm {
  ($test_name:ident, $js_name:ident) => {
    #[wasm_bindgen(js_name = $js_name)]
    pub fn $test_name(storage: WasmStorage) -> PromiseVoid {
      let promise = future_to_promise(async move {
        tests::$test_name(Box::new(storage))
          .await
          .map_err(|err| JsValue::from_str(&err.to_string()))
          .map(|_| JsValue::undefined())
      });

      promise.unchecked_into::<PromiseVoid>()
    }
  };
}

expose_to_wasm!(storage_did_create_test, storageDidCreateTest);
expose_to_wasm!(storage_key_generate_test, storageKeyGenerateTest);
expose_to_wasm!(storage_key_delete_test, storageKeyDeleteTest);
expose_to_wasm!(storage_key_insert_test, storageKeyInsertTest);
expose_to_wasm!(storage_did_list_test, storageDidListTest);
expose_to_wasm!(storage_key_sign_ed25519_test, storageKeySignEd25519Test);
