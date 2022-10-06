// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::rc::Rc;

use identity_storage::IdentitySuite;
use identity_storage::SignatureHandler;
use js_sys::Function;
use js_sys::Map;
use js_sys::Promise;
use js_sys::Uint8Array;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;

use crate::error::JsValueResult;
use crate::error::Result;
use crate::key_storage::WasmKeyStorage;

#[wasm_bindgen(js_name = IdentitySuite)]
pub struct WasmIdentitySuite(pub(crate) Rc<IdentitySuite<WasmKeyStorage>>);

#[wasm_bindgen(js_class = IdentitySuite)]
impl WasmIdentitySuite {
  #[wasm_bindgen(constructor)]
  #[allow(non_snake_case)]
  pub fn new(storage: WasmKeyStorage, handlers: Option<MapSignatureHandler>) -> Result<WasmIdentitySuite> {
    let mut id_suite = IdentitySuite::new(storage);

    if let Some(handlers) = handlers {
      let map: &Map = handlers.dyn_ref::<js_sys::Map>().expect("TODO");

      for key in map.keys() {
        if let Ok(js_method) = key {
          let js_handler: JsValue = map.get(&js_method);
          let signature_identifier: String = js_method.as_string().expect("TODO");
          let handler: Function = js_handler
            .dyn_into::<Function>()
            .map_err(|_| "could not construct TODO: the handler map contains a value which is not a function")?;

          id_suite.register_raw(signature_identifier, Box::new(WasmSignatureHandler(handler)));
        } else {
          todo!("error")
        }
      }
    }

    Ok(WasmIdentitySuite(Rc::new(id_suite)))
  }
}

impl Clone for WasmIdentitySuite {
  fn clone(&self) -> Self {
    Self(Rc::clone(&self.0))
  }
}

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(typescript_type = "MapSignatureHandler")]
  pub type MapSignatureHandler;

  #[wasm_bindgen(typescript_type = "IdentitySuiteConfig")]
  pub type WasmSuiteHandlers;

  #[wasm_bindgen(method, getter)]
  pub(crate) fn handlers(this: &WasmSuiteHandlers) -> Option<MapSignatureHandler>;
}

// Workaround because JSDocs does not support arrows (=>) while TS does not support the "function" word in type
// definitions (which would be accepted by JSDocs).
#[wasm_bindgen(typescript_custom_section)]
const HANDLERS: &'static str =
  "export type MapSignatureHandler = Map<string, (data: Uint8Array, keyStorage: KeyStorage) => Promise<Uint8Array>>;";

#[wasm_bindgen(typescript_custom_section)]
const TS_RESOLVER_CONFIG: &'static str = r#"
export type WasmSuiteHandlers = {
    handlers?: Map<string, (data: Uint8Array, keyStorage: KeyStorage) => Promise<Uint8Array>>;
};
"#;

pub struct WasmSignatureHandler(Function);

#[async_trait::async_trait(?Send)]
impl SignatureHandler<WasmKeyStorage> for WasmSignatureHandler {
  fn typ(&self) -> identity_storage::NewMethodType {
    unimplemented!("remove this method from the trait")
  }

  async fn sign(&self, data: Vec<u8>, key_storage: &WasmKeyStorage) -> Vec<u8> {
    let function_clone = self.0.clone();

    let js_data: JsValue = Uint8Array::from(data.as_slice()).into();
    let promise: Promise = Promise::resolve(
      &function_clone
        .call2(&JsValue::null(), &js_data, key_storage)
        .expect("TODO"),
    );

    let awaited_output: JsValue = JsValueResult::from(JsFuture::from(promise).await)
      .stringify_error()
      .expect("TODO");

    crate::util::uint8array_to_bytes(awaited_output).expect("TODO")
  }
}
