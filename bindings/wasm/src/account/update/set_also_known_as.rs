// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::account::wasm_account::WasmAccount;
use crate::common::PromiseVoid;
use crate::error::Result;
use crate::error::WasmResult;
use identity::account::Account;
use identity::core::{OneOrMany, OrderedSet, Url};

use js_sys::Promise;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::future_to_promise;

#[wasm_bindgen(js_class = Account)]
impl WasmAccount {
  /// Sets the controllers of the DID document.
  #[wasm_bindgen(js_name = setAlsoKnownAs)]
  pub fn set_also_known_as(&mut self, options: &SetAlsoKnownAsOptions) -> Result<PromiseVoid> {
    let urls: Vec<String> = options
      .urls()
      .into_serde::<OneOrMany<String>>()
      .map(OneOrMany::into_vec)
      .wasm_result()?;

    let mut urls_set: OrderedSet<Url> = OrderedSet::new();
    for url in urls {
      urls_set.append(Url::parse(url).wasm_result()?);
    }

    let account: Rc<RefCell<Account>> = Rc::clone(&self.0);
    let promise: Promise = future_to_promise(async move {
      account
        .borrow_mut()
        .update_identity()
        .set_also_known_as()
        .urls(urls_set)
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
  #[wasm_bindgen(typescript_type = "SetAlsoKnownAsOptions")]
  pub type SetAlsoKnownAsOptions;

  #[wasm_bindgen(getter, method)]
  pub fn urls(this: &SetAlsoKnownAsOptions) -> JsValue;
}

#[wasm_bindgen(typescript_custom_section)]
const TS_SET_CONTROLLER_OPTION: &'static str = r#"
/**
 * Options for setting the `alsoKnownAs` property.
 */
 export type SetAlsoKnownAsOptions = {

    /**
     * The URLs to add.
     */
    urls: string | string[],
};
"#;