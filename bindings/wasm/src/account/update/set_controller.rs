// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::account::wasm_account::WasmAccount;
use crate::common::PromiseVoid;
use crate::error::Result;
use crate::error::WasmResult;
use identity::account::Account;
use identity::core::OneOrSet;
use identity::iota::IotaDID;
use js_sys::Promise;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::future_to_promise;

#[wasm_bindgen(js_class = Account)]
impl WasmAccount {
  /// Sets the controllers of the DID document.
  #[wasm_bindgen(js_name = setController)]
  pub fn set_controller(&mut self, controllers: &SetControllerOptions) -> Result<PromiseVoid> {
    let controllers: Option<OneOrSet<IotaDID>> = controllers.controllers().into_serde().wasm_result()?;
    let account: Rc<RefCell<Account>> = Rc::clone(&self.0);

    let promise: Promise = future_to_promise(async move {
      account
        .borrow_mut()
        .update_identity()
        .set_controller()
        .controllers(controllers)
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
  #[wasm_bindgen(typescript_type = "SetControllerOptions")]
  pub type SetControllerOptions;

  #[wasm_bindgen(getter, method)]
  pub fn controllers(this: &SetControllerOptions) -> JsValue;
}

#[wasm_bindgen(typescript_custom_section)]
const TS_SET_CONTROLLER_OPTION: &'static str = r#"
/**
 * Options for setting DID controllers.
 */
 export type SetControllerOptions = {

    /**
     * List of DIDs to be set as controllers, use `null` to remove all controllers.
     */
    controllers: DID | DID[] | null,
};
"#;
