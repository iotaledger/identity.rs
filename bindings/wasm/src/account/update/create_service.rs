// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use js_sys::Promise;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;

use identity::account::UpdateError::MissingRequiredField;
use identity::account::{Account, Update};
use identity::core::{Object, Url};
use identity::did::ServiceEndpoint;
use wasm_bindgen::__rt::WasmRefCell;

use crate::account::wasm_account::WasmAccount;
use crate::error::wasm_error;
use crate::error::Result;
use crate::error::WasmResult;

#[wasm_bindgen(js_class = Account)]
impl WasmAccount {
  /// Adds a new Service to the DID Document.
  #[wasm_bindgen(js_name = createService)]
  pub fn create_service(&mut self, options: &CreateServiceOptions) -> Result<Promise> {
    let account: Rc<WasmRefCell<Account>> = Rc::clone(&self.0);

    let service_type: String = options
      .type_()
      .ok_or_else(|| wasm_error(MissingRequiredField("type")))?;

    let fragment: String = options
      .fragment()
      .ok_or(MissingRequiredField("fragment"))
      .wasm_result()?;
    let endpoint: String = options
      .endpoint()
      .ok_or(MissingRequiredField("endpoint"))
      .wasm_result()?;
    let endpoint: Url = Url::parse(endpoint.as_str()).wasm_result()?;
    let properties: Option<Object> = options.properties().into_serde().wasm_result()?;
    let update = Update::CreateService {
      fragment,
      type_: service_type,
      endpoint: ServiceEndpoint::from(endpoint),
      properties,
    };

    let promise: Promise = future_to_promise(async move {
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
  #[wasm_bindgen(typescript_type = "CreateServiceOptions")]
  pub type CreateServiceOptions;

  #[wasm_bindgen(structural, getter, method)]
  pub fn fragment(this: &CreateServiceOptions) -> Option<String>;

  #[wasm_bindgen(structural, getter, method, js_name= type)]
  pub fn type_(this: &CreateServiceOptions) -> Option<String>;

  #[wasm_bindgen(structural, getter, method)]
  pub fn endpoint(this: &CreateServiceOptions) -> Option<String>;

  #[wasm_bindgen(structural, getter, method)]
  pub fn properties(this: &CreateServiceOptions) -> JsValue;
}

#[wasm_bindgen(typescript_custom_section)]
const TS_CREATE_SERVICE_OPTIONS: &'static str = r#"
export type CreateServiceOptions = {
  fragment: string,
  type: string,
  endpoint: string,
  properties?: any,
};
"#;
