// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::cell::RefCell;
use std::cell::RefMut;
use std::rc::Rc;

use identity::account::CreateMethodBuilder;
use identity::account::IdentityUpdater;
use identity::account::MethodSecret;
use identity::account::UpdateError::MissingRequiredField;
use identity::did::MethodScope;
use identity::did::MethodType;
use identity::iota::Client;
use js_sys::Promise;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::future_to_promise;

use crate::account::types::OptionMethodSecret;
use crate::account::types::WasmMethodSecret;
use crate::account::wasm_account::account::AccountRc;
use crate::account::wasm_account::WasmAccount;
use crate::common::PromiseVoid;
use crate::did::OptionMethodScope;
use crate::did::OptionMethodType;
use crate::error::Result;
use crate::error::WasmResult;

#[wasm_bindgen(js_class = Account)]
impl WasmAccount {
  /// Adds a new verification method to the DID document.
  #[wasm_bindgen(js_name = createMethod)]
  pub fn create_method(&mut self, options: &CreateMethodOptions) -> Result<PromiseVoid> {
    let method_type: Option<MethodType> = options.methodType().into_serde().wasm_result()?;

    let fragment: String = options
      .fragment()
      .ok_or(MissingRequiredField("fragment"))
      .wasm_result()?;

    let method_scope: Option<MethodScope> = options.methodScope().into_serde().wasm_result()?;
    let method_secret: Option<WasmMethodSecret> = options.methodSecret().into_serde().wasm_result()?;

    let account: Rc<RefCell<AccountRc>> = Rc::clone(&self.0);
    let promise: Promise = future_to_promise(async move {
      let mut account: RefMut<AccountRc> = account.borrow_mut();
      let mut updater: IdentityUpdater<'_, Rc<Client>> = account.update_identity();
      let mut create_method: CreateMethodBuilder<'_, Rc<Client>> = updater.create_method().fragment(fragment);

      if let Some(type_) = method_type {
        create_method = create_method.type_(type_);
      };

      if let Some(scope) = method_scope {
        create_method = create_method.scope(scope);
      };

      if let Some(method_secret) = method_secret.map(MethodSecret::try_from).transpose()? {
        create_method = create_method.method_secret(method_secret);
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
  pub fn methodScope(this: &CreateMethodOptions) -> OptionMethodScope;

  #[wasm_bindgen(getter, method)]
  pub fn methodType(this: &CreateMethodOptions) -> OptionMethodType;

  #[wasm_bindgen(getter, method)]
  pub fn methodSecret(this: &CreateMethodOptions) -> OptionMethodSecret;
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
