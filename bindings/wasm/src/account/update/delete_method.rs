// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::account::account::WasmAccount;

use crate::error::{wasm_error, Result, WasmResult};

use identity::account::UpdateError::MissingRequiredField;
use identity::account::{Update};




use js_sys::Promise;






use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;

#[wasm_bindgen(js_class = Account)]
impl WasmAccount {
  #[wasm_bindgen(js_name = deleteMethod)]
  pub fn delete_method(&mut self, input: &deleteMethodInput) -> Result<Promise> {
    let account = self.0.clone();

    let fragment = match input.fragment() {
      Some(value) => value.clone(),
      None => return Err(wasm_error(MissingRequiredField("fragment"))),
    };

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

#[wasm_bindgen]
extern "C" {
  pub type deleteMethodInput;

  #[wasm_bindgen(structural, getter, method)]
  pub fn fragment(this: &deleteMethodInput) -> Option<String>;
}

#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"
export type deleteMethodInput = {
  fragment?: string,
};
"#;
