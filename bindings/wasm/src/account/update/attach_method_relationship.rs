// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use js_sys::Promise;

use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;

use identity::account::Account;
use identity::account::Update;
use identity::account::UpdateError::MissingRequiredField;
use identity::core::OneOrMany;

use identity::did::MethodRelationship;
use wasm_bindgen::__rt::WasmRefCell;

use crate::account::wasm_account::WasmAccount;
use crate::account::wasm_method_relationship::WasmMethodRelationship;
use crate::error::wasm_error;
use crate::error::Result;
use crate::error::WasmResult;

#[wasm_bindgen(js_class = Account)]
impl WasmAccount {
  /// Attach one or more verification relationships to a method.
  ///
  /// Note: the method must exist and be in the set of verification methods;
  /// it cannot be an embedded method.
  #[wasm_bindgen(js_name = attachMethodRelationships)]
  pub fn attach_relationships(&mut self, options: &AttachMethodRelationshipOptions) -> Result<Promise> {
    let relationships: Vec<MethodRelationship> = options
      .relationships()
      .into_serde::<OneOrMany<WasmMethodRelationship>>()
      .map(OneOrMany::into_vec)
      .wasm_result()?
      .into_iter()
      .map(Into::into)
      .collect();

    if relationships.is_empty() {
      return Err(wasm_error(MissingRequiredField("relationships is missing or empty")));
    }
    let account: Rc<WasmRefCell<Account>> = Rc::clone(&self.0);
    let fragment: String = options
      .fragment()
      .ok_or(MissingRequiredField("fragment"))
      .wasm_result()?;

    let promise: Promise = future_to_promise(async move {
      let update = Update::AttachMethodRelationship {
        fragment,
        relationships,
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
  #[wasm_bindgen(typescript_type = "AttachMethodRelationshipOptions")]
  pub type AttachMethodRelationshipOptions;

  #[wasm_bindgen(structural, getter, method)]
  pub fn fragment(this: &AttachMethodRelationshipOptions) -> Option<String>;

  #[wasm_bindgen(structural, getter, method)]
  pub fn relationships(this: &AttachMethodRelationshipOptions) -> JsValue;
}

#[wasm_bindgen(typescript_custom_section)]
const TS_ATTACH_METHOD_RELATIONSHIP_OPTIONS: &'static str = r#"
export type AttachMethodRelationshipOptions = {
    /**
     * Fragment of Verification Method.
     */
    fragment: string,

    /**
     * Set one or more method relationships.
     */
    relationships: MethodRelationship | MethodRelationship[]
};
"#;
