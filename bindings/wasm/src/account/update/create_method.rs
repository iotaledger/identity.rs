// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use js_sys::Promise;
use std::cell::RefCell;
use std::cell::RefMut;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;

use identity::account::Account;
use identity::account::CreateMethodBuilder;
use identity::account::IdentityUpdater;
use identity::account::MethodSecret;
use identity::account::UpdateError::MissingRequiredField;
use identity::did::MethodScope;
use identity::did::MethodType;

use crate::account::wasm_account::WasmAccount;
use crate::account::wasm_method_secret::WasmMethodSecret;
use crate::did::WasmMethodScope;
use crate::did::WasmMethodType;

use crate::error::Result;
use crate::error::WasmResult;

#[wasm_bindgen(js_class = Account)]
impl WasmAccount {
  /// Adds a new verification method to the DID document.
  #[wasm_bindgen(js_name = createMethod)]
  pub fn create_method(&mut self, options: &CreateMethodOptions) -> Result<Promise> {
    let method_type: Option<MethodType> = options.methodType().map(|m| m.0);

    let fragment: String = options
      .fragment()
      .ok_or(MissingRequiredField("fragment"))
      .wasm_result()?;

    let method_scope: Option<MethodScope> = options.methodScope().map(|ms| ms.0);

    let method_secret: Option<MethodSecret> = options.methodSecret().map(|ms| ms.0);

    let account: Rc<RefCell<Account>> = Rc::clone(&self.0);
    let promise: Promise = future_to_promise(async move {
      let mut account: RefMut<Account> = account.borrow_mut();
      let mut updater: IdentityUpdater<'_> = account.update_identity();
      let mut create_method: CreateMethodBuilder<'_> = updater.create_method().fragment(fragment);

      if let Some(type_) = method_type {
        create_method = create_method.type_(type_);
      };

      if let Some(scope) = method_scope {
        create_method = create_method.scope(scope);
      };

      if let Some(method_secret) = method_secret {
        create_method = create_method.method_secret(method_secret);
      };

      create_method.apply().await.wasm_result().map(|_| JsValue::undefined())
    });

    Ok(promise)
  }
}

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(typescript_type = "CreateMethodOptions")]
  pub type CreateMethodOptions;

  #[wasm_bindgen(getter, method)]
  pub fn fragment(this: &CreateMethodOptions) -> Option<String>;

  #[wasm_bindgen(getter, method)]
  pub fn methodScope(this: &CreateMethodOptions) -> Option<WasmMethodScope>;

  #[wasm_bindgen(getter, method)]
  pub fn methodType(this: &CreateMethodOptions) -> Option<WasmMethodType>;

  #[wasm_bindgen(getter, method)]
  pub fn methodSecret(this: &CreateMethodOptions) -> Option<WasmMethodSecret>;
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
    methodScope?: MethodScope,

    /**
     * The type of the method, defaults to Ed25519VerificationKey2018.
     */
    methodType?: MethodType,

    /**
     * The private key to use for the method, optional. A new private key will be generated if omitted.
     */
    methodSecret?: MethodSecret
  };
"#;
