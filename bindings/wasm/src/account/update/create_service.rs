// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::account::account::WasmAccount;
use crate::error::{wasm_error, Result, WasmResult};
use identity::account::Update;
use identity::account::UpdateError::MissingRequiredField;
use identity::core::Url;
use identity::did::ServiceEndpoint;
use js_sys::Promise;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;

#[wasm_bindgen(js_class = Account)]
impl WasmAccount {
  /// Adds a new Service to the DID Document.
  #[wasm_bindgen(js_name = createService)]
  pub fn create_service(&mut self, options: &CreateServiceOptions) -> Result<Promise> {
    let account = self.0.clone();

    let service_type: String = match options.serviceType() {
      Some(value) => value,
      None => return Err(wasm_error(MissingRequiredField("serviceType"))),
    };

    let fragment = match options.fragment() {
      Some(value) => value.clone(),
      None => return Err(wasm_error(MissingRequiredField("fragment"))),
    };

    let endpoint = match options.endpoint() {
      Some(v) => v,
      None => return Err(wasm_error(MissingRequiredField("endpoint"))),
    };

    let endpoint = Url::parse(endpoint.as_str()).wasm_result()?;

    let update = Update::CreateService {
      fragment,
      type_: service_type,
      endpoint: ServiceEndpoint::from(endpoint),
      properties: None, //ToDo
    };

    let promise: Promise = future_to_promise(async move {
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
  #[wasm_bindgen(typescript_type = "CreateServiceOptions")]
  pub type CreateServiceOptions;

  #[wasm_bindgen(structural, getter, method)]
  pub fn fragment(this: &CreateServiceOptions) -> Option<String>;

  #[wasm_bindgen(structural, getter, method)]
  pub fn serviceType(this: &CreateServiceOptions) -> Option<String>;

  #[wasm_bindgen(structural, getter, method)]
  pub fn endpoint(this: &CreateServiceOptions) -> Option<String>;
}

//ToDo add `properties`
#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"
export type CreateServiceOptions = {
  fragment: string,
  serviceType: string,
  endpoint: string,
};
"#;
