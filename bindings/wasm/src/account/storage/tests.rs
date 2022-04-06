// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#![allow(non_snake_case)]

use identity::account_storage::tests;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::future_to_promise;

use crate::common::PromiseVoid;

use super::WasmStorage;

#[wasm_bindgen]
pub struct StorageTestSuite;

macro_rules! expose_to_wasm {
  ($test_name:ident, $js_name:ident) => {
    #[wasm_bindgen]
    impl StorageTestSuite {
      #[wasm_bindgen(js_name = $js_name)]
      pub fn $test_name(storage: WasmStorage) -> PromiseVoid {
        let promise = future_to_promise(async move {
          tests::$test_name(Box::new(storage))
            .await
            // TODO: Use custom error cause string to prevent newlines from being added.
            .map_err(|err| JsValue::from_str(&format!("{:?}", err)))
            .map(|_| JsValue::undefined())
        });

        promise.unchecked_into::<PromiseVoid>()
      }
    }
  };
}

expose_to_wasm!(storage_did_create_test, didCreateTest);
expose_to_wasm!(storage_did_list_test, didListTest);
expose_to_wasm!(storage_did_purge_test, didPurgeTest);
expose_to_wasm!(storage_key_generate_test, keyGenerateTest);
expose_to_wasm!(storage_key_delete_test, keyDeleteTest);
expose_to_wasm!(storage_key_insert_test, keyInsertTest);
expose_to_wasm!(storage_key_sign_ed25519_test, keySignEd25519Test);
