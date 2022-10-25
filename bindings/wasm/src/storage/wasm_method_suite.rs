// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::rc::Rc;

use crate::did::WasmMethodContent;
use crate::did::WasmMethodData;
use crate::error::Result;
use crate::error::WasmResult;
use identity_iota::did::MethodData;
use identity_iota::did::MethodType;
use identity_storage::KeyAlias;
use identity_storage::MethodContent;
use identity_storage::MethodHandler;
use identity_storage::MethodSuite;
use identity_storage::Storage;
use js_sys::Map;
use js_sys::Promise;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;

use crate::storage::WasmKeyStorage;

use super::WasmBlobStorage;
use super::WasmKeyAlias;

#[wasm_bindgen(js_name = MethodSuite)]
pub struct WasmMethodSuite(pub(crate) Rc<MethodSuite<WasmKeyStorage, WasmBlobStorage>>);

#[wasm_bindgen(js_class = MethodSuite)]
impl WasmMethodSuite {
  #[wasm_bindgen(constructor)]
  #[allow(non_snake_case)]
  pub fn new(
    keyStorage: WasmKeyStorage,
    blobStorage: WasmBlobStorage,
    handlers: Option<MapMethodHandler>,
  ) -> Result<WasmMethodSuite> {
    let mut method_suite = MethodSuite::new(Storage::new(keyStorage, blobStorage));

    if let Some(handlers) = handlers {
      let map: &Map = handlers.dyn_ref::<Map>().expect("TODO");

      for key in map.keys() {
        if let Ok(js_method) = key {
          let js_handler: JsValue = map.get(&js_method);
          let method_type: String = js_method.as_string().expect("TODO");

          // TODO: dyn_into fails, why?
          let handler: WasmMethodHandlerInterface = js_handler.unchecked_into::<WasmMethodHandlerInterface>();
          // .map_err(|_| "could not construct TODO: the handler map contains a value which is not a ...")?;

          method_suite.register_unchecked(
            MethodType::from(method_type),
            Box::new(WasmMethodHandler(handler.into())),
          );
        } else {
          todo!("error")
        }
      }
    }

    Ok(WasmMethodSuite(Rc::new(method_suite)))
  }
}

impl Clone for WasmMethodSuite {
  fn clone(&self) -> Self {
    Self(Rc::clone(&self.0))
  }
}

#[wasm_bindgen(js_name = CreateMethodResult)]
#[derive(Serialize, Deserialize)]
pub struct WasmCreateMethodResult {
  pub(crate) key_alias: KeyAlias,
  pub(crate) method_data: MethodData,
}

#[wasm_bindgen(js_class = CreateMethodResult)]
impl WasmCreateMethodResult {
  #[allow(non_snake_case)]
  #[wasm_bindgen(constructor)]
  pub fn new(keyAlias: &WasmKeyAlias, methodData: &WasmMethodData) -> WasmCreateMethodResult {
    Self {
      key_alias: keyAlias.clone().into(),
      method_data: methodData.clone().into(),
    }
  }

  /// Serializes this to a JSON object.
  #[wasm_bindgen(js_name = toJSON)]
  pub fn to_json(&self) -> crate::error::Result<JsValue> {
    JsValue::from_serde(self).wasm_result()
  }

  /// Deserializes an instance from a JSON object.
  #[wasm_bindgen(js_name = fromJSON)]
  pub fn from_json(json: &JsValue) -> crate::error::Result<WasmCreateMethodResult> {
    json.into_serde().wasm_result()
  }
}

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(typescript_type = "undefined | MethodSuite")]
  pub type OptionMethodSuite;

  #[wasm_bindgen(typescript_type = "MapMethodHandler")]
  pub type MapMethodHandler;

  #[wasm_bindgen(typescript_type = "MethodSuiteHandlers")]
  pub type MethodSuiteHandlers;

  #[wasm_bindgen(method, getter)]
  pub(crate) fn handlers(this: &MethodSuiteHandlers) -> Option<MapMethodHandler>;

  #[wasm_bindgen(typescript_type = "MethodHandler")]
  pub type WasmMethodHandlerInterface;

  #[wasm_bindgen(typescript_type = "Promise<CreateMethodResult>")]
  pub type PromiseCreateMethodResult;

  #[wasm_bindgen(method)]
  fn create(
    this: &WasmMethodHandlerInterface,
    method_content: WasmMethodContent,
    key_storage: WasmKeyStorage,
  ) -> PromiseCreateMethodResult;

  #[wasm_bindgen(method, js_name = "methodType")]
  pub fn method_type(this: &WasmMethodHandlerInterface) -> String;
}

#[wasm_bindgen(typescript_custom_section)]
const TS_SECTION: &'static str = r#"
export type MapMethodHandler = Map<string, MethodHandler>;

export type MethodSuiteHandlers = {
  handlers?: MapMethodHandler;
};

export interface MethodHandler {
  create(methodContent: MethodContent, keyStorage: KeyStorage): Promise<CreateMethodResult>;
  methodType(): string;
}
"#;

pub struct WasmMethodHandler(Rc<WasmMethodHandlerInterface>);

impl From<WasmMethodHandlerInterface> for WasmMethodHandler {
  fn from(interface: WasmMethodHandlerInterface) -> Self {
    Self(Rc::new(interface))
  }
}

#[async_trait::async_trait(?Send)]
impl MethodHandler<WasmKeyStorage> for WasmMethodHandler {
  fn method_type(&self) -> MethodType {
    self.0.method_type().into()
  }

  async fn create(&self, method_content: MethodContent, key_storage: &WasmKeyStorage) -> (KeyAlias, MethodData) {
    let wasm_method_content: WasmMethodContent = method_content.into();
    let storage_clone: WasmKeyStorage = JsValue::clone(key_storage).unchecked_into();

    let result: WasmCreateMethodResult =
      JsFuture::from(Promise::resolve(&self.0.create(wasm_method_content, storage_clone)))
        .await
        .expect("TODO")
        .into_serde()
        .expect("TODO");

    (result.key_alias, result.method_data)
  }
}
