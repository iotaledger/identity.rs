// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::cell::RefCell;
use std::rc::Rc;

use identity_iota::core::OneOrMany;
use identity_iota::core::OneOrSet;
use identity_iota::core::OrderedSet;
use identity_iota::iota_core::IotaDID;
use js_sys::Promise;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::future_to_promise;

use crate::account::wasm_account::account::AccountRc;
use crate::account::wasm_account::WasmAccount;
use crate::common::PromiseVoid;
use crate::error::Result;
use crate::error::WasmResult;

#[wasm_bindgen(js_class = Account)]
impl WasmAccount {
  /// Sets the controllers of the DID document.
  #[wasm_bindgen(js_name = setController)]
  pub fn set_controller(&mut self, options: &SetControllerOptions) -> Result<PromiseVoid> {
    let controllers: Option<OneOrMany<IotaDID>> = options.controllers().into_serde().wasm_result()?;

    let controller_set: Option<OneOrSet<IotaDID>> = if let Some(controllers) = controllers {
      match controllers {
        OneOrMany::One(controller) => Some(OneOrSet::new_one(controller)),
        OneOrMany::Many(controllers) => {
          if controllers.is_empty() {
            None
          } else {
            let mut set: OrderedSet<IotaDID> = OrderedSet::new();
            for controller in controllers {
              set.append(controller);
            }
            Some(OneOrSet::new_set(set).wasm_result()?)
          }
        }
      }
    } else {
      None
    };

    let account: Rc<RefCell<AccountRc>> = Rc::clone(&self.0);

    let promise: Promise = future_to_promise(async move {
      account
        .borrow_mut()
        .update_identity()
        .set_controller()
        .controllers(controller_set)
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
const TS_SET_CONTROLLER_OPTIONS: &'static str = r#"
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
