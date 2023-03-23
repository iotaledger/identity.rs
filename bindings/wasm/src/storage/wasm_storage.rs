// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::rc::Rc;

use identity_iota::storage::storage::Storage;

use super::WasmJwkStorage;
use super::WasmKeyIdStorage;
use wasm_bindgen::prelude::*;

pub(crate) type WasmStorageInner = Storage<WasmJwkStorage, WasmKeyIdStorage>;
#[wasm_bindgen(js_name = Storage)]
pub struct WasmStorage(pub(crate) Rc<WasmStorageInner>);

#[wasm_bindgen(js_class = Storage)]
impl WasmStorage {
  #[wasm_bindgen(constructor)]
  #[allow(non_snake_case)]
  pub fn new(jwkStorage: WasmJwkStorage, keyIdStorage: WasmKeyIdStorage) -> WasmStorage {
    WasmStorage(Rc::new(Storage::new(jwkStorage, keyIdStorage)))
  }

  #[wasm_bindgen(js_name = keyIdStorage)]
  pub fn key_id_storage(&self) -> WasmKeyIdStorage {
    JsValue::from(self.0.key_id_storage()).unchecked_into()
  }

  #[wasm_bindgen(js_name = keyStorage)]
  pub fn key_storage(&self) -> WasmJwkStorage {
    JsValue::from(self.0.key_storage()).unchecked_into()
  }
}
