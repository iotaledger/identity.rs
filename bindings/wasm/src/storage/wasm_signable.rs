// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

use identity_storage::Signable;
use wasm_bindgen::prelude::*;

use crate::credential::WasmCredential;

// TODO: Use TokioLock approach over RefCell.
#[wasm_bindgen(js_name = Signable)]
pub struct WasmSignable(pub(crate) Rc<RefCell<Signable>>);

#[wasm_bindgen(js_class = Signable)]
impl WasmSignable {
  #[wasm_bindgen(js_name = Credential)]
  pub fn credential(cred: &WasmCredential) -> WasmSignable {
    Self(Rc::new(RefCell::new(Signable::Credential(cred.0.clone()))))
  }

  /// Serializes this to a JSON object.
  #[wasm_bindgen(js_name = toJSON)]
  pub fn to_json(&self) -> crate::error::Result<JsValue> {
    use crate::error::WasmResult;
    JsValue::from_serde(self.0.borrow().deref()).wasm_result()
  }
}

impl From<Signable> for WasmSignable {
  fn from(signable: Signable) -> Self {
    WasmSignable(Rc::new(RefCell::new(signable)))
  }
}
