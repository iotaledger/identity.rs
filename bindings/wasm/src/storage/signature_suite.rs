// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::rc::Rc;

use identity_iota::crypto::ProofValue;
use identity_storage::MethodType1;
use identity_storage::Signable;
use identity_storage::SignatureHandler;
use identity_storage::SignatureSuite;
use identity_storage::StorageResult;
use js_sys::Map;
use js_sys::Promise;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;

use crate::crypto::PromiseProofValue;
use crate::crypto::WasmProofValue;
use crate::error::Result;
use crate::storage::WasmKeyStorage;
use crate::storage::WasmSignable;

#[wasm_bindgen(js_name = SignatureSuite)]
pub struct WasmSignatureSuite(pub(crate) Rc<SignatureSuite<WasmKeyStorage>>);

#[wasm_bindgen(js_class = SignatureSuite)]
impl WasmSignatureSuite {
  #[wasm_bindgen(constructor)]
  #[allow(non_snake_case)]
  pub fn new(storage: WasmKeyStorage, handlers: Option<MapSignatureHandler>) -> Result<WasmSignatureSuite> {
    let mut signature_suite = SignatureSuite::new(storage);

    if let Some(handlers) = handlers {
      let map: &Map = handlers.dyn_ref::<js_sys::Map>().expect("TODO");

      for key in map.keys() {
        if let Ok(js_method) = key {
          let js_handler: JsValue = map.get(&js_method);
          let method_type: String = js_method.as_string().expect("TODO");

          // TODO: dyn_into fails, why?
          let handler: WasmSignatureHandlerInterface = js_handler.unchecked_into::<WasmSignatureHandlerInterface>();
          // .map_err(|_| "could not construct TODO: the handler map contains a value which is not a ...")?;

          signature_suite.register_unchecked(
            MethodType1::from(method_type),
            Box::new(WasmSignatureHandler(handler.into())),
          );
        } else {
          todo!("error")
        }
      }
    }

    Ok(WasmSignatureSuite(Rc::new(signature_suite)))
  }
}

impl Clone for WasmSignatureSuite {
  fn clone(&self) -> Self {
    Self(Rc::clone(&self.0))
  }
}

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(typescript_type = "MapSignatureHandler")]
  pub type MapSignatureHandler;

  #[wasm_bindgen(typescript_type = "SignatureSuiteHandlers")]
  pub type SignatureSuiteHandlers;

  #[wasm_bindgen(method, getter)]
  pub(crate) fn handlers(this: &SignatureSuiteHandlers) -> Option<MapSignatureHandler>;

  #[wasm_bindgen(typescript_type = "SignatureHandler")]
  pub type WasmSignatureHandlerInterface;

  #[wasm_bindgen(method)]
  pub fn sign(
    this: &WasmSignatureHandlerInterface,
    value: WasmSignable,
    keyStorage: WasmKeyStorage,
  ) -> PromiseProofValue;

  #[wasm_bindgen(method, js_name = "signatureName")]
  pub fn signature_name(this: &WasmSignatureHandlerInterface) -> String;
}

#[wasm_bindgen(typescript_custom_section)]
const TS_SECTION: &'static str = r#"
export type MapSignatureHandler = Map<string, SignatureHandler>;

export type SignatureSuiteHandlers = {
  handlers?: MapSignatureHandler;
};

export interface SignatureHandler {
  sign(value: Signable, keyStorage: KeyStorage): Promise<ProofValue>;
  signatureName(): string;
}
"#;

pub struct WasmSignatureHandler(Rc<WasmSignatureHandlerInterface>);

impl From<WasmSignatureHandlerInterface> for WasmSignatureHandler {
  fn from(interface: WasmSignatureHandlerInterface) -> Self {
    Self(Rc::new(interface))
  }
}

#[async_trait::async_trait(?Send)]
impl SignatureHandler<WasmKeyStorage> for WasmSignatureHandler {
  fn signature_name(&self) -> String {
    self.0.signature_name()
  }

  async fn sign(&self, value: Signable, key_storage: &WasmKeyStorage) -> StorageResult<ProofValue> {
    // let handler_clone: Rc<WasmSignatureHandlerInterface> = Rc::clone(&self.0);
    let wasm_signable: WasmSignable = value.into();
    let storage_clone: WasmKeyStorage = JsValue::clone(key_storage).unchecked_into();

    let result: WasmProofValue = JsFuture::from(Promise::resolve(&self.0.sign(wasm_signable, storage_clone)))
      .await
      .expect("TODO")
      .into_serde()
      .expect("TODO");

    Ok(result.into())
  }
}
