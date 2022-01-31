// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use js_sys::Promise;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;

use identity::account::Update;
use identity::account::UpdateError::MissingRequiredField;
use identity::account::{Account, MethodSecret};
use identity::did::MethodScope;
use identity::did::MethodType;
use wasm_bindgen::__rt::WasmRefCell;

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
    let account: Rc<WasmRefCell<Account>> = Rc::clone(&self.0);

    let method_type: MethodType = options
      .methodType()
      .map(|m| m.0)
      .unwrap_or(MethodType::Ed25519VerificationKey2018);

    let fragment: String = options
      .fragment()
      .ok_or(MissingRequiredField("fragment"))
      .wasm_result()?;

    let method_scope: MethodScope = options.methodScope().map(|ms| ms.0).unwrap_or_default();

    let method_secret: Option<MethodSecret> = options.methodSecret().map(|ms| ms.0);

    let promise: Promise = future_to_promise(async move {
      let update = Update::CreateMethod {
        type_: method_type,
        fragment,
        method_secret,
        scope: method_scope,
      };

      account
        .as_ref()
        .borrow_mut()
        .process_update(update)
        .await
        .wasm_result()
        .map(|_| JsValue::undefined())
    });

    Ok(promise)
  }
}

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(typescript_type = "CreateMethodOptions")]
  pub type CreateMethodOptions;

  #[wasm_bindgen(structural, getter, method)]
  pub fn fragment(this: &CreateMethodOptions) -> Option<String>;

  #[wasm_bindgen(structural, getter, method)]
  pub fn methodScope(this: &CreateMethodOptions) -> Option<WasmMethodScope>;

  #[wasm_bindgen(structural, getter, method)]
  pub fn methodType(this: &CreateMethodOptions) -> Option<WasmMethodType>;

  #[wasm_bindgen(structural, getter, method)]
  pub fn methodSecret(this: &CreateMethodOptions) -> Option<WasmMethodSecret>;
}

#[wasm_bindgen(typescript_custom_section)]
const TS_CREATE_METHOD_OPTIONS: &'static str = r#"
export type CreateMethodOptions = {
  fragment: string,
  methodScope?: MethodScope,
  methodType?: MethodType,
  methodSecret?: MethodSecret
};
"#;
