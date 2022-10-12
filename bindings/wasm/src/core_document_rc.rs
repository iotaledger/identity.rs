// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::cell::RefCell;
use std::cell::RefMut;
use std::rc::Rc;

use crate::common::PromiseVoid;
use crate::did::OptionMethodContent;
use crate::did::OptionMethodScope;
use crate::did::OptionMethodType1;
use crate::did::WasmCoreDocument;
use crate::did::WasmMethodContent;
use crate::error::Result;
use crate::key_storage::WasmKeyStorage;
// use crate::wasm_method_suite::WasmMethodSuite;
use identity_iota::did::CoreDocument;

use identity_storage::CoreDocumentExt;
use identity_storage::CreateMethodBuilder;
use identity_storage::IdentityUpdater;
use identity_storage::MethodContent;
use identity_storage::MethodType1;
use js_sys::Promise;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::future_to_promise;

#[wasm_bindgen(js_name = CoreDocumentRc)]
pub struct WasmCoreDocumentRc(Rc<RefCell<CoreDocument>>);

#[wasm_bindgen(js_class = CoreDocumentRc)]
impl WasmCoreDocumentRc {
  #[wasm_bindgen(constructor)]
  pub fn new(core_document: &WasmCoreDocument) -> WasmCoreDocumentRc {
    WasmCoreDocumentRc(Rc::new(RefCell::new(core_document.0.clone())))
  }

  #[wasm_bindgen(js_name = intoDocument)]
  pub fn into_document(&self) -> WasmCoreDocument {
    WasmCoreDocument(RefCell::borrow(&self.0).clone())
  }

  #[wasm_bindgen(js_name = createMethod)]
  pub fn create_method(&mut self, options: &CreateMethodOptions) -> Result<PromiseVoid> {
    let fragment: String = options.fragment().expect("TODO");
    // let scope: Option<MethodScope> = options.scope().into_serde().expect("TODO");
    let content: MethodContent = options
      .content()
      .into_serde::<Option<WasmMethodContent>>()
      .expect("TODO")
      .map(MethodContent::from)
      .expect("TODO");

    let method_type: MethodType1 = options
      .type_()
      .into_serde::<Option<MethodType1>>()
      .expect("TODO")
      .expect("TODO");
    // let key_storage: WasmKeyStorage = options.key_storage().expect("TODO");

    let document: Rc<RefCell<CoreDocument>> = Rc::clone(&self.0);
    let promise: Promise = future_to_promise(async move {
      let mut document_ref: RefMut<CoreDocument> = document.borrow_mut();
      let mut updater: IdentityUpdater<'_> = document_ref.update_identity();

      let create_method: CreateMethodBuilder<'_, WasmKeyStorage> = updater
        .create_method()
        // .key_storage(&key_storage)
        .content(content)
        .type_(method_type.into())
        .fragment(&fragment);

      // TODO: Not implemented currently.
      // if let Some(scope) = scope {
      //   create_method = create_method.scope(scope);
      // };

      create_method.apply().await;

      Ok(JsValue::undefined())
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

  #[wasm_bindgen(getter, method, js_name = type)]
  pub fn type_(this: &CreateMethodOptions) -> OptionMethodType1;

  #[wasm_bindgen(getter, method)]
  pub fn content(this: &CreateMethodOptions) -> OptionMethodContent;

  // #[wasm_bindgen(getter, method)]
  // pub fn key_storage(this: &CreateMethodOptions) -> Option<WasmKeyStorage>;

  // #[wasm_bindgen(getter, method)]
  // pub fn method_suite(this: &CreateMethodOptions) -> Option<WasmMethodSuite>;
}

// TODO: Match the above.
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
    content: MethodContent,

    type: MethodType1,

    /** A KeyStorage implementation. */
    key_storage: KeyStorage
  };
"#;
