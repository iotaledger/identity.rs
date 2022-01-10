// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::account::account::WasmAccount;
use crate::did::{WasmDID, WasmMethodScope, WasmMethodType};
use crate::error::{wasm_error, Result, WasmResult};
use identity::account::Error::UpdateError;
use identity::account::UpdateError::MissingRequiredField;
use identity::account::{Account, IdentityUpdater, MethodSecret, Update};
use identity::core::OneOrMany;
use identity::core::OneOrMany::{Many, One};
use identity::did::{MethodScope, MethodType};
use identity::iota::TangleRef;
use js_sys::Promise;
use std::borrow::Borrow;
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
use std::sync::Mutex;
use wasm_bindgen::__rt::WasmRefCell;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;

#[wasm_bindgen(js_class = Account)]
impl WasmAccount {
  #[wasm_bindgen(js_name = createMethod)]
  pub fn create_method(&mut self, input: &CreateMethodInput) -> Result<Promise> {
    let account = self.0.clone();

    let method_type: MethodType = match input.methodType() {
      Some(value) => value.0.clone(),
      None => MethodType::Ed25519VerificationKey2018,
    };

    let fragment = match input.fragment() {
      Some(value) => value.clone(),
      None => return Err(wasm_error(MissingRequiredField("fragment"))),
    };

    let method_scope: MethodScope = match input.methodScope() {
      Some(value) => value.0.clone(),
      None => MethodScope::default(),
    };

    let promise: Promise = future_to_promise(async move {
      let update = Update::CreateMethod {
        type_: method_type,
        fragment,
        method_secret: None, //ToDo
        scope: method_scope,
      };

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
  #[wasm_bindgen(typescript_type = "CreateMethodInput")]
  pub type CreateMethodInput;

  #[wasm_bindgen(structural, getter, method)]
  pub fn fragment(this: &CreateMethodInput) -> Option<String>;

  #[wasm_bindgen(structural, getter, method)]
  pub fn methodScope(this: &CreateMethodInput) -> Option<WasmMethodScope>;

  #[wasm_bindgen(structural, getter, method)]
  pub fn methodType(this: &CreateMethodInput) -> Option<WasmMethodType>;

  //ToDo methodSecret!
}

#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"
export type CreateMethodInput = {
  fragment: string,
  methodScope?: MethodScope,
  methodType?: MethodType,
};
"#;
