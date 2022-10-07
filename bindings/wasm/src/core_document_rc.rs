use std::cell::RefCell;
use std::cell::RefMut;
use std::rc::Rc;

use crate::common::PromiseVoid;
use crate::did::OptionMethodContent;
use crate::did::OptionMethodScope;
use crate::did::WasmCoreDocument;
use crate::did::WasmMethodContent;
use crate::error::Result;
use crate::error::WasmResult;
use crate::key_storage::WasmKeyStorage;
use identity_iota::did::CoreDocument;
use identity_iota::did::MethodScope;

use identity_storage::CoreDocumentExt;
use identity_storage::CreateMethodBuilder;
use identity_storage::IdentityUpdater;
use identity_storage::MethodContent;
use js_sys::Promise;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::future_to_promise;

#[wasm_bindgen(js_name = CoreDocumentRc)]
pub struct CoreDocumentRc(Rc<RefCell<CoreDocument>>);

#[wasm_bindgen(js_class = CoreDocumentRc)]
impl CoreDocumentRc {
  #[wasm_bindgen(constructor)]
  pub fn new(core_document: WasmCoreDocument) -> CoreDocumentRc {
    CoreDocumentRc(Rc::new(RefCell::new(core_document.0)))
  }

  // TODO Add method to get back a WasmCoreDocument (if possible?).

  #[wasm_bindgen(js_name = createMethod)]
  pub fn create_method(&mut self, options: &CreateMethodOptions) -> Result<PromiseVoid> {
    let fragment: String = options.fragment().expect("TODO");
    let scope: Option<MethodScope> = options.scope().into_serde().expect("TODO");
    let content: MethodContent = options
      .content()
      .into_serde::<Option<WasmMethodContent>>()
      .wasm_result()?
      .map(MethodContent::from)
      .expect("TODO");

    let key_storage: WasmKeyStorage = options.key_storage().expect("TODO");

    let account: Rc<RefCell<CoreDocument>> = Rc::clone(&self.0);
    let promise: Promise = future_to_promise(async move {
      let mut account: RefMut<CoreDocument> = account.borrow_mut();
      let mut updater: IdentityUpdater<'_> = account.update_identity();

      let mut create_method: CreateMethodBuilder<'_, WasmKeyStorage> = updater
        .create_method()
        .key_storage(&key_storage)
        .content(content)
        .fragment(&fragment);
      // TODO: Not implemented currently.
      // if let Some(scope) = scope {
      //   create_method = create_method.scope(scope);
      // };

      let res = create_method.apply().await;

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

  #[wasm_bindgen(getter, method)]
  pub fn content(this: &CreateMethodOptions) -> OptionMethodContent;

  #[wasm_bindgen(getter, method)]
  pub fn key_storage(this: &CreateMethodOptions) -> Option<WasmKeyStorage>;
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
    content: MethodContent
  };
"#;
