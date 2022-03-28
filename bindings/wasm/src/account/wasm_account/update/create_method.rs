// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::cell::RefCell;
use std::cell::RefMut;
use std::rc::Rc;

use identity::account::CreateMethodBuilder;
use identity::account::IdentityUpdater;
use identity::account::MethodContent;
use identity::account::UpdateError::MissingRequiredField;
use identity::did::MethodScope;
use identity::iota::Client;
use js_sys::Promise;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::future_to_promise;

use crate::account::types::OptionMethodContent;
use crate::account::types::WasmMethodContent;
use crate::account::wasm_account::account::AccountRc;
use crate::account::wasm_account::WasmAccount;
use crate::common::PromiseVoid;
use crate::did::OptionMethodScope;
use crate::error::Result;
use crate::error::WasmResult;

#[wasm_bindgen(js_class = Account)]
impl WasmAccount {
  /// Adds a new verification method to the DID document.
  #[wasm_bindgen(js_name = createMethod)]
  pub fn create_method(&mut self, options: &CreateMethodOptions) -> Result<PromiseVoid> {
    let fragment: String = options
      .fragment()
      .ok_or(MissingRequiredField("fragment"))
      .wasm_result()?;
    let scope: Option<MethodScope> = options.scope().into_serde().wasm_result()?;
    let content: MethodContent = options
      .content()
      .into_serde::<Option<WasmMethodContent>>()
      .wasm_result()?
      .map(MethodContent::from)
      .ok_or(MissingRequiredField("content"))
      .wasm_result()?;

    let account: Rc<RefCell<AccountRc>> = Rc::clone(&self.0);
    let promise: Promise = future_to_promise(async move {
      let mut account: RefMut<AccountRc> = account.borrow_mut();
      let mut updater: IdentityUpdater<'_, Rc<Client>> = account.update_identity();

      let mut create_method: CreateMethodBuilder<'_, Rc<Client>> =
        updater.create_method().content(content).fragment(fragment);
      if let Some(scope) = scope {
        create_method = create_method.scope(scope);
      };

      create_method.apply().await.wasm_result().map(|_| JsValue::undefined())
    });

    Ok(promise.unchecked_into::<PromiseVoid>())
  }
}

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(typescript_type = "CreateMethodOptions")]
  pub type CreateMethodOptions;

  #[wasm_bindgen(getter, method)]
  pub fn fragment(this: &CreateMethodOptions) -> Option<String>;

  #[wasm_bindgen(getter, method)]
  pub fn scope(this: &CreateMethodOptions) -> OptionMethodScope;

  #[wasm_bindgen(getter, method)]
  pub fn content(this: &CreateMethodOptions) -> OptionMethodContent;
}

#[wasm_bindgen(typescript_custom_section)]
const TS_CREATE_METHOD_OPTIONS: &'static str = r#"
/**
 * Options for creating a new method on an identity.
 */
export type CreateMethodOptions = {
    /**
     * The identifier of the method in the document.
     */
    fragment: string,

    /**
     * The scope of the method, defaults to VerificationMethod.
     */
    scope?: MethodScope,

    /**
     * Method content for the new method.
     */
    content: MethodContent
  };
"#;
