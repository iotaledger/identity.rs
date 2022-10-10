// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::rc::Rc;

use identity_iota::core::OneOrMany;
use identity_iota::core::OrderedSet;
use identity_iota::core::Url;
use js_sys::Promise;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::future_to_promise;

use crate::account::wasm_account::account::TokioLock;
use crate::account::wasm_account::WasmAccount;
use crate::common::PromiseVoid;
use crate::error::Result;
use crate::error::WasmResult;

#[wasm_bindgen(js_class = Account)]
impl WasmAccount {
  /// Sets the `alsoKnownAs` property in the DID document.
  #[wasm_bindgen(js_name = setAlsoKnownAs)]
  pub fn set_also_known_as(&mut self, options: &SetAlsoKnownAsOptions) -> Result<PromiseVoid> {
    let urls: Option<OneOrMany<String>> = options.urls().into_serde::<Option<OneOrMany<String>>>().wasm_result()?;

    let mut urls_set: OrderedSet<Url> = OrderedSet::new();
    if let Some(urls) = urls {
      for url in urls.into_vec() {
        urls_set.append(Url::parse(url).wasm_result()?);
      }
    }

    let account: Rc<TokioLock> = Rc::clone(&self.0);
    let promise: Promise = future_to_promise(async move {
      account
        .write()
        .await
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
const TS_SET_ALSO_KNOWN_AS_OPTIONS: &'static str = r#"
/**
 * Options for setting the `alsoKnownAs` property.
 */
 export type SetAlsoKnownAsOptions = {

    /**
     * List of URLs for the `alsoKnownAs` property. Duplicates are ignored.
     */
    urls: string | string[] | null,
};
"#;
